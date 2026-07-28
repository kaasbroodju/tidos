use crate::tokens::Content;
use proc_macro2::{Delimiter, TokenTree};
use syn::buffer::Cursor;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::token::Lt;
use syn::Token;

pub fn matches_corresponding_command_tag(cursor: Cursor, target_tag: &str) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() == 2
			&& matches!(&peeked[0], TokenTree::Punct(punct) if punct.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(ident) if *ident.to_string() == *target_tag)
	} else {
		false
	}
}

pub fn matches_case_statement(cursor: Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(3).collect();

		peeked.len() >= 3
			&& matches!(&peeked[0], TokenTree::Punct(punct) if punct.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(ident) if ident.to_string().as_str() == "case")
	} else {
		false
	}
}

pub fn is_cursor_on_new_if_branch(cursor: &Cursor) -> bool {
	is_cursor_on_else_if_branch(cursor)
		|| is_cursor_on_else_branch(cursor)
		|| is_cursor_on_end_of_if_branch(cursor)
}

pub fn is_cursor_on_else_if_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(3).collect();

		peeked.len() >= 3
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "else")
			&& matches!(&peeked[2], TokenTree::Ident(i) if &i.to_string() == "if")
	} else {
		false
	}
}

pub fn is_cursor_on_else_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() >= 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "else")
	} else {
		false
	}
}

pub fn is_cursor_on_end_of_if_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() >= 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(i) if &i.to_string() == "if")
	} else {
		false
	}
}

pub fn peek_closing_tag_name(cursor: Cursor) -> Option<String> {
	let mut rest = cursor;
	let (p, next) = rest.punct()?;
	if p.as_char() != '<' {
		return None;
	}
	rest = next;
	let (p, next) = rest.punct()?;
	if p.as_char() != '/' {
		return None;
	}
	rest = next;
	let (ident, mut rest) = rest.ident()?;
	let mut name = ident.to_string();

	while let Some((p, next)) = rest.punct() {
		if p.as_char() != '-' {
			break;
		}
		if let Some((ident, next)) = next.ident() {
			name.push('-');
			name.push_str(&ident.to_string());
			rest = next;
		} else {
			break;
		}
	}

	Some(name)
}

pub fn matches_tag(cursor: Cursor, target_tag: &String) -> bool {
	let mut rest = cursor;
	if !matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '<') {
		return false;
	}
	let (_, next) = rest.punct().unwrap();
	rest = next;

	if !matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '/') {
		return false;
	}
	let (_, next) = rest.punct().unwrap();
	rest = next;

	let mut right_hand_side = String::new();
	let Some((ident, next)) = rest.ident() else {
		return false;
	};
	right_hand_side.push_str(ident.to_string().as_str());

	rest = next;

	while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
		let (_, next) = rest.punct().unwrap();
		rest = next;

		right_hand_side.push('-');
		let Some((ident, next)) = rest.ident() else {
			return false;
		};

		right_hand_side.push_str(ident.to_string().as_str());
		rest = next;
	}

	if right_hand_side != *target_tag {
		return false;
	}

	matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '>')
}
pub struct HTMLutils;

impl HTMLutils {
	pub(crate) fn extract_name(
		input: ParseStream,
	) -> Result<(String, proc_macro2::Span), syn::Error> {
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let Some((ident, next)) = rest.ident() else {
				return Err(cursor.error("expected an HTML tag name like `p`, `div`, or `custom-element`"));
			};
			let span = ident.span();
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let (_, next) = rest.punct().unwrap();
				rest = next;

				output.push('-');
				let Some((ident, next)) = rest.ident() else {
					return Err(cursor.error("native html or custom elements cannot end with a hyphen:\n\tremove the `-`\n\tadd a name segment after `-`"));
				};

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			Ok(((output, span), rest))
		})
	}

	pub(crate) fn is_peeking_at_self_closing_tag(input: ParseStream) -> bool {
		input.peek(Token![/]) && input.peek2(Token![>])
	}

	pub(crate) fn parse_body(
		input: ParseStream,
		start_first_tag_token: Lt,
		tag: &String,
	) -> Result<Vec<Content>, syn::Error> {
		let mut children: Vec<Content> = Vec::new();
		while !matches_tag(input.cursor(), tag) {
			if let Some(found_tag) = peek_closing_tag_name(input.cursor()) {
				return Err(syn::Error::new(
					input.span(),
					format!("unexpected closing tag `</{found_tag}>`, expected `</{tag}>`"),
				));
			}
			let child = input.parse::<Content>()?;
			children.push(child);
			if input.is_empty() {
				return Err(syn::Error::new(
					start_first_tag_token.span(),
					format!("missing matching closing tag `</{tag}>`"),
				));
			}
		}
		Ok(children)
	}

	pub(crate) fn peek_is_component(input: ParseStream) -> bool {
		let Some((_, cursor)) = input.cursor().punct() else {
			return false;
		};
		let Some((ident, _)) = cursor.ident() else {
			return false;
		};
		ident
			.to_string()
			.chars()
			.next()
			.unwrap()
			.is_ascii_uppercase()
	}
}
