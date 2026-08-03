//! Parses the attribute list of an opening tag (`<tag attr1 attr2="v" ..>`)
//! into `tokens::Attributes`, and implements `Parse` for a single
//! `tokens::Attribute`.
//!
//! An attribute comes in five forms: a bare name (`disabled`), a string
//! literal value (`class="wrapper"`), a braced expression value
//! (`value={ expr }`), or either of the last two prefixed with `:` to mark it
//! as a boolean "toggle" attribute (`:disabled`, `:disabled={ expr }`) that is
//! rendered conditionally rather than as `name="value"`.

use crate::parsing::utils::HTMLutils;
use crate::tokens::{Attribute, AttributeType, Attributes, TextContent};
use proc_macro2::{Group, Ident, Literal};
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

/// Parses one `name`, `name="literal"`, `name={expr}`, `:name`, or
/// `:name={expr}` attribute. The leading `:` (if present) is consumed first
/// and only changes how the *value* is interpreted afterwards: a toggle
/// attribute's value must be a braced boolean expression (`{ expr }`), never
/// a bare literal or identifier, since those would be ambiguous with the
/// non-toggle forms. The branches that reject a literal/ident after `:name=`
/// exist purely to turn that ambiguity into an actionable compile error
/// instead of a confusing parse failure further down.
impl Parse for Attribute {
	fn parse(input: ParseStream) -> syn::Result<Self> {
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
