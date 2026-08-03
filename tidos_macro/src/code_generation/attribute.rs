//! Generates code for a single [`Attribute`], in two different contexts:
//! `ToFlatArgs` renders it as part of a native HTML tag's opening tag string
//! (`name="value"`), while `to_tokens_custom_element` renders it as a
//! component struct field initializer (`name: value`). Components don't go
//! through string rendering for their props at all, since they're
//! constructed as real Rust struct literals.

use crate::code_generation::component::{flush_flat, FlatArg, ToFlatArgs};
use crate::tokens::{Attribute, AttributeType, TextContent};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use uuid::Uuid;

/// Contribute an attribute's value(s) to `flat_args`.
/// Toggle attributes (ImplicitToggle / ExplicitToggle) are always boundaries:
/// they flush `flat_args` and emit a direct `if … { page.push_static(…); }`.
impl ToFlatArgs for Attribute {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, result: &mut TokenStream) {
		let name = self.name.as_str();
		let name_trimmed = name.trim_start_matches("r#");

		match &self.attribute_type {
			AttributeType::Constant => {
				flat_args.push(FlatArg::Literal(name_trimmed.to_string()));
				flat_args.push(FlatArg::Literal(" ".to_string()));
			}
			AttributeType::ConstantLiteral { literal } => {
				// The parser only ever accepts a string literal here (see
				// `parsing::attribute`), so this always matches and can be
				// merged into the surrounding literal run like any other
				// static text.
				let syn::Lit::Str(value) = syn::Lit::new(literal.clone()) else {
					unreachable!("ConstantLiteral is always a string literal")
				};

				let lit = TextContent::sanitize_lit_str(value.clone());

				flat_args.push(FlatArg::Literal(name_trimmed.to_string()));
				flat_args.push(FlatArg::Literal("=\"".to_string()));
				flat_args.push(FlatArg::Literal(lit));
				flat_args.push(FlatArg::Literal("\" ".to_string()));
			}
			AttributeType::Expression { content } => {
				flat_args.push(FlatArg::Literal(name_trimmed.to_string()));
				flat_args.push(FlatArg::Literal("=\"".to_string()));
				content.to_flat_args(flat_args, result);
				flat_args.push(FlatArg::Literal("\" ".to_string()));
			}
			AttributeType::ImplicitToggle => {
				flush_flat(flat_args, result);
				let ident = format_ident!("{}", name);
				result.append_all(quote! {
					if #ident { page.push_static(concat!(#name_trimmed, " ")); }
				});
			}
			AttributeType::ExplicitToggle { value } => {
				flush_flat(flat_args, result);
				result.append_all(quote! {
					if #value { page.push_static(concat!(#name_trimmed, " ")); }
				});
			}
			AttributeType::ScopedCss { file, rest } => {
				// Unlike `scoped_css!` used as an expression, the class name is
				// generated as a plain string literal here (not a block
				// expression), so it can be pushed as `FlatArg::Literal` and
				// merged with surrounding static markup instead of forcing a
				// `FlatArg::Expr` split. The head-registration call is the only
				// part that has to run at runtime, so it's appended to `result`
				// directly rather than going through `flat_args`.
				let class_name = format!("tidos-{}", Uuid::new_v4());
				let class_literal = Literal::string(&class_name);

				result.append_all(quote! {
					page.add_elements_to_head(#class_literal, String::from(concat!("<style>.", #class_literal, " {", include_str!(#file), "}</style>")));
				});

				flat_args.push(FlatArg::Literal(name_trimmed.to_string()));
				flat_args.push(FlatArg::Literal("=\"".to_string()));
				flat_args.push(FlatArg::Literal(class_name));
				// `rest` reuses `TextContent`'s own `ToFlatArgs` impl, so a
				// literal `rest` stays fully static (merges into the same
				// literal run as the class name) while a format-string/expr
				// `rest` becomes exactly the kind of `FlatArg::Expr` split a
				// plain `class={...}` attribute would already produce.
				if let Some(rest) = rest {
					flat_args.push(FlatArg::Literal(" ".to_string()));
					rest.to_flat_args(flat_args, result);
				}
				flat_args.push(FlatArg::Literal("\" ".to_string()));
			}
		}
	}
}

impl Attribute {
	/// Renders this attribute as a component struct field initializer
	/// (`name: value`), for splicing into a `ComponentName { name: value, ... }`
	/// struct literal. Unlike the `ToFlatArgs` impl above, toggle attributes
	/// become plain boolean fields here (`name` shorthand for `true`, or the
	/// expression directly) rather than conditional `if` statements, since
	/// there's no HTML string being built to toggle a fragment of.
	pub fn to_tokens_custom_element(&self) -> TokenStream {
		let name = Ident::new(&self.name, self.name_span).to_token_stream();
		match &self.attribute_type {
			AttributeType::ImplicitToggle => {
				quote! { #name }
			}
			AttributeType::ExplicitToggle { value } => {
				quote! { #name: #value }
			}
			AttributeType::Constant => {
				quote! { #name: true }
			}
			AttributeType::ConstantLiteral { literal } => {
				quote! { #name: #literal }
			}
			AttributeType::Expression { content } => match content {
				TextContent::Literal(literal) => {
					quote! { #name: #literal }
				}
				TextContent::Formatted(literal, contents) => {
					quote! { #name: format!(#literal, #contents ) }
				}
				TextContent::Expression(expr) => {
					quote! { #name:  #expr }
				}
			},
			AttributeType::ScopedCss { .. } => unreachable!(
				"`@class` on a component tag is rejected in `parsing::html_tag::ComponentContent::parse`, \
				 before `to_tokens_custom_element` ever runs; a `to_compile_error()` here would land as a \
				 malformed struct-literal field instead of a clean error"
			),
		}
	}
}
