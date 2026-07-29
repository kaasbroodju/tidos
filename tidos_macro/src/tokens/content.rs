use crate::tokens::ControlTag;
use crate::tokens::{ComponentContent, NativeContent};
use proc_macro2::{Ident, Literal, TokenTree};

#[derive(Debug)]
pub enum Content {
	// <p>...</p>
	NativeElement(NativeContent),

	// <Custom>...<Custom>
	CustomComponent(ComponentContent),

	// {#if x > 10} ... {/if}
	// {#for x in numbers} ... {/for}
	// {#match x} ... {/match}
	// {#slot:name} ... {/slot}
	ControlTag(ControlTag),

	// text
	Text(TextContent),

	// <p>@html{"<b>potential danger</b>"}</p>
	RawHTMLExpression(TextContent),

	// @slot{self.field_name}
	SlotRender(Vec<TokenTree>),
}

#[derive(Debug)]
pub enum TextContent {
	Literal(Literal),
	Formatted(Literal, Vec<Vec<TokenTree>>),
	Expression(Vec<TokenTree>),
}

pub enum TypeOfCommandTag {
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
	},
	If(Vec<TokenTree>),
	Match(Vec<TokenTree>),
	Slot(Ident),
}
