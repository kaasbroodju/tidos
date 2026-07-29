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

	let has_default_flag = if input.peek(Token![.]) {
		let first_default_dot = input.parse::<Token![.]>()?;
		if input.parse::<Token![.]>().is_err() {
			return Err(syn::Error::new(
				first_default_dot.span(),
				"Expected second `.` for default attribute".to_string(),
			));
		} else {
			true
		}
	} else {
		false
	};

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

		let Ok((attribute_name, attribute_name_span)) = HTMLutils::extract_name(input) else {
			return Err(syn::Error::new(
				input.span(),
				"Expected an attribute like `class` or `data-tidos`",
			));
		};

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
					if let Ok(literal) = input.parse::<Literal>() {
						return Err(syn::Error::new(
							literal.span(),
							format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ boolean expression }}\n{attribute_name}={literal}\n{attribute_name}={{ {literal} }}"),
						));
					}

					if let Ok(ident) = input.parse::<Ident>() {
						return Err(syn::Error::new(
							ident.span(),
							format!("Unable to have a toggle attribute from a ident, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ {ident} }}\n{attribute_name}={{ {ident} }}"),
						));
					}

					match input.parse::<Group>() {
						Ok(group) => {
							Ok(Attribute {
								name: attribute_name,
								name_span: attribute_name_span,
								attribute_type: AttributeType::ExplicitToggle {
									value: group.stream(),
								},
							})
						}
						Err(error) => {
							Err(syn::Error::new(
								error.span(),
								format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ boolean expression }}\n{attribute_name}={{ expression }}"),
							))
						}
					}
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
						if !matches!(syn::Lit::new(literal.clone()), syn::Lit::Str(_)) {
							return Err(syn::Error::new(
								literal.span(),
								format!("Attribute values must be a string, change it into the following:\n{attribute_name}={{ {literal} }}"),
							));
						}

						let attribute = Attribute {
							name: attribute_name,
							name_span: attribute_name_span,
							attribute_type: AttributeType::ConstantLiteral { literal },
						};

						Ok(attribute)
					} else if input.peek(syn::token::Brace) {
						let content = input.parse::<TextContent>()?;
						let attribute = Attribute {
							name: attribute_name,
							name_span: attribute_name_span,
							attribute_type: AttributeType::Expression { content },
						};

						Ok(attribute)
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
