use crate::tokens::{IsStatic, TextContent};
use proc_macro2::{Literal, Span, TokenStream};

#[derive(Debug)]
pub struct Attribute {
	pub name: String,
	pub name_span: Span,
	pub attribute_type: AttributeType,
}

#[derive(Debug)]
pub enum AttributeType {
	/// :disabled
	ImplicitToggle,

	/// :disabled={ true }
	ExplicitToggle { value: TokenStream },

	/// disabled
	Constant,

	/// class="wrapper"
	ConstantLiteral { literal: Literal },

	/// value={ person.name }
	Expression { content: TextContent },
}

impl IsStatic for Attribute {
	fn is_static(&self) -> bool {
		match &self.attribute_type {
			AttributeType::ImplicitToggle => false,
			AttributeType::ExplicitToggle { .. } => false,
			AttributeType::Constant => true,
			// todo identifier of scoped css is static
			AttributeType::ConstantLiteral { .. } => true,
			AttributeType::Expression { content } => content.is_static(),
		}
	}
}
