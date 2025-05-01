use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Attribute {
	pub is_toggle_attribute: bool,
	pub name: String,
	pub value: Option<AttributeType>,
}

#[derive(Debug)]
pub enum AttributeType {
	Literal(TokenStream),
	Group(TokenStream),
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
					AttributeType::Group(_) => false,
					// todo identifier of scoped css is static
					AttributeType::Literal(_) => true,
				}
			}
		}
	}
}
