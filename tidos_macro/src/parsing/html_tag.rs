//! Parses opening/closing tag pairs and self-closing tags for both native
//! HTML elements (`<p>...</p>`, `<img/>`) and custom components
//! (`<MyComp>...</MyComp>`). The two share an identical open-tag →
//! attributes → children → close-tag grammar (both call
//! [`HTMLutils::parse_body`] for their children), but produce different AST
//! nodes because native tags forbid the `..` default-fill flag and don't need
//! a tracked span for their tag name, whereas components do (see
//! `ComponentContent::tag_span` / `closing_tag_span`, used later by
//! `code_generation` to splice the identifier back into real Rust code).
//! Conversely, components forbid the `@class` scoped-CSS attribute (there's
//! no HTML string for it to merge a class into), rejected right here rather
//! than in `code_generation`, so the error surfaces as one clean
//! `syn::Error` instead of a malformed struct-literal field.

use crate::parsing::attribute::parse_attributes;
use crate::parsing::utils::HTMLutils;
use crate::tokens::{AttributeType, ComponentContent, NativeContent};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

/// Parses a native HTML element: `<tag attrs>children</tag>` or the
/// self-closing `<tag attrs/>`. Which of the two applies is decided by
/// peeking for `/>` right after the attributes.
impl Parse for NativeContent {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		let start_first_tag_token = input
			.parse::<Token![<]>()
			.expect("Expected caller to peek `<` first");
		let (tag, _) = HTMLutils::extract_name(input)?;
		let attributes = parse_attributes(input, &tag, start_first_tag_token.span())?;

		attributes
			.has_default_flag
			.then_some(syn::Error::new(
				start_first_tag_token.span(),
				"native html tags cannot have a default flag, remove the `..`",
			))
			.map_or(Ok(()), Err)?;

		let slash = input.parse::<Token![/]>();
		let closing_bracket = input.parse::<Token![>]>();

		match (slash, closing_bracket) {
			(Ok(_), Ok(_)) => Ok(NativeContent {
				tag,
				attributes: attributes.attributes,
				children: vec![],
				is_self_closing: true,
			}),
			(Err(_), Ok(_end_first_tag_token)) => {
				// ...
				let children = HTMLutils::parse_body(input, start_first_tag_token, &tag)?;

				// </p>
				input
					.parse::<Token![<]>()
					.expect("Peeked and matched in HTMLutils::parse_body");
				input
					.parse::<Token![/]>()
					.expect("Peeked and matched in HTMLutils::parse_body");
				HTMLutils::extract_name(input)
					.expect("Peeked and matched in HTMLutils::parse_body");
				input
					.parse::<Token![>]>()
					.expect("Peeked and matched in HTMLutils::parse_body");

				Ok(NativeContent {
					tag,
					attributes: attributes.attributes,
					children,
					is_self_closing: false,
				})
			}
			(Ok(slash), Err(_)) => Err(syn::Error::new(
				start_first_tag_token.span().resolved_at(slash.span()),
				format!("Expected <{tag} /> to self close"),
			)),
			(Err(err), Err(_)) => Err(syn::Error::new(
				start_first_tag_token.span().resolved_at(err.span()),
				format!("<{tag} is not closed add `>` or `/>` at the end of the tag"),
			)),
		}
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
		let start_first_tag_token = input
			.parse::<Token![<]>()
			.expect("Expected caller to peek `<` first");
		let (tag, tag_span) = HTMLutils::extract_name(input)?;
		let attributes = parse_attributes(input, &tag, start_first_tag_token.span())?;

		if let Some(attribute) = attributes
			.attributes
			.iter()
			.find(|attribute| matches!(attribute.attribute_type, AttributeType::ScopedCss { .. }))
		{
			return Err(syn::Error::new(
				attribute.name_span,
				"`@class` is only supported on native HTML tags, not components",
			));
		}

		let slash = input.parse::<Token![/]>();
		let closing_bracket = input.parse::<Token![>]>();

		match (slash, closing_bracket) {
			(Ok(_), Ok(_)) => Ok(ComponentContent {
				tag,
				tag_span,
				closing_tag_span: None,
				attributes: attributes.attributes,
				has_default_flag: attributes.has_default_flag,
				children: vec![],
			}),
			(Err(_), Ok(_end_first_tag_token)) => {
				// ...
				let children = HTMLutils::parse_body(input, start_first_tag_token, &tag)?;

				// </p>
				input
					.parse::<Token![<]>()
					.expect("Peeked and matched in HTMLutils::parse_body");
				input
					.parse::<Token![/]>()
					.expect("Peeked and matched in HTMLutils::parse_body");
				let (_, closing_span) = HTMLutils::extract_name(input)
					.expect("Peeked and matched in HTMLutils::parse_body");
				let closing_bracket = input
					.parse::<Token![>]>()
					.expect("Peeked and matched in HTMLutils::parse_body");

				if children.is_empty() {
					return Err(syn::Error::new(
						start_first_tag_token
							.span()
							.resolved_at(closing_bracket.span()),
						format!("<{tag}> has no children it should be <{tag} />"),
					));
				}

				Ok(ComponentContent {
					tag,
					tag_span,
					closing_tag_span: Some(closing_span),
					attributes: attributes.attributes,
					has_default_flag: attributes.has_default_flag,
					children,
				})
			}
			(Ok(slash), Err(_)) => Err(syn::Error::new(
				start_first_tag_token.span().resolved_at(slash.span()),
				format!("Expected <{tag} /> to self close"),
			)),
			(Err(err), Err(_)) => Err(syn::Error::new(
				start_first_tag_token.span().resolved_at(err.span()),
				format!("<{tag} is not closed add `>` or `/>` at the end of the component"),
			)),
		}
	}
}
