use crate::tokens::HTMLTag;
use crate::tokens::{Content, ControlTag};
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

const COMMAND_PREFIX: char = '#';
const RAW_HTML_PREFIX: char = '@';
const RAW_HTML_IDENTIFIER: &'static str = "html";

impl Parse for Content {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		
		if input.is_empty() {
			// Expected to have tokens over, but there's none.
			
			Err(syn::Error::new(input.span(), "No tokens left to parse"))
		
		} else if input.peek(Token![<]) {
			// <p></p>
			
			Ok(Content::Tag(input.parse::<HTMLTag>()?))
		
		} else if matches! { input.cursor().punct(), Some((punct, _)) if punct.as_char() == RAW_HTML_PREFIX} {
			// @html{"<p>hello world</p>"}

			Self::parse_raw_html_statement(input)
		
		} else if input.peek(Brace) && is_cursor_at_command(input.cursor()) {
			// {#for ... in ... } {#if} {#match}

			Ok(Content::ControlTag(ControlTag::parse(input)?))
				
		} else if input.peek(Brace) && !is_cursor_at_command(input.cursor()){
			// { name }
			
			Ok(Content::Expression(input.parse::<Group>()?))
			
		} else {
			// text between tags

			Self::parse_text_between_tags(input)
			
		}
	}
}

impl Content {
	fn parse_text_between_tags(input: ParseStream) -> syn::Result<Self> {
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
		Ok(Content::Literal(output))
	}

	fn parse_raw_html_statement(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Punct>()?;
		let ident = input.parse::<Ident>()?;
		if ident.to_string() != RAW_HTML_IDENTIFIER {
			Err(syn::Error::new(ident.span(), format!("Did you mean `{RAW_HTML_IDENTIFIER}`?")))
		} else {
			Ok(Content::RawHTMLExpression(input.parse::<Group>()?))
		}
	}
}

fn is_cursor_at_command(cursor: syn::buffer::Cursor) -> bool {
	// First check if we're at a brace
	if !cursor.group(Delimiter::Brace).is_some() {
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
