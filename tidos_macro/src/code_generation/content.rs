use crate::tokens::{Content, TextContent};
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};

impl ToTokens for Content {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Content::Tag(html_tag) => html_tag.to_tokens(tokens),
			Content::ControlTag(control_tag) => control_tag.to_tokens(tokens),
			Content::Text(text_content) => text_content.to_tokens(tokens),
			Content::RawHTMLExpression(text_content) => text_content.to_tokens_raw(tokens),
			Content::SlotRender(expr) => {
				quote! { (#( #expr )*)(page); }.to_tokens(tokens);
			}
		}
	}
}

impl ToTokens for TextContent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			TextContent::Literal(literal) => {
				let literal = TextContent::sanitize_literal(literal.clone());
				quote! { page.push_static(#literal); }.to_tokens(tokens);
			}
			TextContent::Formatted(literal, contents) => {
				let literal = TextContent::sanitize_literal(literal.clone());
				quote! {
					{ let _v = format!(#literal #( , #( #contents )* )* ); page.push_dynamic(_v); }
				}
				.to_tokens(tokens);
			}
			TextContent::Expression(expr) => {
				quote! {
					{ let _v = tidos::sanitize!( #( #expr )* ); page.push_dynamic(_v); }
				}
				.to_tokens(tokens);
			}
		}
	}
}

impl TextContent {
	pub fn to_tokens_raw(&self, tokens: &mut TokenStream) {
		match self {
			TextContent::Literal(literal) => {
				quote! { page.push_static(#literal); }.to_tokens(tokens);
			}
			TextContent::Formatted(literal, contents) => {
				quote! {
					{ let _v = format!(#literal #( , #( #contents )* )* ); page.push_dynamic(_v); }
				}
				.to_tokens(tokens);
			}
			TextContent::Expression(expr) => {
				quote! {
					{ let _v = #( #expr )*; page.push_dynamic(_v); }
				}
				.to_tokens(tokens);
			}
		}
	}

	pub(crate) fn sanitize_literal(literal: Literal) -> Literal {
		let input = &literal.to_string();
		let input = &input[1..input.len() - 1];

		if !input.contains(['&', '<', '>', '"', '\'']) {
			return literal;
		}

		let mut result = String::new();

		for c in input.chars() {
			match c {
				'&' => result.push_str("&amp;"),
				'<' => result.push_str("&lt;"),
				'>' => result.push_str("&gt;"),
				'"' => result.push_str("&quot;"),
				'\'' => result.push_str("&#x27;"),
				_ => result.push(c),
			}
		}

		Literal::string(result.as_str())
	}
}
