use crate::parsing::attribute::parse_attributes;
use crate::parsing::utils::{matches_tag, peek_closing_tag_name};
use crate::tokens::{Content, HTMLTag};
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
		let attributes = parse_attributes(input, &tag, start_first_tag_token.span())?;

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
	pub(crate) fn extract_name(
		input: ParseStream,
	) -> Result<(String, proc_macro2::Span), syn::Error> {
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

	pub(crate) fn is_peeking_at_self_closing_tag(input: ParseStream) -> bool {
		input.peek(Token![/]) && input.peek2(Token![>])
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
