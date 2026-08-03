//! AST types for a tag's attribute list, produced by `parsing::attribute`.

use crate::tokens::TextContent;
use proc_macro2::{Literal, Span, TokenStream};

/// One attribute on a tag (`name`, `name="lit"`, `name={expr}`, `:name`,
/// `:name={expr}`, or the `class`-only `@class="file.css"` /
/// `@class={"file.css", ...}`). `name_span` is kept separately from the plain
/// `String` name so error messages and codegen can still point at the
/// attribute's original source location.
#[derive(Debug)]
pub struct Attribute {
	pub name: String,
	pub name_span: Span,
	pub attribute_type: AttributeType,
}

/// The six forms an attribute's value can take, corresponding 1:1 to what
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

	/// @class="file.css" or @class={"file.css", ...}. `rest`, when present,
	/// is parsed with the exact same grammar as a plain `class={...}` value
	/// (bare literal, format-string + args, or expression) and rendered
	/// after the generated class name, space-separated.
	ScopedCss {
		file: Literal,
		rest: Option<TextContent>,
	},
}
