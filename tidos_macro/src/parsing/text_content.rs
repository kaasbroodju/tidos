use crate::tokens::TextContent;
use proc_macro2::{Literal, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::Token;

impl Parse for TextContent {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let content;
		syn::braced!(content in input);
		if content.peek(syn::LitStr) {
			let base_string = content.parse::<Literal>()?;
			let mut params = vec![];
			while !content.is_empty() && content.peek(Token![,]) {
				content.parse::<Token![,]>()?;
				let mut contents = vec![];
				while !content.is_empty() && !content.peek(Token![,]) {
					contents.push(content.parse::<TokenTree>()?);
				}
				params.push(contents);
			}

			if params.is_empty() {
				Ok(TextContent::Literal(base_string))
			} else {
				Ok(TextContent::Formatted(base_string, params))
			}
		} else {
			let mut contents = vec![];
			while let Ok(token) = content.parse::<TokenTree>() {
				contents.push(token);
			}
			Ok(TextContent::Expression(contents))
		}
	}
}
