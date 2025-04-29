use proc_macro2::{Delimiter, TokenTree};
use syn::buffer::Cursor;

pub fn matches_corresponding_command_tag(cursor: Cursor, target_tag: &str) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() == 2
			&& matches!(peeked[0], TokenTree::Punct(ref punct) if punct.as_char() == '/')
			&& matches!(peeked[1], TokenTree::Ident(ref ident) if ident.to_string() == target_tag)
	} else {
		false
	}
}

pub fn matches_case_statement(cursor: Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(3).collect();

		peeked.len() >= 3
			&& matches!(peeked[0], TokenTree::Punct(ref punct) if punct.as_char() == ':')
			&& matches!(peeked[1], TokenTree::Ident(ref ident) if ident.to_string().as_str() == "case")
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
			&& matches!(&peeked[1], TokenTree::Ident(i) if i.to_string() == "else")
			&& matches!(&peeked[2], TokenTree::Ident(i) if i.to_string() == "if")
	} else {
		false
	}
}

pub fn is_cursor_on_else_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() >= 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == ':')
			&& matches!(&peeked[1], TokenTree::Ident(i) if i.to_string() == "else")
	} else {
		false
	}
}

pub fn is_cursor_on_end_of_if_branch(cursor: &Cursor) -> bool {
	if let Some((a, _, _)) = cursor.group(Delimiter::Brace) {
		let peeked: Vec<TokenTree> = a.token_stream().into_iter().take(2).collect();

		peeked.len() >= 2
			&& matches!(&peeked[0], TokenTree::Punct(p) if p.as_char() == '/')
			&& matches!(&peeked[1], TokenTree::Ident(i) if i.to_string() == "if")
	} else {
		false
	}
}

pub fn matches_tag(cursor: Cursor, target_tag: String) -> bool {
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
	let (ident, next) = rest
		.ident()
		.expect("Expected an html like <p> or <custom-element>");
	right_hand_side.push_str(ident.to_string().as_str());

	rest = next;

	while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
		let next = rest.punct().unwrap().1;
		rest = next;

		right_hand_side.push('-');
		let (ident, next) = rest
			.ident()
			.expect("Expected an html like <p> or <custom-element>");

		right_hand_side.push_str(ident.to_string().as_str());
		rest = next;
	}

	if right_hand_side != target_tag {
		return false;
	}

	if !matches!(rest.punct(), Some((punct, _)) if punct.as_char() == '>') {
		return false;
	}

	true
}
