use crate::tokens::Component;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let children = &self.children;
		
		tokens.append_all(quote! {
			String::new() #( + #children )*
		});
	}
}
