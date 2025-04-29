use crate::tokens::Attribute;
use crate::tokens::Content;
use proc_macro2::{Group, Literal};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::Token;

fn matches_tag(cursor: Cursor, target_tag: String) -> bool {
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
