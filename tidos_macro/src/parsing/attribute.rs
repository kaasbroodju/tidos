//! Parses the attribute list of an opening tag (`<tag attr1 attr2="v" ..>`)
//! into `tokens::Attributes`, and implements `Parse` for a single
//! `tokens::Attribute`.
//!
//! An attribute comes in five forms: a bare name (`disabled`), a string
//! literal value (`class="wrapper"`), a braced expression value
//! (`value={ expr }`), or either of the last two prefixed with `:` to mark it
//! as a boolean "toggle" attribute (`:disabled`, `:disabled={ expr }`) that is
//! rendered conditionally rather than as `name="value"`. A `class` attribute
//! prefixed with `@` (`@class="file.css"`, `@class={"file.css", ...}`) is a
//! sixth, restricted form: the first value must be a string literal path to a
//! CSS file, injected the same way `scoped_css!` is; anything after it is
//! parsed as a normal `class={...}` value and appended after the generated
//! class name.

use crate::parsing::utils::HTMLutils;
use crate::tokens::{Attribute, AttributeType, Attributes, TextContent};
use proc_macro2::{Group, Ident, Literal, Span, TokenStream};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

/// Parses zero or more attributes after a tag name, stopping at `/>`, `>`, or
/// the `..` default-fill marker (recorded as `Attributes::has_default_flag`).
/// Shared by both native element and component tag parsing since the grammar
/// is identical for both.
pub(crate) fn parse_attributes(
	input: ParseStream,
	tag: &str,
	tag_span: proc_macro2::Span,
) -> Result<Attributes, syn::Error> {
	let mut attributes = Vec::new();
	while !(HTMLutils::is_peeking_at_self_closing_tag(input)
		|| input.peek(Token![>])
		|| input.peek(Token![.]))
	{
		if input.is_empty() {
			return Err(syn::Error::new(
				tag_span,
				format!("missing closing `>` for `<{tag}>` tag"),
			));
		}

		let attribute = input.parse::<Attribute>()?;
		attributes.push(attribute);
	}

	let has_default_flag = input
		.parse::<Token![.]>()
		.ok()
		.map(|first_dot| {
			input.parse::<Token![.]>().map_err(|_| {
				syn::Error::new(
					first_dot.span(),
					"Expected second `.` for default attribute".to_string(),
				)
			})
		})
		.transpose()?
		.is_some();

	Ok(Attributes {
		attributes,
		has_default_flag,
	})
}

/// Parses one `name`, `name="literal"`, `name={expr}`, `:name`, `:name={expr}`,
/// or `@class="file.css"` / `@class={"file.css"}` attribute. The leading `:`
/// (if present) is consumed first and only changes how the *value* is
/// interpreted afterwards: a toggle attribute's value must be a braced
/// boolean expression (`{ expr }`), never a bare literal or identifier, since
/// those would be ambiguous with the non-toggle forms. The branches that
/// reject a literal/ident after `:name=` exist purely to turn that ambiguity
/// into an actionable compile error instead of a confusing parse failure
/// further down. A leading `@` is handled separately by
/// [`parse_scoped_css_attribute`], since it has its own restricted grammar
/// (only on `class`, value must be a string literal path).
impl Parse for Attribute {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if let Ok(at_token) = input.parse::<Token![@]>() {
			return parse_scoped_css_attribute(input, at_token.span());
		}

		let toggle_token = input.parse::<Token![:]>();
		let is_toggle_attribute = toggle_token.is_ok();

		let (attribute_name, attribute_name_span) =
			HTMLutils::extract_name(input).map_err(|_| {
				syn::Error::new(
					input.span(),
					"Expected an attribute like `class` or `data-tidos`",
				)
			})?;

		let equal_sign_token = input.parse::<Token![=]>();

