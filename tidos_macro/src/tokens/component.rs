use crate::tokens::{Content, IsStatic};

#[derive(Debug)]
pub struct Component {
	pub children: Vec<Content>,
}

impl IsStatic for Component {
	fn is_static(&self) -> bool {
		self.children.iter().all(Content::is_static)
	}
}
