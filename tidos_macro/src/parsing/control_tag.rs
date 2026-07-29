//! Parses the four control-flow constructs: `{#for ... in ...}`, `{#if ...}`
//! (with `{:else if}`/`{:else}`), `{#match ...}`, and `{#slot:name}`, each
//! terminated by its own closing tag (`{/for}`, `{/if}`, `{/match}`,
//! `{/slot}`).
//!
//! Parsing happens in two passes. First, `TypeOfCommandTag::parse` looks only
//! at the command word right after `#` and captures whatever raw tokens make
//! up the loop pattern / iterator / condition / match scrutinee, *without*
//! parsing them as Rust syntax yet (see the `parse_for_loop` /
//! `parse_conditional_statements` / `parse_pattern_matching` functions on
//! `TypeOfCommandTag`). Second, `ControlTag::parse` uses that to figure out
//! which per-construct body parser to run, which recursively parses the
//! nested `Content` children and consumes the matching closing tag. The raw
//! token vectors collected in the first pass are only turned into real Rust
//! expressions later, in `code_generation`, where they get spliced back into
//! a `for`, `if`, or `match` via `quote!`.

use crate::parsing::utils::{
	is_cursor_on_else_branch, is_cursor_on_else_if_branch, is_cursor_on_end_of_if_branch,
	is_cursor_on_new_if_branch, matches_case_statement, matches_corresponding_command_tag,
};
use crate::tokens::{Content, ControlTag, TypeOfCommandTag};
use proc_macro2::{Group, Ident, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::token::Pound;
use syn::Token;

const LOOP_TAG: &str = "for";
const CONDITIONAL_TAG: &str = "if";
const MATCH_TAG: &str = "match";
const SLOT_TAG: &str = "slot";

/// Parses one `{#command ...}` group, identifies which of the four
/// constructs it opens via [`TypeOfCommandTag`], then runs that construct's
/// body parser to consume its children and closing tag.
impl Parse for ControlTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let group = input.parse::<Group>()?;
		let type_of_command = syn::parse2::<TypeOfCommandTag>(group.stream())?;

		let control_tag = match type_of_command {
			TypeOfCommandTag::For {
				left_side,
				right_side,
			} => Self::parse_for_loop_body(input, &group, left_side, right_side),
			TypeOfCommandTag::If(if_statement) => {
				Self::parse_if_statement_body(&input, &group, if_statement)
			}
			TypeOfCommandTag::Match(match_statement) => {
				Self::parse_match_body(input, group, match_statement)
			}
			TypeOfCommandTag::Slot(name) => Self::parse_slot_body(input, &group, name),
		}?;

		Ok(control_tag)
	}
}

impl ControlTag {
	/// Parses the body of `{#for left in right} ... {/for}` after the header
	/// has already been split into `left_side`/`right_side` token vectors.
	fn parse_for_loop_body(
		input: ParseStream,
		group: &Group,
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
	) -> syn::Result<Self> {
		let contents = Self::parse_content_until(&input, group.span(), LOOP_TAG, |cursor| {
			matches_corresponding_command_tag(cursor, LOOP_TAG)
		})?;

		// {/for}
		Self::parse_closing_tag(input, group.span(), LOOP_TAG, || ControlTag::For {
			left_side,
			right_side,
			contents,
		})
	}

	/// Parses `{#if ...} ... {:else if ...} ... {:else} ... {/if}`.
	///
	/// The `{:else if}` branches are collected first, one at a time, by
	/// looping for as long as the cursor is sitting on another `{:else if}`
	/// group. Only once that loop ends do we check for a single optional
	/// `{:else}`. The two ordering errors below exist because that loop
	/// structure can't reject "`{:else}` before `{:else if}`" or "two
	/// `{:else}` blocks" on its own: both would otherwise be silently
	/// swallowed as extra content or produce a confusing "missing closing tag"
	/// error, so they're checked explicitly right after the (single) `{:else}`
	/// body is parsed. The `skip(3)` used when reading an `{:else if ...}`
	/// header drops its fixed `:`, `else`, `if` prefix tokens, leaving just
	/// the condition.
	fn parse_if_statement_body(
		input: &ParseStream,
		group: &Group,
		if_statement: Vec<TokenTree>,
	) -> syn::Result<Self> {
		let if_content =
			Self::parse_content_until(input, group.span(), CONDITIONAL_TAG, |cursor| {
				is_cursor_on_new_if_branch(&cursor)
			})?;

		let mut if_else_chain = Vec::new();
		while is_cursor_on_else_if_branch(&input.cursor()) {
			let else_if_tag = input.parse::<Group>()?;
			let else_if_statement = else_if_tag.stream().into_iter().skip(3).collect::<Vec<_>>();
			let content_else_if_branch: Vec<Content> =
				Self::parse_content_until(input, group.span(), CONDITIONAL_TAG, |cursor| {
					is_cursor_on_new_if_branch(&cursor)
				})?;

			if_else_chain.push((else_if_statement, content_else_if_branch));
		}

		let else_content = if is_cursor_on_else_branch(&input.cursor()) {
			let else_group = input.parse::<Group>()?;

			let contents =
				Self::parse_content_until(input, group.span(), CONDITIONAL_TAG, |cursor| {
					is_cursor_on_end_of_if_branch(&cursor)
						|| is_cursor_on_else_if_branch(&cursor)
						|| is_cursor_on_else_branch(&cursor)
				})?;

			if is_cursor_on_else_if_branch(&input.cursor()) {
				return Err(syn::Error::new(
					else_group.span(),
					"{:else if} must come before {:else}, not after",
				));
			}

			if is_cursor_on_else_branch(&input.cursor()) {
				let duplicate = input.parse::<Group>()?;
				return Err(syn::Error::new(
					duplicate.span(),
					"cannot have two {:else} blocks in one {#if}",
				));
			}

			Some(contents)
		} else {
			None
		};

		Self::parse_closing_tag(input, group.span(), CONDITIONAL_TAG, || {
			ControlTag::IfChain {
				if_statement,
				if_content,
				if_else_chain,
				else_content,
			}
		})
	}

