use crate::tokens::Attribute;
use crate::tokens::Content;

#[derive(Debug)]
pub struct HTMLTag {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

impl HTMLTag {
	pub fn is_static(&self) -> bool {
		if self.is_component() {
			return false;
		}

		let has_only_static_attributes = self.attributes.iter().all(Attribute::is_static);

		let has_only_static_children = self.children.iter().all(Content::is_static);

		has_only_static_attributes && has_only_static_children
	}

	fn is_component(&self) -> bool {
		self.tag.chars().next().unwrap().is_ascii_uppercase()
	}
}
