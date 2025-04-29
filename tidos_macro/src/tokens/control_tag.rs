use crate::tokens::Content;
use proc_macro2::{Group, Ident, Punct, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::Token;

#[derive(Debug)]
pub enum ControlTag {
	// {#if ... } ... {:else if ... } ... {:else} ... {/if}
	IfChain {
		if_statement: Vec<TokenTree>,
		if_content: Vec<Content>,
		if_else_chain: Vec<(Vec<TokenTree>, Vec<Content>)>,
		else_content: Option<Vec<Content>>,
	},
	// {#for ... in ... } ... {/for}
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
		contents: Vec<Content>,
	},
	// {#match ... } ... {/match}
	Match {
		match_statement: Vec<TokenTree>,
		cases: Vec<(Vec<TokenTree>, Vec<Content>)>,
	},
}
