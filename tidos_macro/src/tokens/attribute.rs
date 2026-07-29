use crate::tokens::TextContent;
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
