//! The [`ControlTag`] AST node: a fully parsed control-flow construct,
//! header and body together. Produced by `parsing::control_tag`.

use crate::tokens::Content;
use proc_macro2::{Ident, TokenTree};

/// A parsed control-flow construct with its body already collected. Unlike
/// [`crate::tokens::TypeOfCommandTag`] (just the header), each variant here
/// also holds the nested [`Content`] for its branch(es)/body, ready for
/// `code_generation` to splice into a real `for`/`if`/`match`.
#[derive(Debug)]
pub enum ControlTag {
	/// {#if ... } ... {:else if ... } ... {:else} ... {/if}
	IfChain {
		if_statement: Vec<TokenTree>,
		if_content: Vec<Content>,
		if_else_chain: Vec<(Vec<TokenTree>, Vec<Content>)>,
		else_content: Option<Vec<Content>>,
	},
	/// {#for ... in ... } ... {/for}
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
		contents: Vec<Content>,
	},
	/// {#match ... } ... {/match}
	Match {
		match_statement: Vec<TokenTree>,
		cases: Vec<(Vec<TokenTree>, Vec<Content>)>,
	},
	/// {#slot:name } ... {/slot}
	Slot { name: Ident, contents: Vec<Content> },
}
