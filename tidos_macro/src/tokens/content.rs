use crate::tokens::{ComponentContent, NativeContent};
use crate::tokens::{ControlTag, IsStatic};
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

impl IsStatic for Content {
	fn is_static(&self) -> bool {
		match self {
			Content::NativeElement(element) => element.is_static(),
			Content::CustomComponent(component) => component.is_static(),
			Content::ControlTag(_) => false,
			Content::Text(content) => content.is_static(),
			Content::RawHTMLExpression(content) => content.is_static(),
			Content::SlotRender(_) => false,
		}
	}
}

#[derive(Debug)]
pub enum TextContent {
	Literal(Literal),
	Formatted(Literal, Vec<Vec<TokenTree>>),
	Expression(Vec<TokenTree>),
}

impl IsStatic for TextContent {
	fn is_static(&self) -> bool {
		match self {
			TextContent::Literal(_) => true,
			TextContent::Formatted(_, _) => false,
			TextContent::Expression(_) => false,
		}
	}
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
