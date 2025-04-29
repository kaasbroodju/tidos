use crate::tokens::Attribute;
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match (&self.value, &self.is_toggle_attribute) {
			// :disabled
			(None, true) => {
				let ident = format_ident!("{}", &self.name);
				let attribute_name = &self.name.to_string();

				tokens.append_all(quote! {
					if #ident { concat!(#attribute_name, " ") } else { "" }
				});
			}
			// disabled
			(None, false) => {
				let attribute_name = &self.name.to_string();
				tokens.append_all(quote! {
					concat!(#attribute_name, " ")
				});
			}
			// :disabled={ true }
			(Some(value), true) => {
				let attribute_name = &self.name.to_string();
				tokens.append_all(quote! {
					if #value { concat!(#attribute_name, " ") } else { "" }
				});
			}
			// class="wrapper" or value={ person.name }
			(Some(value), false) => {
				let attribute_name = &(&self.name)
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();

				match value {
					TokenTree::Group(group) => {
						tokens.append_all(quote! {
							concat!(#attribute_name, "=\"") + &tidos::sanitize!(#group) + "\" "

							//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
						});
					}
					TokenTree::Literal(literal) => {
						tokens.append_all(quote! {
							concat!(#attribute_name, "=\"", #literal, "\" ")

							//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
						});
					}
					_ => {
						panic!("Tidos macro error: expected group or ident")
					}
				}
			}
		}
	}
}
