use crate::tokens::Attribute;
use crate::tokens::Content;
use proc_macro2::Span;

#[derive(Debug)]
pub struct NativeContent {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

#[derive(Debug)]
pub struct ComponentContent {
	pub tag: String,
	pub tag_span: Span,
	pub closing_tag_span: Option<Span>,
	pub attributes: Vec<Attribute>,
	pub has_default_flag: bool,
	pub children: Vec<Content>,
}

#[derive(Debug)]
pub struct Attributes {
	pub attributes: Vec<Attribute>,
	pub has_default_flag: bool,
}
