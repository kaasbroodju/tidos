use crate::tokens::Content;
use crate::tokens::{Attribute, IsStatic};
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

impl IsStatic for NativeContent {
	fn is_static(&self) -> bool {
		let has_only_static_attributes = self.attributes.iter().all(Attribute::is_static);

		let has_only_static_children = self.children.iter().all(Content::is_static);

		has_only_static_attributes && has_only_static_children
	}
}

impl IsStatic for ComponentContent {
	fn is_static(&self) -> bool {
		false
	}
}
