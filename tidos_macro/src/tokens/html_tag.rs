use crate::tokens::Content;
use crate::tokens::{Attribute, IsStatic};
use proc_macro2::Span;

#[derive(Debug)]
pub struct HTMLTag {
	pub tag: String,
	pub tag_span: Span,
	pub closing_tag_span: Option<Span>,
	pub attributes: Attributes,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

#[derive(Debug)]
pub struct Attributes {
	pub attributes: Vec<Attribute>,
	pub has_default_flag: bool,
}

impl HTMLTag {
	pub fn is_component(&self) -> bool {
		self.tag.chars().next().unwrap().is_ascii_uppercase()
	}
}

impl IsStatic for HTMLTag {
	fn is_static(&self) -> bool {
		if self.is_component() {
			return false;
		}

		let has_only_static_attributes =
			self.attributes.attributes.iter().all(Attribute::is_static)
				&& !self.attributes.has_default_flag;

		let has_only_static_children = self.children.iter().all(Content::is_static);

		has_only_static_attributes && has_only_static_children
	}
}
