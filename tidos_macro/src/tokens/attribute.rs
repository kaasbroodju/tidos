//! AST types for a tag's attribute list, produced by `parsing::attribute`.

use crate::tokens::TextContent;
use proc_macro2::{Literal, Span, TokenStream};

/// One attribute on a tag (`name`, `name="lit"`, `name={expr}`, `:name`, or
/// `:name={expr}`). `name_span` is kept separately from the plain `String`
/// name so error messages and codegen can still point at the attribute's
/// original source location.
#[derive(Debug)]
pub struct Attribute {
	pub name: String,
	pub name_span: Span,
	pub attribute_type: AttributeType,
}

/// The five forms an attribute's value can take, corresponding 1:1 to what
/// `parsing::attribute::Parse for Attribute` recognizes.
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
