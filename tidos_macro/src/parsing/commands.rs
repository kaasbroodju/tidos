use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::Token;
use syn::token::Token;

use crate::parsing::utils::matches_corresponding_command_tag;
use crate::r#impl::Content;

pub enum TypeOfCommandTag {
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>
	},
	If(Vec<TokenTree>),
	Match(Vec<TokenTree>),
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

			let right_side  = input.step(|cursor| {
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
			return Ok(TypeOfCommandTag::For { left_side , right_side });
		} else if input.peek(Token![match]) {
			input.parse::<Token![match]>()?;

			let match_content  = input.step(|cursor| {
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

			return Ok(TypeOfCommandTag::Match(match_content))
		} else if input.peek(Token![if]) {
			input.parse::<Token![if]>()?;

			let if_content  = input.step(|cursor| {
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

struct ForLoopChildren(Vec<Content>);

impl Parse for ForLoopChildren {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut children: Vec<Content> = Vec::new();
		while !input.is_empty() && !matches_corresponding_command_tag(input.cursor(), "for") {
			let child = input.parse::<Content>()?;
			children.push(child);
		}

		Ok(ForLoopChildren(children))
	}
}