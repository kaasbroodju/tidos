use proc_macro2::{Delimiter, TokenTree};
use syn::buffer::Cursor;

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
