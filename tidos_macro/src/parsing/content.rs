//! The central dispatch point of the parser: `Content::parse` looks at the
//! next tokens and decides which grammar construct they belong to, then
//! delegates to the matching parser in `html_tag.rs`, `control_tag.rs`, or
//! `text_content.rs`. Every recursive parser in this module (for loop
//! bodies, if/match branches, tag children) bottoms out by calling back into
//! `Content::parse`, so this file effectively defines the shape of the whole
//! grammar even though most of the actual parsing logic lives elsewhere.

use crate::parsing::utils::HTMLutils;
use crate::tokens::TextContent;
use crate::tokens::{ComponentContent, Content, ControlTag, NativeContent};
use proc_macro2::{Delimiter, Ident, Punct, Spacing, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

const COMMAND_PREFIX: char = '#';
const RAW_HTML_PREFIX: char = '@';
const RAW_HTML_IDENTIFIER: &str = "html";
const SLOT_IDENTIFIER: &str = "slot";

/// Dispatches on the next token(s) to decide which `Content` variant to parse:
/// `<` starts a native element or component tag (distinguished by the tag
/// name's first-letter casing, see [`HTMLutils::peek_is_component`]), `@`
/// starts a raw-HTML or slot-render statement, and `{` starts either a
/// control tag (if the brace's first inner token is `#`) or a plain text
/// expression.
impl Parse for Content {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			unreachable!(
				"Content::parse called on empty stream: callers must check is_empty() first"
			);
		} else if input.peek(Token![<]) {
			if HTMLutils::peek_is_component(input) {
				// <Custom></Custom>
				Ok(Content::CustomComponent(input.parse::<ComponentContent>()?))
			} else {
				// <p></p>
				Ok(Content::NativeElement(input.parse::<NativeContent>()?))
			}
		} else if matches! { input.cursor().punct(), Some((punct, _)) if punct.as_char() == RAW_HTML_PREFIX}
		{
			// @html{"<p>Hello world</p>"} @html{"<p>Hello {}</p>", name} @html{ markdown }
			// @slot{self.body}

			Self::parse_at_statement(input)
		} else if input.peek(Brace) && is_cursor_at_command(input.cursor()) {
			// {#for ... in ... } {#if} {#match}

			Ok(Content::ControlTag(input.parse::<ControlTag>()?))
		} else if input.peek(Brace) && !is_cursor_at_command(input.cursor()) {
			// {"Hello world"} {"Hello {}", name} { name }

			Ok(Content::Text(input.parse::<TextContent>()?))
		} else {
			// TODO: legacy code remove after release candidate.
			// text between tags
			if let Ok(x) = Self::parse_text_between_tags(input) {
				panic!("Raw text between tags is no longer supported. Use `{{\"{x}\"}}` instead.")
			}
			unreachable!();
		}
	}
}

impl Content {
	/// Reassembles raw, brace-free text between tags purely so its contents
	/// can be shown in the panic message below. Bare text between tags is no
	/// longer accepted grammar (everything must be wrapped in `{ }`), so this
	/// is only ever reached to produce a helpful "use `{\"...\"}` instead"
	/// error; it never actually returns a `Content` value.
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

	/// Parses the `@`-prefixed statements: `@html{...}` (raw, unsanitized HTML)
	/// and `@slot{expr}` (renders a slot closure passed in as a component
	/// prop). The identifier right after `@` selects which one.
	fn parse_at_statement(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Punct>()?;
		let ident = input.parse::<Ident>()?;
		match ident.to_string().as_str() {
			RAW_HTML_IDENTIFIER => {
				let text_content = input.parse::<TextContent>()?;
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

	/// Collects the raw tokens inside `@slot{ ... }` without parsing them as a
	/// Rust expression here; they are re-emitted verbatim as a function-call
	/// expression by `code_generation` (`(#expr)(page)`), so keeping them as
	/// unparsed `TokenTree`s avoids committing to a specific expression
	/// grammar at this stage.
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

/// Looks inside a `{ ... }` group without consuming it, and reports whether
/// its first token is `#`, the marker that distinguishes a control tag
/// (`{#for}`, `{#if}`, `{#match}`, `{#slot:name}`) from a plain text
/// expression (`{expr}`). Needed because both start with the same `{` brace,
/// so `Content::parse` must peek one level deeper to tell them apart.
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
