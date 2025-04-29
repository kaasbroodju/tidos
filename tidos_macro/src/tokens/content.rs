use crate::tokens::ControlTag;
use crate::tokens::HTMLTag;
use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

#[derive(Debug)]
pub enum Content {
	// <p>...</p>
	Tag(HTMLTag),

	// {#if x > 10} ... {/if}
	// {#for x in numbers} ... {/for}
	// {#match x} ... {/match}
	ControlTag(ControlTag),

	// // <Custom></Custom>
	// Custom,

	// // <tidos:self></tidos:self>
	// Instruction,

	// text
	Literal(String),

	// expression <p>{ format!("Hello {}", name) }</p>
	Expression(Group),

	// <p>@html{"<p>potential danger"}</p>
	RawHTMLExpression(Group),
}

impl Content {
	pub fn is_static(&self) -> bool {
		match self {
			Content::Tag(element) => element.is_static(),
			Content::ControlTag(_) => false,
			Content::Literal(_) => true,
			Content::Expression(_) => false,
			Content::RawHTMLExpression(_) => false,
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
}
