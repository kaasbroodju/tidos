//! Lookahead helpers shared across `parsing/*`.
//!
//! Two families of functions live here: cursor-peeking predicates (the
//! `matches_*` / `is_cursor_on_*` functions) that let control-tag and tag
//! parsers decide *whether to stop* without consuming any tokens, and
//! [`HTMLutils`], which holds the actual tag-name/body parsing routines
//! shared by native elements and components. All of the `Cursor`-based
//! functions work by peeking into an upcoming `{ ... }` brace group and
//! comparing its first few tokens against a fixed pattern (e.g. `/` + a
//! tag-name ident for a closing tag, or `:` + `else` for an else branch).
//! They never advance the actual `ParseStream`.

use crate::tokens::Content;
use proc_macro2::{Delimiter, Ident, TokenTree};
use syn::buffer::Cursor;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::token::Lt;
use syn::Token;

/// Checks whether the upcoming `{ ... }` group is the closing tag for
/// `target_tag`, i.e. its first two tokens are `/` and an identifier equal to
/// `target_tag` (e.g. `{/for}`, `{/if}`, `{/match}`, `{/slot}`).
pub fn matches_corresponding_command_tag(cursor: Cursor, target_tag: &str) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() == 2
			&& matches!(&peeked[0], TokenTree::Punct(punct) if punct.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(ident) if *ident.to_string() == *target_tag)
	} else {
		false
	}
}

/// Checks whether the upcoming `{ ... }` group opens a `{:case ...}` match
/// arm, i.e. its first two tokens are `:` and the identifier `case`.
pub fn matches_case_statement(cursor: Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(3).collect();

		peeked.len() >= 3
			&& matches!(&peeked[0], TokenTree::Punct(punct) if punct.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(ident) if ident.to_string().as_str() == "case")
	} else {
		false
	}
}

/// Checks whether the upcoming `{ ... }` group starts a new branch of the
/// current `{#if}` chain: an `{:else if}`, an `{:else}`, or the chain's own
/// `{/if}`. Used as the stop condition when parsing an `{#if}` branch's body,
/// since all three end that branch's content.
pub fn is_cursor_on_new_if_branch(cursor: &Cursor) -> bool {
	is_cursor_on_else_if_branch(cursor)
		|| is_cursor_on_else_branch(cursor)
		|| is_cursor_on_end_of_if_branch(cursor)
}

/// Checks whether the upcoming `{ ... }` group is an `{:else if ...}` header,
/// i.e. its first three tokens are `:`, `else`, `if`.
pub fn is_cursor_on_else_if_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(3).collect();

		peeked.len() == 3
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "else")
			&& matches!(&peeked[2], TokenTree::Ident(i) if &i.to_string() == "if")
	} else {
		false
	}
}

/// Checks whether the upcoming `{ ... }` group is an `{:else}` header (not
/// followed by `if`), i.e. its first two tokens are `:`, `else`. Note this
/// also matches the start of an `{:else if}` header, so callers that need to
/// tell the two apart must check [`is_cursor_on_else_if_branch`] first.
pub fn is_cursor_on_else_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() == 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "else")
	} else {
		false
	}
}

/// Checks whether the upcoming `{ ... }` group is the `{/if}` that closes the
/// whole `{#if}` chain, i.e. its first two tokens are `/`, `if`.
pub fn is_cursor_on_end_of_if_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() == 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "if")
	} else {
		false
	}
}

/// Checks whether the upcoming tokens are exactly the closing tag
/// `</target_tag>` (hyphenated names supported). This is the loop condition
/// [`HTMLutils::parse_body`] uses to know when to stop reading children, so
/// it's what actually enforces that a native or component tag's closing name
/// matches its opening name; a mismatch just makes the loop keep consuming
/// content until it hits end-of-input or a *different* unexpected closing
/// tag, at which point [`peek_closing_tag_name`] supplies the tag name for
/// the error message.
pub fn matches_tag(cursor: Cursor, target_tag: &String) -> bool {
	let mut rest = cursor;
	if !matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '<') {
		return false;
	}
	let (_, next) = rest.punct().unwrap();
	rest = next;

	if !matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '/') {
		return false;
	}
	let (_, next) = rest.punct().unwrap();
	rest = next;

	let mut right_hand_side = String::new();
	let Some((ident, next)) = rest.ident() else {
		return false;
	};
	right_hand_side.push_str(ident.to_string().as_str());

	rest = next;

	while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
		let (_, next) = rest.punct().unwrap();
		rest = next;

		right_hand_side.push('-');
		let Some((ident, next)) = rest.ident() else {
			return false;
		};

		right_hand_side.push_str(ident.to_string().as_str());
		rest = next;
	}

	if right_hand_side != *target_tag {
		return false;
	}

	matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '>')
}
/// Namespace for tag-name and tag-body parsing helpers shared by native
/// elements and components (a unit struct rather than free functions, purely
/// for call-site grouping, e.g. `HTMLutils::extract_name(...)`).
pub struct HTMLutils;

