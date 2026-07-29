//! AST types for tags: [`NativeContent`] (native HTML elements),
//! [`ComponentContent`] (custom components), and the intermediate
//! [`Attributes`] produced while parsing either one's attribute list.

use crate::tokens::Attribute;
use crate::tokens::Content;
use proc_macro2::Span;

/// A parsed native HTML element or self-closing tag, e.g. `<p>...</p>` or
/// `<img/>`. Has no default-flag support and no tag-name span, unlike
/// [`ComponentContent`]; see `parsing::html_tag` for why.
#[derive(Debug)]
pub struct NativeContent {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

/// A parsed custom component tag, e.g. `<MyComp prop={expr}>...</MyComp>` or
/// self-closing `<MyComp/>`. `tag_span`/`closing_tag_span` preserve the
/// opening/closing identifier's source location so `code_generation` can
/// re-emit it with correct spans (error messages, IDE navigation).
/// `has_default_flag` records a trailing `..` that fills remaining struct
/// fields via `Default::default()`.
#[derive(Debug)]
pub struct ComponentContent {
	pub tag: String,
	pub tag_span: Span,
	pub closing_tag_span: Option<Span>,
	pub attributes: Vec<Attribute>,
	pub has_default_flag: bool,
	pub children: Vec<Content>,
}

/// The result of parsing an attribute list: the attributes themselves plus
/// whether a trailing `..` default-fill flag was present. Kept separate from
/// [`NativeContent`]/[`ComponentContent`] since the default flag is only
/// valid on components (`parsing::html_tag::NativeContent::parse` rejects it).
#[derive(Debug)]
pub struct Attributes {
	pub attributes: Vec<Attribute>,
	pub has_default_flag: bool,
}
