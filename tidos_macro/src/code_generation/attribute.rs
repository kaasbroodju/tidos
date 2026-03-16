use crate::tokens::{Attribute, AttributeType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match &self {
			Attribute::ImplicitToggle { name } => { // :disabled
				let ident = format_ident!("{}", name);
				let attribute_name = name.to_string();

				tokens.append_all(quote! {
					if #ident { concat!(#attribute_name, " ") } else { "" }
				});
			}
			Attribute::ExplicitToggle { name, value } => { // :disabled={ true }
				let attribute_name = name.to_string();
				tokens.append_all(quote! {
					if #value { concat!(#attribute_name, " ") } else { "" }
				});
			}
			Attribute::Constant { name } => { // disabled
				let attribute_name = name.to_string();
				tokens.append_all(quote! {
					concat!(#attribute_name, " ")
				});
			}
			Attribute::ConstantLiteral { name, literal } => { // class="wrapper"
				let attribute_name = &name
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();

				tokens.append_all(quote! {
					concat!(#attribute_name, "=\"", #literal, "\" ")

					//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
				});
			}
			Attribute::ConstantGroup { name, contents } => { // value={ person.name }
				let attribute_name = &name
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();

				tokens.append_all(quote! {
					concat!(#attribute_name, "=\"") + &tidos::sanitize!(#contents) + "\" "

					//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
				});
			}
		}
	}
}

impl Attribute {
	pub fn to_tokens_custom_element(&self) -> TokenStream {
		match &self {
			Attribute::ImplicitToggle{ name } => { quote! { #name }}
			Attribute::ExplicitToggle{ name, value } => { quote! { #name: #value }}
			Attribute::Constant{ name } => { quote! { #name: true }}
			Attribute::ConstantLiteral{ name, literal } => { quote! { #name: #literal }}
			Attribute::ConstantGroup{ name, contents } => { quote! { #name: #contents } }
		}
	}
}
