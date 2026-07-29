//! Parses the contents of a `{ ... }` text block into a [`TextContent`]:
//! either a bare string literal, a format-string literal followed by
//! comma-separated parameter expressions, or a single arbitrary Rust
//! expression. Used both for plain `{...}` text and, via
//! `Content::parse_at_statement`, for `@html{...}`.

use crate::tokens::TextContent;
use proc_macro2::{Literal, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::Token;

/// Distinguishes the three `TextContent` forms by whether the brace starts
/// with a string literal: `{"lit"}` / `{"fmt {}", args...}` if so (further
/// split into `Literal` vs `Formatted` by whether any `,`-separated params
/// follow), otherwise `{expr}` is parsed as a raw token stream and kept
/// unparsed (it's re-parsed as a Rust expression later by `code_generation`).
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
