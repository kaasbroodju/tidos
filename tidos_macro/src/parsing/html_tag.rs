use crate::parsing::attribute::parse_attributes;
use crate::parsing::utils::HTMLutils;
use crate::tokens::{ComponentContent, NativeContent};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

impl Parse for NativeContent {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		let start_first_tag_token = input.parse::<Token![<]>()?;
		let (tag, _) = HTMLutils::extract_name(input)?;
		let attributes = parse_attributes(input, &tag, start_first_tag_token.span())?;

		if attributes.has_default_flag {
			return Err(syn::Error::new(
				start_first_tag_token.span(),
				"native html tags cannot have a default flag, remove the `..`".to_string(),
			));
		}

		// self closing tags, like <img />
		if HTMLutils::is_peeking_at_self_closing_tag(input) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(NativeContent {
				tag,
				attributes: attributes.attributes,
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
		let children = HTMLutils::parse_body(input, start_first_tag_token, &tag)?;

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		HTMLutils::extract_name(input)?;
		input.parse::<Token![>]>()?;

		Ok(NativeContent {
			tag,
			attributes: attributes.attributes,
			children,
			is_self_closing: false,
		})
	}
}

impl Parse for ComponentContent {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		let start_first_tag_token = input.parse::<Token![<]>()?;
		let (tag, tag_span) = HTMLutils::extract_name(input)?;
		let attributes = parse_attributes(input, &tag, start_first_tag_token.span())?;

		// self closing tags, like <img />
		if HTMLutils::is_peeking_at_self_closing_tag(input) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(ComponentContent {
				tag,
				tag_span,
				closing_tag_span: None,
				attributes: attributes.attributes,
				has_default_flag: attributes.has_default_flag,
				children: vec![],
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
		let children = HTMLutils::parse_body(input, start_first_tag_token, &tag)?;

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		let (_, closing_span) = HTMLutils::extract_name(input)?;
		input.parse::<Token![>]>()?;

		Ok(ComponentContent {
			tag,
			tag_span,
			closing_tag_span: Some(closing_span),
			attributes: attributes.attributes,
			has_default_flag: attributes.has_default_flag,
			children,
		})
	}
}