impl HTMLutils {
	/// Reads a tag name (`p`, `div`, `custom-element`) starting at the current
	/// identifier. Hyphens are supported but optional, since native HTML
	/// custom elements conventionally contain one while ordinary tags don't.
	pub(crate) fn extract_name(
		input: ParseStream,
	) -> Result<(String, proc_macro2::Span), syn::Error> {
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let Some((ident, next)) = rest.ident() else {
				return Err(cursor.error("expected an HTML tag name like `p`, `div`, or `custom-element`"));
			};
			let span = ident.span();
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let (_, next) = rest.punct().unwrap();
				rest = next;

				output.push('-');
				let Some((ident, next)) = rest.ident() else {
					return Err(cursor.error("native html or custom elements cannot end with a hyphen:\n\tremove the `-`\n\tadd a name segment after `-`"));
				};

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			Ok(((output, span), rest))
		})
	}

	/// Checks whether the input is positioned right before `/>` (a
	/// self-closing tag), without consuming anything.
	pub(crate) fn is_peeking_at_self_closing_tag(input: ParseStream) -> bool {
		input.peek(Token![/]) && input.peek2(Token![>])
	}

	pub(crate) fn is_peeking_at_start_of_a_closing_tag(input: ParseStream) -> bool {
		input.peek(Token![<]) && input.peek2(Token![/])
	}

	/// Parses a tag's children: repeatedly parses `Content` until the
	/// upcoming tokens are that tag's own closing tag ([`matches_tag`]).
	/// Shared by native elements and components since both close the same
	/// way. Distinguishes "found a *different* closing tag" (a nesting
	/// mistake, reported with the found tag's name) from "ran out of input
	/// entirely" (a missing closing tag) so the error message points at the
	/// actual problem.
	pub(crate) fn parse_body(
		input: ParseStream,
		start_first_tag_token: Lt,
		tag: &String,
	) -> Result<Vec<Content>, syn::Error> {
		let mut children: Vec<Content> = Vec::new();
		loop {
			if input.is_empty() {
				return Err(syn::Error::new(
					start_first_tag_token.span(),
					format!("missing matching closing tag `</{tag}>`"),
				));
			}
			if matches_tag(input.cursor(), tag) {
				break;
			}
			if HTMLutils::is_peeking_at_start_of_a_closing_tag(input) {
				input.parse::<Token![<]>().expect("Peeked");
				let slash = input.parse::<Token![/]>().expect("Peeked");
				let found_tag = input.parse::<Ident>();
				let gt = input.parse::<Token![>]>();
				// It is peeking at a closing tag but not the right one
				return match (found_tag, gt) {
					(Ok(found_tag), Ok(_)) => Err(syn::Error::new(
						found_tag.span(),
						format!(
							"Unexpected closing tag `</{}>`, expected `</{}>`",
							found_tag, tag
						),
					)),
					(Ok(found_tag), Err(_)) => Err(syn::Error::new(
						found_tag.span(),
						format!("Tag was unproperly closed expected `</{}>`", found_tag),
					)),
					_ => Err(syn::Error::new(
						slash.span(),
						"Expected an HTML tag name like `p`, `div`, or `custom-element`"
							.to_string(),
					)),
				};
			}
			let child = input.parse::<Content>()?;
			children.push(child);
		}
		Ok(children)
	}

	/// Decides `<Foo>` vs `<foo>` by the tag name's first-letter casing: an
	/// upper-case first letter means a custom component, anything else a
	/// native HTML element. This is the one rule `Content::parse` relies on
	/// to route between `ComponentContent` and `NativeContent` parsing.
	pub(crate) fn peek_is_component(input: ParseStream) -> bool {
		let Some((_, cursor)) = input.cursor().punct() else {
			return false;
		};
		let Some((ident, _)) = cursor.ident() else {
			return false;
		};
		ident
			.to_string()
			.chars()
			.next()
			.unwrap()
			.is_ascii_uppercase()
	}
}
