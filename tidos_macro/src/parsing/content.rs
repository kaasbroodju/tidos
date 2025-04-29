use crate::parsing::utils::{
	is_cursor_on_else_branch, is_cursor_on_else_if_branch, is_cursor_on_end_of_if_branch,
	is_cursor_on_new_if_branch, matches_case_statement, matches_corresponding_command_tag,
};
use crate::tokens::HTMLTag;
use crate::tokens::{Content, ControlTag, TypeOfCommandTag};
use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

impl Parse for Content {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			panic!("No tokens left to parse")
		}
		// <p></p>
		if input.peek(Token![<]) {
			let temp = input.parse::<HTMLTag>()?;

			return Ok(Content::Tag(temp));

		// @html{"<p>hello world</p>"}
		} else if matches! { input.cursor().punct(), Some((punct, _)) if punct.as_char() == '@'} {
			input.parse::<Punct>()?;
			let ident = input.parse::<Ident>()?;
			if ident.to_string() != "html" {
				panic!()
			}
			let group = input.parse::<Group>()?;

			return Ok(Content::RawHTMLExpression(group));
		// {#for ... in ... }, {#if}, {#match} or { name }
		} else if input.peek(Brace) {
			let group = input.parse::<Group>()?;

			let is_command = matches! { group.stream().into_iter().take(1).collect::<Vec<_>>()[0], TokenTree::Punct(ref punct) if punct.as_char() == '#'};
			if !is_command {
				// { name }
				return Ok(Content::Expression(group));
			}

			// {#for ... in ... } {#if} {#match}
			let type_of_command = syn::parse2::<TypeOfCommandTag>(group.stream())?;

			return match type_of_command {
				TypeOfCommandTag::For {
					left_side,
					right_side,
				} => {
					// ...
					let mut contents: Vec<Content> = Vec::new();
					while !input.is_empty()
						&& !matches_corresponding_command_tag(input.cursor(), "for")
					{
						let child = input.parse::<Content>()?;
						contents.push(child);
					}

					// {/for}
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::For {
						left_side,
						right_side,
						contents,
					}))
				}
				TypeOfCommandTag::If(if_statement) => {
					let mut if_content: Vec<Content> = Vec::new();
					while !is_cursor_on_new_if_branch(&input.cursor()) {
						let child = input.parse::<Content>()?;
						if_content.push(child);
					}

					let mut if_else_chain = Vec::new();
					while is_cursor_on_else_if_branch(&input.cursor()) {
						let else_if_statement = input.parse::<Group>()?;
						let else_if_statement = else_if_statement
							.stream()
							.into_iter()
							.skip(3)
							.collect::<Vec<_>>();
						let mut children_else_if_branch: Vec<Content> = Vec::new();
						while !is_cursor_on_new_if_branch(&input.cursor()) {
							let child = input.parse::<Content>()?;
							children_else_if_branch.push(child);
						}

						if_else_chain.push((else_if_statement, children_else_if_branch));
					}

					let else_content = if is_cursor_on_else_branch(&input.cursor()) {
						input.parse::<Group>()?;

						let mut children_if_branch: Vec<Content> = Vec::new();
						while !is_cursor_on_new_if_branch(&input.cursor()) {
							let child = input.parse::<Content>()?;
							children_if_branch.push(child);
						}

						Some(children_if_branch)
					} else {
						None
					};

					if !is_cursor_on_end_of_if_branch(&input.cursor()) {
						input.error("Expected {/if}");
					}
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::IfChain {
						if_statement,
						if_content,
						if_else_chain,
						else_content,
					}))
				}
				TypeOfCommandTag::Match(match_statement) => {
					// {:case ...}
					let mut cases = Vec::new();

					while !input.is_empty()
						&& !matches_corresponding_command_tag(input.cursor(), "match")
					{
						let case = input.parse::<Group>()?;
						let mut children: Vec<Content> = Vec::new();
						while !matches_case_statement(input.cursor())
							&& !matches_corresponding_command_tag(input.cursor(), "match")
						{
							let child = input.parse::<Content>()?;
							children.push(child);
						}

						cases.push((
							case.stream()
								.into_iter()
								.skip(2)
								.collect::<Vec<TokenTree>>(),
							children,
						));
					}

					// {/match}>
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::Match {
						match_statement,
						cases,
					}))
				}
			};

			panic!();

		// text between tags
		} else {
			let mut output = String::new();
			let mut last_was_punct = false;
			while !input.is_empty() && !(input.peek(Token![<]) || input.peek(Brace)) {
				let token = input.parse::<TokenTree>()?;
				match token {
					TokenTree::Group(_) => {
						panic!()
					}
					TokenTree::Ident(ident) => {
						if !output.is_empty() {
							output.push(' ');
						}
						last_was_punct = false;
						output.push_str(ident.to_string().as_str());
					}
					TokenTree::Punct(punct) => {
						output.push(punct.as_char());

						// Check if the next token is also a punctuation
						if !output.is_empty()
							&& proc_macro2::Punct::peek(input.cursor())
							&& punct.spacing() == Spacing::Alone
						{
							output.push(' ');
						}

						last_was_punct = true;
					}
					TokenTree::Literal(lit) => {
						if !output.is_empty() {
							output.push(' ');
						}
						last_was_punct = false;
						output.push_str(lit.to_string().as_str());
					}
				}
			}
			return Ok(Content::Literal(output));
		}
	}
}

impl Parse for TypeOfCommandTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Token![#]>()?;

		if input.peek(Token![for]) {
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

				Err(cursor.error("No `in` found in for loop "))
			})?;

			let right_side = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("Empty right side of `in`."));
				}

				return Ok((output, rest));
			})?;

			// let x = input.parse::<Group>()?;
			return Ok(TypeOfCommandTag::For {
				left_side,
				right_side,
			});
		} else if input.peek(Token![match]) {
			input.parse::<Token![match]>()?;

			let match_content = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("No variable to match against."));
				}

				return Ok((output, rest));
			})?;

			return Ok(TypeOfCommandTag::Match(match_content));
		} else if input.peek(Token![if]) {
			input.parse::<Token![if]>()?;

			let if_content = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("If statement is empty."));
				}

				return Ok((output, rest));
			})?;

			return Ok(TypeOfCommandTag::If(if_content));
		} else {
			panic!("Unknown command tag");
		}
	}
}
