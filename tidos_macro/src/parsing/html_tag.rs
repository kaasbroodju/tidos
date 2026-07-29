//! Parses opening/closing tag pairs and self-closing tags for both native
//! HTML elements (`<p>...</p>`, `<img/>`) and custom components
//! (`<MyComp>...</MyComp>`). The two share an identical open-tag →
//! attributes → children → close-tag grammar (both call
//! [`HTMLutils::parse_body`] for their children), but produce different AST
//! nodes because native tags forbid the `..` default-fill flag and don't need
//! a tracked span for their tag name, whereas components do (see
//! `ComponentContent::tag_span` / `closing_tag_span`, used later by
//! `code_generation` to splice the identifier back into real Rust code).

use crate::parsing::attribute::parse_attributes;
use crate::parsing::utils::HTMLutils;
use crate::tokens::{ComponentContent, NativeContent};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

/// Parses a native HTML element: `<tag attrs>children</tag>` or the
/// self-closing `<tag attrs/>`. Which of the two applies is decided by
/// peeking for `/>` right after the attributes.
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

/// Parses a custom component tag: `<Tag attrs>children</Tag>` or the
/// self-closing `<Tag attrs/>`. Structurally identical to [`NativeContent`]
/// parsing, except the tag name's span is kept (`tag_span`,
/// `closing_tag_span`) so `code_generation` can re-emit the identifier with
/// correct error-reporting/IDE-navigation spans when it constructs the
/// component struct literal.
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
