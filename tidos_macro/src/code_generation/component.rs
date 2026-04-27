use crate::tokens::{Component, IsStatic};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let children = &self.children;
		if self.is_static() {
			tokens.append_all(quote! { String::from(concat!( #( #children ),* ))});
		} else {
			tokens.append_all(quote! { tidos::combine!(String::new() #( , #children )* )});
		}
	}
}
