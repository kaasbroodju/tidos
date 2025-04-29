use crate::tokens::Component;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let children = &self.children;
		let binding = "{}".repeat(children.len());
		let format_string = binding.as_str();
		let x = { String::from(String::new()) + &String::from("Hello") };
		tokens.append_all(quote! {
			String::new()
				#(
					+ #children
				)*

		});
	}
}
