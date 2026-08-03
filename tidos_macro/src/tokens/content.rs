//! The [`Content`] enum, one node per template item, plus the
//! [`TextContent`] and [`TypeOfCommandTag`] types nested inside it.

use crate::tokens::ControlTag;
use crate::tokens::{ComponentContent, NativeContent};
use proc_macro2::{Ident, Literal, TokenStream};

/// One item inside a template body (a tag's children, a loop/if/match body,
/// or the top-level `Component`). Produced by `parsing::content::Content::parse`,
/// which dispatches to the matching variant based on the next tokens.
#[derive(Debug)]
pub enum Content {
	/// `<p>...</p>` or self-closing `<img/>`.
	NativeElement(NativeContent),

	/// `<Custom>...</Custom>` or self-closing `<Custom/>`.
	CustomComponent(ComponentContent),

	/// `{#if x > 10} ... {/if}`, `{#for x in numbers} ... {/for}`,
	/// `{#match x} ... {/match}`, or `{#slot:name} ... {/slot}`.
	ControlTag(ControlTag),

	/// A sanitized `{...}` text expression.
	Text(TextContent),

	/// An unsanitized `@html{...}` expression, e.g.
	/// `<p>@html{"<b>potential danger</b>"}</p>`.
	RawHTMLExpression(TextContent),

	/// `@slot{expr}`: invokes a slot-render closure passed in as a component
	/// prop, e.g. `@slot{self.field_name}`.
	SlotRender(TokenStream),
}

/// The three forms text inside `{ ... }` (or `@html{ ... }`) can take: a bare
/// string literal, a format-string literal with comma-separated parameter
/// expressions, or an arbitrary Rust expression. Parameter/expression tokens
/// are kept unparsed since they're only ever re-emitted into a `format!(...)`
/// or used directly by `code_generation`, never inspected here.
#[derive(Debug)]
pub enum TextContent {
	/// {"hello world"}
	Literal(Literal),

	/// {"Hello {}", name}
	Formatted(Literal, TokenStream),

	/// { text expression }
	Expression(TokenStream),
}

/// The header of a `{#command ...}` control tag, before its body has been
/// parsed. Produced by `parsing::control_tag::TypeOfCommandTag::parse`, which
/// splits the raw tokens after `#` according to which command they belong to
/// (see that module for why the tokens are kept unparsed).
pub enum TypeOfCommandTag {
	For {
		left_side: TokenStream,
		right_side: TokenStream,
	},
	If(TokenStream),
	Match(TokenStream),
	Slot(Ident),
}
