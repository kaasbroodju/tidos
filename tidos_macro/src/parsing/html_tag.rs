use crate::parsing::utils::matches_tag;
use crate::tokens::Content;
use crate::tokens::{Attribute, HTMLTag};
use proc_macro2::{Group, Literal};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for HTMLTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		input.parse::<Token![<]>()?;
		let left_tag = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let (ident, next) = rest
				.ident()
				.expect("Expected an html like <p> or <custom-element>");
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let next = rest.punct().unwrap().1;
				rest = next;

				output.push('-');
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like <p> or <custom-element>");

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			return Ok((output, rest));
		})?;

		let mut attributes = Vec::new();
		while !((input.peek(Token![/]) && input.peek2(Token![>])) || input.peek(Token![>])) {
			let is_toggle_attribute = input.parse::<Token![:]>().is_ok();

			// todo also allow 'in names'
			// let name = input.parse::<Ident>()?.to_token_stream();
			let name = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = String::new();
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like <p> or <custom-element>");
				output.push_str(ident.to_string().as_str());

				rest = next;

				// parse custom element's tag name.
				// custom elements should contain '-', however it is not required.
				while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
					let next = rest.punct().unwrap().1;
					rest = next;

					output.push('-');
					let (ident, next) = rest
						.ident()
						.expect("Expected an html like <p> or <custom-element>");

					output.push_str(ident.to_string().as_str());
					rest = next;
				}

				return Ok((output, rest));
			})?;

			if input.parse::<Token![=]>().is_ok() {
				if let Ok(literal) = input.parse::<Literal>() {
					if is_toggle_attribute {
						panic!("Unable to have a toggle attribute from a literal");
					}

					attributes.push(Attribute {
						is_toggle_attribute,
						name,
						value: Some(proc_macro2::TokenTree::Literal(literal)),
					});
				} else if let Ok(group) = input.parse::<Group>() {
					attributes.push(Attribute {
						is_toggle_attribute,
						name,
						value: Some(proc_macro2::TokenTree::Group(group)),
					});
				} else {
					panic!("Expected a literal \"\" or a group {{}}");
				}
			} else {
				attributes.push(Attribute {
					is_toggle_attribute,
					name,
					value: None,
				});
			}
		}

		// self closing tags, like <img />
		if input.peek(Token![/]) && input.peek2(Token![>]) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(HTMLTag {
				tag: left_tag.to_string(),
				attributes,
				children: vec![],
				is_self_closing: true,
			});
		}

		input.parse::<Token![>]>()?;

		// ...
		let mut children: Vec<Content> = Vec::new();
		while !input.is_empty() && !matches_tag(input.cursor(), left_tag.to_string()) {
			let child = input.parse::<Content>()?;
			children.push(child);
		}

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		// right tag
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let (ident, next) = rest
				.ident()
				.expect("Expected an html like </p> or </custom-element>");
			output.push_str(ident.to_string().as_str());

			rest = next;

			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let next = rest.punct().unwrap().1;
				rest = next;

				output.push('-');
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like </p> or </custom-element>");

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			return Ok((output, rest));
		})?;
		input.parse::<Token![>]>()?;

		Ok(HTMLTag {
			tag: left_tag.to_string(),
			attributes,
			children,
			is_self_closing: false,
		})
	}
}
