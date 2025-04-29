use crate::parsing::utils::matches_corresponding_command_tag;
use crate::tokens::{Content, ControlTag};
use proc_macro2::{Group, Ident, Punct, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for ControlTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// {#

		if input.peek(Token![for]) {
			let gr = input.parse::<Group>()?;
			// input.parse::<Token![for]>()?;
			// ...
			let mut children: Vec<Content> = Vec::new();
			while !input.is_empty() && !matches_corresponding_command_tag(input.cursor(), "for") {
				let child = input.parse::<Content>()?;
				children.push(child);
			}

			return Ok(ControlTag::Match {
				match_statement: vec![],
				cases: vec![],
			});
		} else if input.peek(Token![if]) {
		} else if input.peek(Token![match]) {
		} else {
			panic!("invalid logic block");
		}

		input.parse::<Punct>()?;
		input.parse::<Token![#]>()?;
		input.parse::<Token![<]>()?;
		let left_tag: Ident = input.parse()?;
		input.parse::<Token![>]>()?;

		// {/for}
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		input.parse::<Ident>()?;
		input.parse::<Token![>]>()?;

		return Ok(ControlTag::Match {
			match_statement: vec![],
			cases: vec![],
		});
	}
}
