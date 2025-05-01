use crate::parsing::utils::matches_tag;
use crate::tokens::{AttributeType, Content};
use crate::tokens::{Attribute, HTMLTag};
use proc_macro2::{Group, Literal};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

impl Parse for HTMLTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		let start_first_tag_token = input.parse::<Token![<]>()?;
		let tag = Self::extract_name(input)
			.map_err(|_| syn::Error::new(input.span(), "Expected an html like <p> or <custom-element>"))?;

		let mut attributes = Vec::new();
		while !((input.peek(Token![/]) && input.peek2(Token![>])) || input.peek(Token![>])) {
			let is_toggle_attribute = input.parse::<Token![:]>().is_ok();
			
			let attribute_name = Self::extract_name(input)
				.map_err(|_| syn::Error::new(input.span(), "Expected an attribute like `class` or `data-octo`"))?;

			let parsing_equal_sign_result = input.parse::<Token![=]>();
			let attribute = match parsing_equal_sign_result {
				Ok(equal_sign_token) => {
					if let Ok(literal) = input.parse::<Literal>() {
						if is_toggle_attribute {
							return Err(syn::Error::new(literal.span(), format!("Unable to have a toggle attribute from a literal, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ bool }}\n{attribute_name}={literal}\n{attribute_name}={{ {literal} }}")));
						} else {
							Attribute {
								is_toggle_attribute,
								name: attribute_name,
								value: Some(AttributeType::Literal(literal.to_token_stream())),
							}
						}
					} else if let Ok(group) = input.parse::<Group>() {
						Attribute {
							is_toggle_attribute,
							name: attribute_name,
							value: Some(AttributeType::Group(group.stream())),
						}
					} else {
						return if is_toggle_attribute {
							Err(syn::Error::new(equal_sign_token.span(), format!("Expected a group {{}}, change it into the following:\n:{attribute_name}\n:{attribute_name}={{ bool }}")))
						} else {
							Err(syn::Error::new(equal_sign_token.span(), format!("Expected a literal \"\" or a group {{}}, change it into the following:\n{attribute_name}=\"value\"\n{attribute_name}={{ \"value\" }}")))
						}
						
					}
				}
				Err(_) => {
					Attribute {
						is_toggle_attribute,
						name: attribute_name,
						value: None,
					}
				}
			};
			
			attributes.push(attribute)
		}

		// self closing tags, like <img />
		if input.peek(Token![/]) && input.peek2(Token![>]) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(HTMLTag {
				tag,
				attributes,
				children: vec![],
				is_self_closing: true,
			});
		}
		
		let end_first_tag_token = input.parse::<Token![>]>()?;
		if input.is_empty() {
			return Err(syn::Error::new(
				start_first_tag_token.span().join(end_first_tag_token.span()).unwrap(),
				format!("missing matching closing tag `</{tag}>`")
			));
		}

		// ...
		let mut children: Vec<Content> = Vec::new();
		while !matches_tag(input.cursor(), &tag) {
			let child = input.parse::<Content>()?;
			children.push(child);
			if input.is_empty() {
				return Err(syn::Error::new(
					start_first_tag_token.span().join(end_first_tag_token.span()).unwrap(),
					format!("missing matching closing tag `</{tag}>`")
				));
			}
		}

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		Self::extract_name(input)?;
		input.parse::<Token![>]>()?;

		Ok(HTMLTag {
			tag,
			attributes,
			children,
			is_self_closing: false,
		})
	}
}

impl HTMLTag {
	fn extract_name(input: ParseStream) -> Result<String, syn::Error> {
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let (ident, next) = rest
				.ident().unwrap();
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let next = rest.punct().unwrap().1;
				rest = next;

				output.push('-');
				let (ident, next) = rest
					.ident().unwrap();

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			return Ok((output, rest));
		})
	}
}