		if is_toggle_attribute {
			match equal_sign_token {
				Err(_) => {
					// :disabled, shorthand for :disabled={disabled}
					Ok(Attribute {
						name: attribute_name,
						name_span: attribute_name_span,
						attribute_type: AttributeType::ImplicitToggle,
					})
				}
				Ok(_) => {
					input.parse::<Literal>()
						.map(|literal| {
							syn::Error::new(
								literal.span(),
								format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ boolean expression }}\n{attribute_name}={literal}\n{attribute_name}={{ {literal} }}"),
							)
						})
						.map_or(Ok(()), Err)?;

					input.parse::<Ident>()
						.map(|ident| {
							syn::Error::new(
								ident.span(),
								format!("Unable to have a toggle attribute from a ident, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ {ident} }}\n{attribute_name}={{ {ident} }}"),
							)
						})
						.map_or(Ok(()), Err)?;

					input.parse::<Group>()
						.map_err(|error| {
							syn::Error::new(
								error.span(),
								format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ boolean expression }}\n{attribute_name}={{ expression }}"),
							)
						})
						.map(|group| {
							Attribute {
								name: attribute_name,
								name_span: attribute_name_span,
								attribute_type: AttributeType::ExplicitToggle {
									value: group.stream(),
								},
							}
						})
				}
			}
		} else {
			match equal_sign_token {
				Err(_) => {
					// disabled
					Ok(Attribute {
						name: attribute_name,
						name_span: attribute_name_span,
						attribute_type: AttributeType::Constant,
					})
				}
				Ok(equal_sign_token) => {
					if let Ok(literal) = input.parse::<Literal>() {
						(!matches!(syn::Lit::new(literal.clone()), syn::Lit::Str(_)))
							.then_some(
								syn::Error::new(
									literal.span(),
									format!("Attribute values must be a string, change it into the following:\n{attribute_name}={{ {literal} }}"),
								)
							)
							.map_or(
								Ok(Attribute {
									name: attribute_name,
									name_span: attribute_name_span,
									attribute_type: AttributeType::ConstantLiteral { literal },
								}),
								Err
							)
					} else if input.peek(syn::token::Brace) {
						input.parse::<TextContent>().map(|content| Attribute {
							name: attribute_name,
							name_span: attribute_name_span,
							attribute_type: AttributeType::Expression { content },
						})
					} else {
						let message = if is_toggle_attribute {
							format!("Expected a group {{}}, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ bool }}")
						} else {
							format!("Expected a literal \"\" or a group {{}}, change it into the following:\n{attribute_name}=\"value\"\n{attribute_name}={{ \"value\" }}")
						};

						Err(syn::Error::new(equal_sign_token.span(), message))
					}
				}
			}
		}
	}
}

/// Parses the restricted `@class="file.css"` / `@class={"file.css", ...}`
/// grammar. Only `class` is accepted (the `@` sugar only makes sense for a
/// value that becomes a `scoped_css!`-generated class name). The first value
/// must be a bare string literal, not an arbitrary expression, since it is
/// later spliced into `include_str!`, which requires a literal path at
/// compile time. Everything after an optional `,` is parsed with the exact
/// same grammar `TextContent::parse` uses for a plain `class={...}` value,
/// see [`parse_text_content_body`].
fn parse_scoped_css_attribute(input: ParseStream, at_span: Span) -> syn::Result<Attribute> {
	let (attribute_name, attribute_name_span) = HTMLutils::extract_name(input).map_err(|_| {
		syn::Error::new(
			at_span.span(),
			"Expected `class` after `@`, e.g. `@class=\"file.css\"`",
		)
	})?;

	if attribute_name != "class" {
		return Err(syn::Error::new(
			attribute_name_span,
			format!(
				"`@` is only supported on `class`, e.g. `@class=\"file.css\"`. Got `@{attribute_name}`"
			),
		));
	}

	input.parse::<Token![=]>().map_err(|_| {
		syn::Error::new(
			at_span,
			"Expected `=` after `@class`, e.g. `@class=\"file.css\"` or `@class={\"file.css\"}`",
		)
	})?;

	let (file, rest) = if let Ok(literal) = input.parse::<Literal>() {
		(literal, None)
	} else if input.peek(syn::token::Brace) {
		let content;
		syn::braced!(content in input);

		let file = content.parse::<Literal>().map_err(|_| {
			syn::Error::new(
				content.span(),
				"Expected a string literal path, e.g. `@class={\"file.css\"}`",
			)
		})?;

		let rest = if content.is_empty() {
			None
		} else {
			content.parse::<Token![,]>().map_err(|error| {
				syn::Error::new(
					error.span(),
					"Expected `,` before the class value, e.g. `@class={\"file.css\", \"extra\"}`",
				)
			})?;

			Some(parse_text_content_body(&content)?)
		};

		(file, rest)
	} else {
		return Err(syn::Error::new(
			input.span(),
			"Expected a string literal path: `@class=\"file.css\"` or `@class={\"file.css\"}`",
		));
	};

	if !matches!(syn::Lit::new(file.clone()), syn::Lit::Str(_)) {
		return Err(syn::Error::new(
			file.span(),
			"`@class` requires a string literal path, e.g. `@class=\"file.css\"`",
		));
	}

	Ok(Attribute {
		name: attribute_name,
		name_span: attribute_name_span,
		attribute_type: AttributeType::ScopedCss { file, rest },
	})
}

/// Parses the tail of `@class={ "file.css", ... }` (everything after the
/// mandatory CSS path and its `,`) with the exact same grammar
/// `TextContent::parse` uses for `{ ... }`, minus the outer braces, which the
/// caller already consumed via `syn::braced!`.
fn parse_text_content_body(content: ParseStream) -> syn::Result<TextContent> {
	if content.peek(syn::LitStr) {
		let base_string = content
			.parse::<Literal>()
			.expect("Literal string peeked above");

		if content.is_empty() {
			return Ok(TextContent::Literal(base_string));
		}

		content.parse::<Token![,]>().map_err(|error| {
			syn::Error::new(error.span(), "Expected `,` for formatting arguments")
		})?;

		let params = content.parse::<TokenStream>()?;

		Ok(TextContent::Formatted(base_string, params))
	} else {
		let params = content.parse::<TokenStream>()?;
		Ok(TextContent::Expression(params))
	}
}
