use proc_macro2::Punct;
use proc_macro2::{Group, Ident, Literal, Spacing, TokenTree};
use quote::ToTokens;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

use crate::parsing::commands::TypeOfCommandTag;
use crate::parsing::utils::{
	is_cursor_on_else_branch, is_cursor_on_else_if_branch, is_cursor_on_end_of_if_branch,
	is_cursor_on_new_if_branch, matches_case_statement, matches_corresponding_command_tag,
};
use crate::r#impl::{Attribute, Component, Content, ControlTag, HTMLTag};

impl Parse for Content {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			panic!("No tokens left to parse")
		}
		// <p></p>
		if input.peek(Token![<]) {
			let temp = input.parse::<HTMLTag>()?;

			return Ok(Content::Tag(temp));

		// @html{"<p>hello world</p>"}
		} else if matches! { input.cursor().punct(), Some((punct, _)) if punct.as_char() == '@'} {
			input.parse::<Punct>()?;
			let ident = input.parse::<Ident>()?;
			if ident.to_string() != "html" {
				panic!()
			}
			let group = input.parse::<Group>()?;

			return Ok(Content::RawHTMLExpression(group));
		// {#for ... in ... }, {#if}, {#match} or { name }
		} else if input.peek(Brace) {
			let group = input.parse::<Group>()?;

			let is_command = matches! { group.stream().into_iter().take(1).collect::<Vec<_>>()[0], TokenTree::Punct(ref punct) if punct.as_char() == '#'};
			if !is_command {
				// { name }
				return Ok(Content::Expression(group));
			}

			// {#for ... in ... } {#if} {#match}
			let type_of_command = syn::parse2::<TypeOfCommandTag>(group.stream())?;

			return match type_of_command {
				TypeOfCommandTag::For {
					left_side,
					right_side,
				} => {
					// ...
					let mut contents: Vec<Content> = Vec::new();
					while !input.is_empty()
						&& !matches_corresponding_command_tag(input.cursor(), "for")
					{
						let child = input.parse::<Content>()?;
						contents.push(child);
					}

					// {/for}
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::For {
						left_side,
						right_side,
						contents,
					}))
				}
				TypeOfCommandTag::If(if_statement) => {
					let mut if_content: Vec<Content> = Vec::new();
					while !is_cursor_on_new_if_branch(&input.cursor()) {
						let child = input.parse::<Content>()?;
						if_content.push(child);
					}

					let mut if_else_chain = Vec::new();
					while is_cursor_on_else_if_branch(&input.cursor()) {
						let else_if_statement = input.parse::<Group>()?;
						let else_if_statement = else_if_statement
							.stream()
							.into_iter()
							.skip(3)
							.collect::<Vec<_>>();
						let mut children_else_if_branch: Vec<Content> = Vec::new();
						while !is_cursor_on_new_if_branch(&input.cursor()) {
							let child = input.parse::<Content>()?;
							children_else_if_branch.push(child);
						}

						if_else_chain.push((else_if_statement, children_else_if_branch));
					}

					let else_content = if is_cursor_on_else_branch(&input.cursor()) {
						input.parse::<Group>()?;

						let mut children_if_branch: Vec<Content> = Vec::new();
						while !is_cursor_on_new_if_branch(&input.cursor()) {
							let child = input.parse::<Content>()?;
							children_if_branch.push(child);
						}

						Some(children_if_branch)
					} else {
						None
					};

					if !is_cursor_on_end_of_if_branch(&input.cursor()) {
						input.error("Expected {/if}");
					}
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::IfChain {
						if_statement,
						if_content,
						if_else_chain,
						else_content,
					}))
				}
				TypeOfCommandTag::Match(match_statement) => {
					// {:case ...}
					let mut cases = Vec::new();

					while !input.is_empty()
						&& !matches_corresponding_command_tag(input.cursor(), "match")
					{
						let case = input.parse::<Group>()?;
						let mut children: Vec<Content> = Vec::new();
						while !matches_case_statement(input.cursor())
							&& !matches_corresponding_command_tag(input.cursor(), "match")
						{
							let child = input.parse::<Content>()?;
							children.push(child);
						}

						cases.push((
							case.stream()
								.into_iter()
								.skip(2)
								.collect::<Vec<TokenTree>>(),
							children,
						));
					}

					// {/match}>
					input.parse::<Group>()?;

					Ok(Content::ControlTag(ControlTag::Match {
						match_statement,
						cases,
					}))
				}
			};

			panic!();

		// text between tags
		} else {
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
			return Ok(Content::Literal(output));
		}
	}
}

