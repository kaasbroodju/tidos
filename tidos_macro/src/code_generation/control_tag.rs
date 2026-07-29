//! Turns a parsed [`ControlTag`] back into a real Rust `for`/`if`/`match`.
//! The header tokens were kept unparsed by `parsing::control_tag` for exactly
//! this moment: they're spliced directly into a `quote!` template here,
//! without this crate ever needing its own expression/pattern parser.

use crate::code_generation::component::to_push_stmts;
use crate::tokens::{Content, ControlTag};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

/// Renders the control tag's own construct (`for`/`if`/`match`); `Slot` has no
/// standalone rendering because its contents are only ever pulled out by the
/// parent's [`crate::code_generation::html_tag`] logic and boxed into a prop
/// closure, never emitted at the slot's own position in the tree.
impl ToTokens for ControlTag {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			ControlTag::IfChain {
				if_statement,
				if_content,
				if_else_chain,
				else_content,
			} => {
				ControlTag::to_tokens_if_chain(
					if_statement,
					if_content,
					if_else_chain,
					else_content,
					tokens,
				);
			}
			ControlTag::For {
				left_side,
				right_side,
				contents,
			} => {
				ControlTag::to_tokens_for_loop(left_side, right_side, contents, tokens);
			}
			ControlTag::Match {
				match_statement,
				cases,
			} => {
				ControlTag::to_tokens_match(match_statement, cases, tokens);
			}
			ControlTag::Slot { .. } => unreachable!("Slot should not be rendered"),
		}
	}
}

impl ControlTag {
	/// Emits `for (left_side) in (right_side).into_iter() { body }`. Wrapping
	/// `left_side` in parens lets it be a tuple-destructuring pattern (e.g.
	/// `(k, v)`) as well as a single binding; wrapping `right_side` and
	/// calling `.into_iter()` lets the iterator expression be anything
	/// `IntoIterator`-compatible (a `Vec`, a `&Vec`, a range, ...).
	fn to_tokens_for_loop(
		left_side: &Vec<TokenTree>,
		right_side: &Vec<TokenTree>,
		contents: &[Content],
		tokens: &mut TokenStream,
	) {
		let body = to_push_stmts(contents);

		let output = quote! {
			for ( #( #left_side )* ) in ( #( #right_side )* ).into_iter() {
				#body
			}
		};

		tokens.append_all(output);
	}

	/// Emits `if if_statement { if_body } else if ... { ... } else { else_body }`.
	/// The `{:else if}` branches are folded into one `TokenStream` of chained
	/// `else if` clauses first, then spliced between the `if` and the final
	/// `else` (only emitted if an `{:else}` was present).
	fn to_tokens_if_chain(
		if_statement: &Vec<TokenTree>,
		if_content: &[Content],
		if_else_chain: &[(Vec<TokenTree>, Vec<Content>)],
		else_content: &Option<Vec<Content>>,
		tokens: &mut TokenStream,
	) {
		let if_body = to_push_stmts(if_content);

		let if_else_chain_tokens = if_else_chain
			.iter()
			.fold(&mut TokenStream::new(), |acc, (statement, contents)| {
				let body = to_push_stmts(contents);
				quote! { else if #( #statement )* { #body } }.to_tokens(acc);
				acc
			})
			.to_owned();

		let output = if let Some(else_content) = else_content {
			let else_body = to_push_stmts(else_content);
			quote! {
				if #( #if_statement )* { #if_body } #if_else_chain_tokens else { #else_body }
			}
		} else {
			quote! {
				if #( #if_statement )* { #if_body } #if_else_chain_tokens
			}
		};

		tokens.append_all(output);
	}

	/// Emits `match match_statement { case_statement => { body }, ... }`, one
	/// arm per `{:case ...}`.
	fn to_tokens_match(
		match_statement: &Vec<TokenTree>,
		cases: &[(Vec<TokenTree>, Vec<Content>)],
		tokens: &mut TokenStream,
	) {
		let cases = cases
			.iter()
			.map(|(case_statement, case_content)| {
				let body = to_push_stmts(case_content);
				quote! {
					#( #case_statement )* => { #body }
				}
			})
			.collect::<Vec<_>>();

		let output = quote! {
			match #( #match_statement )* {
				#( #cases )*
			}
		};

		tokens.append_all(output);
	}
}