	/// Parses `{#match ...} {:case pat} ... {:case pat} ... {/match}`: repeats
	/// `{:case pattern} body` until the closing `{/match}` is seen, collecting
	/// each arm as a `(pattern_tokens, body)` pair. The `skip(2)` below drops
	/// the fixed `:` and `case` tokens that make up the arm header, leaving
	/// just the pattern tokens.
	fn parse_match_body(
		input: ParseStream,
		group: Group,
		match_statement: Vec<TokenTree>,
	) -> syn::Result<Self> {
		let mut cases = Vec::new();

		while !matches_corresponding_command_tag(input.cursor(), MATCH_TAG) {
			let case = input.parse::<Group>()?;

			let statement = case
				.stream()
				.into_iter()
				.skip(2)
				.collect::<Vec<TokenTree>>();

			let children = Self::parse_content_until(&input, case.span(), MATCH_TAG, |cursor| {
				matches_case_statement(cursor)
					|| matches_corresponding_command_tag(cursor, MATCH_TAG)
			})?;

			cases.push((statement, children));
		}

		// {/match}>
		Self::parse_closing_tag(input, group.span(), MATCH_TAG, || ControlTag::Match {
			match_statement,
			cases,
		})
	}

	/// Parses the body of `{#slot:name} ... {/slot}`: the named-slot content
	/// a parent passes into a child component, later turned into a boxed
	/// render closure prop by `code_generation` (see `ComponentContent`'s
	/// `ToTokens` impl).
	fn parse_slot_body(input: ParseStream, group: &Group, name: Ident) -> syn::Result<Self> {
		let contents = Self::parse_content_until(&input, group.span(), SLOT_TAG, |cursor| {
			matches_corresponding_command_tag(cursor, SLOT_TAG)
		})?;

		// {/for}
		Self::parse_closing_tag(input, group.span(), SLOT_TAG, || ControlTag::Slot {
			name,
			contents,
		})
	}

	/// Consumes the next `{...}` group and checks it is the expected closing
	/// tag (`{/tag_name}`) for `for`/`if`/`match`/`slot`; on success builds the
	/// finished `ControlTag` via `on_success` (deferred so callers don't have
	/// to construct it before knowing parsing succeeded). Shared by all four
	/// constructs since the closing-tag grammar is identical.
	fn parse_closing_tag<F>(
		input: ParseStream,
		group_span: proc_macro2::Span,
		tag_name: &str,
		on_success: F,
	) -> syn::Result<ControlTag>
	where
		F: FnOnce() -> ControlTag,
	{
		let Ok(group) = input.parse::<Group>() else {
			return Err(syn::Error::new(
				group_span,
				format!("missing matching closing tag `{{/{tag_name}}}`"),
			));
		};

		let peeked: Vec<TokenTree> = group.stream().into_iter().take(2).collect();

		if peeked.len() == 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(i) if *i.to_string() == *tag_name)
		{
			Ok(on_success())
		} else {
			Err(syn::Error::new(
				group.span(),
				format!("missing matching closing tag `{{/{tag_name}}}`"),
			))
		}
	}

	/// Parses `Content` children in a loop until `should_stop` reports that
	/// the cursor has reached the tag's own terminator (a closing tag or the
	/// next branch, depending on the construct). Each construct passes a
	/// different `should_stop` predicate: e.g. an `{#if}` branch also stops on
	/// `{:else if}`/`{:else}`, not just `{/if}`.
	fn parse_content_until<F>(
		input: &ParseStream,
		group_span: proc_macro2::Span,
		tag_name: &str,
		should_stop: F,
	) -> Result<Vec<Content>, syn::Error>
	where
		F: Fn(syn::buffer::Cursor) -> bool,
	{
		let mut contents: Vec<Content> = Vec::new();
		while !should_stop(input.cursor()) {
			let child = input.parse::<Content>()?;
			contents.push(child);
			if input.is_empty() {
				return Err(syn::Error::new(
					group_span,
					format!("missing matching closing tag `{{/{tag_name}}}`"),
				));
			}
		}
		Ok(contents)
	}
}

