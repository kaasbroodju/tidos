use proc_macro2::TokenTree;

#[derive(Debug)]
pub struct Attribute {
	pub is_toggle_attribute: bool,
	pub name: String,
	pub value: Option<TokenTree>,
}

impl Attribute {
	pub fn is_static(&self) -> bool {
		if self.is_toggle_attribute {
			return false;
		}
		match &self.value {
			None => true,
			Some(token) => {
				match token {
					TokenTree::Group(_) => false,
					// todo identifier of scoped css is static
					TokenTree::Literal(_) => true,
					_ => {
						panic!("Tidos macro error: expected group or ident")
					}
				}
			}
		}
	}
}
