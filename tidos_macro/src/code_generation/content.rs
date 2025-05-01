use crate::tokens::Content;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for Content {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Content::Tag(html_tag) => html_tag.to_tokens(tokens),
			Content::ControlTag(control_tag) => control_tag.to_tokens(tokens),
			Content::Literal(literal) => literal.to_tokens(tokens),
			Content::Expression(expr) => quote!(tidos::sanitize!(#expr)).to_tokens(tokens),
			Content::RawHTMLExpression(expr) => quote!(&#expr).to_tokens(tokens),
		}
	}
}
