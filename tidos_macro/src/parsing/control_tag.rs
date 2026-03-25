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
	fn parse_for_loop_body(
		input: ParseStream,
		group: &Group,
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
	) -> syn::Result<Self> {
		// ...
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

	fn parse_match_body(
		input: ParseStream,
		group: Group,
		match_statement: Vec<TokenTree>,
	) -> syn::Result<Self> {
		// {:case ...}
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

	fn parse_slot_body(input: ParseStream, group: &Group, name: Ident) -> syn::Result<Self> {
		// ...
		let contents = Self::parse_content_until(&input, group.span(), SLOT_TAG, |cursor| {
			matches_corresponding_command_tag(cursor, SLOT_TAG)
		})?;

		// {/for}
		Self::parse_closing_tag(input, group.span(), SLOT_TAG, || ControlTag::Slot {
			name,
			contents,
		})
	}

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
