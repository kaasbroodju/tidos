use crate::tokens::{Attribute, AttributeType};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		match &self.attribute_type {
			AttributeType::ImplicitToggle => {
				let ident = format_ident!("{}", name);
				let attribute_name = name.to_string();
				tokens.append_all(quote! {
					if #ident { page.push_static(concat!(#attribute_name, " ")); }
				});
			}
			AttributeType::ExplicitToggle { value } => {
				let attribute_name = name.to_string();
				tokens.append_all(quote! {
					if #value { page.push_static(concat!(#attribute_name, " ")); }
				});
			}
			AttributeType::Constant => {
				let attribute_name = name.to_string();
				tokens.append_all(quote! {
					page.push_static(concat!(#attribute_name, " "));
				});
			}
			AttributeType::ConstantLiteral { literal } => {
				let attribute_name = name
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();
				tokens.append_all(quote! {
					page.push_static(concat!(#attribute_name, "=\"", #literal, "\" "));
				});
			}
			AttributeType::ConstantGroup { contents } => {
				let attribute_name = name
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();
				tokens.append_all(quote! {
					page.push_static(concat!(#attribute_name, "=\""));
					{ let _v = tidos::sanitize!(#contents); page.push_dynamic(_v); }
					page.push_static("\" ");
				});
			}
		}
	}
}

impl Attribute {
	pub fn to_tokens_custom_element(&self) -> TokenStream {
		let name = Ident::new(&self.name, self.name_span).to_token_stream();
		match &self.attribute_type {
			AttributeType::ImplicitToggle => {
				quote! { #name }
			}
			AttributeType::ExplicitToggle { value } => {
				quote! { #name: #value }
			}
			AttributeType::Constant => {
				quote! { #name: true }
			}
			AttributeType::ConstantLiteral { literal } => {
				quote! { #name: #literal }
			}
			AttributeType::ConstantGroup { contents } => {
				quote! { #name: #contents }
			}
		}
	}
}