/// Reads the command word directly after `#` and dispatches to the matching
/// header parser. This only parses the *header* (the part between `{#` and
/// the closing `}` of the opening tag itself); the body and closing tag are
/// handled afterwards by `ControlTag::parse`.
impl Parse for TypeOfCommandTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let command_token = input.parse::<Token![#]>()?;

		if input.peek(Token![for]) {
			Self::parse_for_loop(input, command_token)
		} else if input.peek(Token![match]) {
			Self::parse_pattern_matching(input, command_token)
		} else if input.peek(Token![if]) {
			Self::parse_conditional_statements(input, command_token)
		} else if input.peek(syn::Ident) {
			Self::parse_slot(input, command_token)
		} else {
			Err(syn::Error::new(
				command_token.span(),
				"Unknown command tag, must be: 'for', 'if', 'match' or 'slot'",
			))
		}
	}
}

impl TypeOfCommandTag {
	/// Splits `for <left_side> in <right_side>` into its two raw token halves
	/// by manually scanning token trees for the `in` keyword, rather than
	/// asking `syn` to parse a pattern. `syn::Pat` doesn't cover every
	/// destructuring shape used here in a single reusable parser, and the
	/// tokens don't need to be validated as a pattern until `code_generation`
	/// splices them into a real `for (left) in (right)` loop anyway, so
	/// deferring that parsing keeps this step permissive and simple.
	fn parse_for_loop(
		input: ParseStream,
		command_token: Pound,
	) -> Result<TypeOfCommandTag, syn::Error> {
		input.parse::<Token![for]>()?;

		let left_side = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = Vec::new();
			while let Some((tt, next)) = rest.token_tree() {
				if tt.to_string().as_str() == "in" {
					return Ok((output, next));
				} else {
					output.push(tt);
					rest = next;
				}
			}

			Err(syn::Error::new(
				command_token.span(),
				"No `in` found in for loop ",
			))
		})?;

		if left_side.is_empty() {
			return Err(syn::Error::new(
				command_token.span(),
				"expected a binding variable before `in`, e.g. `{#for item in items}`",
			));
		}

		let right_side = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = Vec::new();
			while let Some((tt, next)) = rest.token_tree() {
				output.push(tt);
				rest = next;
			}

			if output.is_empty() {
				Err(syn::Error::new(
					command_token.span(),
					"Empty right side of `in`.",
				))
			} else {
				Ok((output, rest))
			}
		})?;

		Ok(TypeOfCommandTag::For {
			left_side,
			right_side,
		})
	}

	/// Captures every remaining token in the `{#match ...}` header verbatim as
	/// the scrutinee expression, re-parsed as Rust only once `code_generation`
	/// splices it into an actual `match ... { }`.
	fn parse_pattern_matching(
		input: ParseStream,
		command_token: Pound,
	) -> Result<TypeOfCommandTag, syn::Error> {
		input.parse::<Token![match]>()?;

		let match_content = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = Vec::new();
			while let Some((tt, next)) = rest.token_tree() {
				output.push(tt);
				rest = next;
			}

			if output.is_empty() {
				Err(syn::Error::new(
					command_token.span(),
					"No variable to match against.",
				))
			} else {
				Ok((output, rest))
			}
		})?;

		Ok(TypeOfCommandTag::Match(match_content))
	}

	/// Captures every remaining token in the `{#if ...}` header verbatim as
	/// the condition expression, for the same reason as
	/// [`Self::parse_pattern_matching`]: it's re-parsed as Rust only once
	/// `code_generation` splices it into an actual `if ... { }`.
	fn parse_conditional_statements(
		input: ParseStream,
		command_token: Pound,
	) -> Result<TypeOfCommandTag, syn::Error> {
		input.parse::<Token![if]>()?;

		let if_content = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = Vec::new();
			while let Some((tt, next)) = rest.token_tree() {
				output.push(tt);
				rest = next;
			}

			if output.is_empty() {
				Err(syn::Error::new(
					command_token.span(),
					"If statement is empty.",
				))
			} else {
				Ok((output, rest))
			}
		})?;

		Ok(TypeOfCommandTag::If(if_content))
	}

	/// Parses `slot:name` after `#`. Unlike `for`/`if`/`match`, `slot` isn't a
	/// Rust keyword, so the dispatcher in `Parse for TypeOfCommandTag` can
	/// only peek a generic identifier and defer here to check it's literally
	/// `"slot"`, producing the same "unknown command tag" error as an
	/// unrecognized keyword would.
	fn parse_slot(
		input: ParseStream,
		command_token: Pound,
	) -> Result<TypeOfCommandTag, syn::Error> {
		let slot_ident = input.parse::<syn::Ident>()?;
		if &slot_ident.to_string() == "slot" {
			input.parse::<Token![:]>()?;
			let name_ident = input.parse::<syn::Ident>()?;
			Ok(TypeOfCommandTag::Slot(name_ident))
		} else {
			Err(syn::Error::new(
				command_token.span(),
				"Unknown command tag, must be: 'for', 'if', 'match' or 'slot'",
			))
		}
	}
}
