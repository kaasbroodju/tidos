use crate::tokens::{Content, ControlTag};
use crate::tokens::{HTMLTag, TextContent};
use proc_macro2::{Delimiter, Ident, Literal, Punct, Spacing, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

const COMMAND_PREFIX: char = '#';
const RAW_HTML_PREFIX: char = '@';
const RAW_HTML_IDENTIFIER: &str = "html";
const SLOT_IDENTIFIER: &str = "slot";

impl Parse for Content {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			unreachable!(
				"Content::parse called on empty stream — callers must check is_empty() first"
			);
		} else if input.peek(Token![<]) {
			// <p></p>

			Ok(Content::Tag(input.parse::<HTMLTag>()?))
		} else if matches! { input.cursor().punct(), Some((punct, _)) if punct.as_char() == RAW_HTML_PREFIX}
		{
			// @html{"<p>Hello world</p>"} @html{"<p>Hello {}</p>", name} @html{ markdown }
			// @slot{self.body}

			Self::parse_at_statement(input)
		} else if input.peek(Brace) && is_cursor_at_command(input.cursor()) {
			// {#for ... in ... } {#if} {#match}

			Ok(Content::ControlTag(ControlTag::parse(input)?))
		} else if input.peek(Brace) && !is_cursor_at_command(input.cursor()) {
			// {"Hello world"} {"Hello {}", name} { name }
			let text_content = Self::parse_text_content(input)?;
			Ok(Content::Text(text_content))
		} else {
			// text between tags
			if let Ok(x) = Self::parse_text_between_tags(input) {
				panic!("Raw text between tags is no longer supported. Use `{{\"{x}\"}}` instead.")
			}
			unreachable!();
		}
	}
}

impl Content {
	fn parse_text_content(input: ParseStream) -> syn::Result<TextContent> {
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

	fn parse_text_between_tags(input: ParseStream) -> syn::Result<String> {
		let mut output = String::new();
		while !(input.is_empty() || input.peek(Token![<]) || input.peek(Brace)) {
			let token = input.parse::<TokenTree>()?;
			match token {
				TokenTree::Group(_) => {
					panic!()
				}
				TokenTree::Ident(ident) => {
					if !output.is_empty() {
						output.push(' ');
					}
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
				}
				TokenTree::Literal(lit) => {
					if !output.is_empty() {
						output.push(' ');
					}

					output.push_str(lit.to_string().as_str());
				}
			}
		}
		Ok(output)
	}

	fn parse_at_statement(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Punct>()?;
		let ident = input.parse::<Ident>()?;
		match ident.to_string().as_str() {
			RAW_HTML_IDENTIFIER => {
				let text_content = Self::parse_text_content(input)?;
				Ok(Content::RawHTMLExpression(text_content))
			}
			SLOT_IDENTIFIER => {
				let expr = Self::parse_slot_expr(input)?;
				Ok(Content::SlotRender(expr))
			}
			other => Err(syn::Error::new(
				ident.span(),
				format!(
					"Did you mean `{RAW_HTML_IDENTIFIER}` or `{SLOT_IDENTIFIER}`? Got `{other}`"
				),
			)),
		}
	}

	fn parse_slot_expr(input: ParseStream) -> syn::Result<Vec<TokenTree>> {
		let content;
		syn::braced!(content in input);
		let mut tokens = vec![];
		while let Ok(token) = content.parse::<TokenTree>() {
			tokens.push(token);
		}
		Ok(tokens)
	}
}

fn is_cursor_at_command(cursor: syn::buffer::Cursor) -> bool {
	// First check if we're at a brace
	if cursor.group(Delimiter::Brace).is_none() {
		return false;
	}

	// If we have a group, check its first token
	if let Some((inside_cursor, _, _)) = cursor.group(Delimiter::Brace) {
		// Look at the first token inside the group
		let first_cursor = inside_cursor;

		// Check if first token is a # punctuation
		if let Some((punct, _)) = first_cursor.punct() {
			return punct.as_char() == COMMAND_PREFIX;
		}
	}

	false
}
