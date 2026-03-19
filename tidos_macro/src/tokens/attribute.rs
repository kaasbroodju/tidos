use proc_macro2::{Literal, TokenStream};

// #[derive(Debug)]
// pub struct Attribute {
// 	pub is_toggle_attribute: bool,
// 	pub name: String,
// 	pub value: Option<AttributeType>,
// }

#[derive(Debug)]
pub enum Attribute {
	/// :disabled
	ImplicitToggle {
		name: String,
	},

	/// :disabled={ true }
	ExplicitToggle {
		name: String,
		value: TokenStream,
	},

	/// disabled
	Constant {
		name: String,
	},

	/// class="wrapper"
	ConstantLiteral {
		name: String,
		literal: Literal,
	},

	/// value={ person.name }
	ConstantGroup {
		name: String,
		contents: TokenStream,
	}

}

#[derive(Debug)]
pub enum AttributeType {
	Literal(TokenStream),
	Group(TokenStream),
}

impl Attribute {
	pub fn is_static(&self) -> bool {
		match &self {
			Attribute::ImplicitToggle { .. } => { false }
			Attribute::ExplicitToggle { .. } => { false }
			Attribute::Constant { .. } => { true }
			// todo identifier of scoped css is static
			Attribute::ConstantLiteral { .. } => { true }
			Attribute::ConstantGroup { .. } => { false }
		}

		// if self.is_toggle_attribute {
		// 	return false;
		// }
		// match &self.value {
		// 	None => true,
		// 	Some(token) => {
		// 		match token {
		// 			AttributeType::Group(_) => false,
		// 			// todo identifier of scoped css is static
		// 			AttributeType::Literal(_) => true,
		// 		}
		// 	}
		// }
	}
}
