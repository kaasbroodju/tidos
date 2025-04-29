use crate::tokens::Attribute;
use crate::tokens::Content;
use proc_macro2::{Group, Literal};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::Token;

#[derive(Debug)]
pub struct HTMLTag {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

impl HTMLTag {
	pub fn is_static(&self) -> bool {
		let is_component = self.tag.chars().next().unwrap().is_ascii_uppercase();
		if is_component {
			return false;
		}

		let has_only_static_attributes = self
			.attributes
			.iter()
			.all(|attribute| attribute.is_static());

		let has_only_static_children = self.children.iter().all(|child| child.is_static());

		has_only_static_attributes && has_only_static_children
	}
}
