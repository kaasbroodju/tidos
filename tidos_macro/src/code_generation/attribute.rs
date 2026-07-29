use crate::code_generation::component::{flush_flat, FlatArg, ToFlatArgs};
use crate::tokens::{Attribute, AttributeType, TextContent};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

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

				flat_args.push(FlatArg::Literal(name_trimmed.to_string()));
				flat_args.push(FlatArg::Literal("=\"".to_string()));
				flat_args.push(FlatArg::Literal(value.value()));
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
					if #ident { page.push_static(concat!(#name, " ")); }
				});
			}
			AttributeType::ExplicitToggle { value } => {
				flush_flat(flat_args, result);
				result.append_all(quote! {
					if #value { page.push_static(concat!(#name_trimmed, " ")); }
				});
			}
		}
	}
}

impl Attribute {
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
					quote! { #name: format!(#literal #( , #( #contents )* )* ) }
				}
				TextContent::Expression(expr) => {
					quote! { #name: #( #expr )* }
				}
			},
		}
	}
}