impl Parse for HTMLTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// <p>
		input.parse::<Token![<]>()?;
		let left_tag = input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let (ident, next) = rest
				.ident()
				.expect("Expected an html like <p> or <custom-element>");
			output.push_str(ident.to_string().as_str());

			rest = next;

			// parse custom element's tag name.
			// custom elements should contain '-', however it is not required.
			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let next = rest.punct().unwrap().1;
				rest = next;

				output.push('-');
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like <p> or <custom-element>");

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			return Ok((output, rest));
		})?;

		let mut attributes = Vec::new();
		while !((input.peek(Token![/]) && input.peek2(Token![>])) || input.peek(Token![>])) {
			let is_toggle_attribute = input.parse::<Token![:]>().is_ok();

			// todo also allow 'in names'
			// let name = input.parse::<Ident>()?.to_token_stream();
			let name = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = String::new();
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like <p> or <custom-element>");
				output.push_str(ident.to_string().as_str());

				rest = next;

				// parse custom element's tag name.
				// custom elements should contain '-', however it is not required.
				while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
					let next = rest.punct().unwrap().1;
					rest = next;

					output.push('-');
					let (ident, next) = rest
						.ident()
						.expect("Expected an html like <p> or <custom-element>");

					output.push_str(ident.to_string().as_str());
					rest = next;
				}

				return Ok((output, rest));
			})?;

			if input.parse::<Token![=]>().is_ok() {
				if let Ok(literal) = input.parse::<Literal>() {
					if is_toggle_attribute {
						panic!("Unable to have a toggle attribute from a literal");
					}

					attributes.push(Attribute {
						is_toggle_attribute,
						name,
						value: Some(proc_macro2::TokenTree::Literal(literal)),
					});
				} else if let Ok(group) = input.parse::<Group>() {
					attributes.push(Attribute {
						is_toggle_attribute,
						name,
						value: Some(proc_macro2::TokenTree::Group(group)),
					});
				} else {
					panic!("Expected a literal \"\" or a group {{}}");
				}
			} else {
				attributes.push(Attribute {
					is_toggle_attribute,
					name,
					value: None,
				});
			}
		}

		// self closing tags, like <img />
		if input.peek(Token![/]) && input.peek2(Token![>]) {
			input.parse::<Token![/]>()?;
			input.parse::<Token![>]>()?;

			return Ok(HTMLTag {
				tag: left_tag.to_string(),
				attributes,
				children: vec![],
				is_self_closing: true,
			});
		}

		input.parse::<Token![>]>()?;

		// ...
		let mut children: Vec<Content> = Vec::new();
		while !input.is_empty() && !matches_tag(input.cursor(), left_tag.to_string()) {
			let child = input.parse::<Content>()?;
			children.push(child);
		}

		// </p>
		input.parse::<Token![<]>()?;
		input.parse::<Token![/]>()?;
		// right tag
		input.step(|cursor| {
			let mut rest = *cursor;
			let mut output = String::new();
			let (ident, next) = rest
				.ident()
				.expect("Expected an html like </p> or </custom-element>");
			output.push_str(ident.to_string().as_str());

			rest = next;

			while matches!(rest.punct(), Some((p, _)) if p.as_char() == '-') {
				let next = rest.punct().unwrap().1;
				rest = next;

				output.push('-');
				let (ident, next) = rest
					.ident()
					.expect("Expected an html like </p> or </custom-element>");

				output.push_str(ident.to_string().as_str());
				rest = next;
			}

			return Ok((output, rest));
		})?;
		input.parse::<Token![>]>()?;

		Ok(HTMLTag {
			tag: left_tag.to_string(),
			attributes,
			children,
			is_self_closing: false,
		})
	}
}

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

// fn is_next_a_open_bracket(cursor: Cursor) -> bool {
//     matches!(cursor.group(), TokenTree::Group(ref group) if group.delim_span() == Delimiter::Brace)
// }

impl Parse for Component {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Ok(Component {
				children: Vec::new(),
			});
		}

		let mut children: Vec<Content> = Vec::new();

		while !input.is_empty() {
			let child = input.parse::<Content>()?;
			children.push(child);
		}

		Ok(Component { children })
	}
}

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
