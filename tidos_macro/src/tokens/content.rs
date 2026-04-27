use crate::tokens::HTMLTag;
use crate::tokens::{ControlTag, IsStatic};
use proc_macro2::{Ident, Literal, TokenTree};

#[derive(Debug)]
pub enum Content {
	// <p>...</p>
	Tag(HTMLTag),

	// {#if x > 10} ... {/if}
	// {#for x in numbers} ... {/for}
	// {#match x} ... {/match}
	// {#slot:name} ... {/slot}
	ControlTag(ControlTag),

	// // <Custom></Custom>
	// Custom,

	// // <tidos:self></tidos:self>
	// Instruction,

	// text
	Text(TextContent),

	// text
	// Literal(String),
	//
	// // expression <p>{ format!("Hello {}", name) }</p>
	// Expression(Group),

	// <p>@html{"<p>potential danger"}</p>
	RawHTMLExpression(TextContent),
}

impl IsStatic for Content {
	fn is_static(&self) -> bool {
		match self {
			Content::Tag(element) => element.is_static(),
			Content::ControlTag(_) => false,
			Content::Text(content) => content.is_static(),
			// Content::Literal(_) => true,
			// Content::Expression(_) => false,
			Content::RawHTMLExpression(content) => content.is_static(),
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
