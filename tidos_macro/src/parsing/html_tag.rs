use crate::parsing::utils::{matches_tag, peek_closing_tag_name};
use crate::tokens::{Attribute, AttributeType, Attributes, Content, HTMLTag};
use proc_macro2::{Group, Literal};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Lt;
use syn::Token;

impl Parse for HTMLTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		let start_first_tag_token = input.parse::<Token![<]>()?;
		let (tag, tag_span) = Self::extract_name(input)?;
		let is_component = tag.chars().next().unwrap().is_ascii_uppercase();
		let attributes = Self::parse_attributes(input, &tag, start_first_tag_token.span())?;

		if !is_component && attributes.has_default_flag {
			return Err(syn::Error::new(
				start_first_tag_token.span(),
				"native html tags cannot have a default flag, remove the `..`".to_string(),
			));
		}

		// self closing tags, like <img />
		if Self::is_peeking_at_self_closing_tag(input) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(HTMLTag {
				tag,
				tag_span,
				closing_tag_span: None,
				attributes,
				children: vec![],
				is_self_closing: true,
			});
		}

		let _end_first_tag_token = input.parse::<Token![>]>()?;
		if input.is_empty() {
			return Err(syn::Error::new(
				start_first_tag_token.span(),
				format!("missing matching closing tag `</{tag}>`"),
			));
		}

		// ...
		let children = Self::parse_body(input, start_first_tag_token, &tag)?;

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		let (_, closing_span) = Self::extract_name(input)?;
		input.parse::<Token![>]>()?;

		Ok(HTMLTag {
			tag,
			tag_span,
			closing_tag_span: Some(closing_span),
			attributes,
			children,
			is_self_closing: false,
		})
	}
}

impl HTMLTag {
	fn extract_name(input: ParseStream) -> Result<(String, proc_macro2::Span), syn::Error> {
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let Some((ident, next)) = rest.ident() else {
				return Err(cursor.error("expected an HTML tag name like `p`, `div`, or `custom-element`"));
			};
			let span = ident.span();
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let (_, next) = rest.punct().unwrap();
				rest = next;

				output.push('-');
				let Some((ident, next)) = rest.ident() else {
					return Err(cursor.error("native html or custom elements cannot end with a hyphen:\n\tremove the `-`\n\tadd a name segment after `-`"));
				};

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			Ok(((output, span), rest))
		})
	}

	fn is_peeking_at_self_closing_tag(input: ParseStream) -> bool {
		input.peek(Token![/]) && input.peek2(Token![>])
	}

	fn parse_attributes(
		input: ParseStream,
		tag: &str,
		tag_span: proc_macro2::Span,
	) -> Result<Attributes, syn::Error> {
		let mut attributes = Vec::new();
		let mut has_default_flag = false;
		while !(Self::is_peeking_at_self_closing_tag(input) || input.peek(Token![>])) {
			if input.is_empty() {
				return Err(syn::Error::new(
					tag_span,
					format!("missing closing `>` for `<{tag}>` tag"),
				));
			}

			if input.parse::<Token![.]>().is_ok() && input.parse::<Token![.]>().is_ok() {
				has_default_flag = true;
				continue;
			}

			let is_toggle_attribute = input.parse::<Token![:]>().is_ok();

			let Ok((attribute_name, attribute_name_span)) = Self::extract_name(input) else {
				return Err(syn::Error::new(
					input.span(),
					"Expected an attribute like `class` or `data-tidos`",
				));
			};

			let Ok(equal_sign_token) = input.parse::<Token![=]>() else {
				let attribute = if is_toggle_attribute {
					Attribute {
						name: attribute_name,
						name_span: attribute_name_span,
						attribute_type: AttributeType::ImplicitToggle,
					}
				} else {
					Attribute {
						name: attribute_name,
						name_span: attribute_name_span,
						attribute_type: AttributeType::Constant,
					}
				};

				attributes.push(attribute);
				continue;
			};

			if let Ok(literal) = input.parse::<Literal>() {
				if is_toggle_attribute {
					return Err(syn::Error::new(literal.span(), format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ bool }}\n{attribute_name}={literal}\n{attribute_name}={{ {literal} }}")));
				} else {
					let attribute = Attribute {
						name: attribute_name,
						name_span: attribute_name_span,
						attribute_type: AttributeType::ConstantLiteral { literal },
					};

					attributes.push(attribute);
				}
			} else if input.peek(syn::token::Brace) && is_toggle_attribute {
				let group = input.parse::<Group>()?;
				let attribute = Attribute {
					name: attribute_name,
					name_span: attribute_name_span,
					attribute_type: AttributeType::ExplicitToggle {
						value: group.stream(),
					},
				};

				attributes.push(attribute);
			} else if input.peek(syn::token::Brace) {
				let content = Content::parse_text_content(input)?;
				let attribute = Attribute {
					name: attribute_name,
					name_span: attribute_name_span,
					attribute_type: AttributeType::Expression { content },
				};

				attributes.push(attribute);
			} else {
				let message = if is_toggle_attribute {
					format!("Expected a group {{}}, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ bool }}")
				} else {
					format!("Expected a literal \"\" or a group {{}}, change it into the following:\n{attribute_name}=\"value\"\n{attribute_name}={{ \"value\" }}")
				};
				return Err(syn::Error::new(equal_sign_token.span(), message));
			}
		}
		Ok(Attributes {
			attributes,
			has_default_flag,
		})
	}

	fn parse_body(
		input: ParseStream,
		start_first_tag_token: Lt,
		tag: &String,
	) -> Result<Vec<Content>, syn::Error> {
		let mut children: Vec<Content> = Vec::new();
		while !matches_tag(input.cursor(), tag) {
			if let Some(found_tag) = peek_closing_tag_name(input.cursor()) {
				return Err(syn::Error::new(
					input.span(),
					format!("unexpected closing tag `</{found_tag}>`, expected `</{tag}>`"),
				));
			}
			let child = input.parse::<Content>()?;
			children.push(child);
			if input.is_empty() {
				return Err(syn::Error::new(
					start_first_tag_token.span(),
					format!("missing matching closing tag `</{tag}>`"),
				));
			}
		}
		Ok(children)
	}
}
