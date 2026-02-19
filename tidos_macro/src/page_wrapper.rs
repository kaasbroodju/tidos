use crate::tokens::Component;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

pub struct PageWrapper {
	component: Component,
}

impl Parse for PageWrapper {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let component = Component::parse(input)?;
		Ok(PageWrapper { component })
	}
}

#[cfg(not(feature = "i18n"))]
impl ToTokens for PageWrapper {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let input = self.component.to_token_stream();
		tokens.append_all(quote! {
			{
				let mut page_output = Page::new();
				page_output.template = {
					let page = &mut page_output;
					#input
				};
				page_output
			}

		});
	}
}

#[cfg(feature = "i18n")]
impl ToTokens for PageWrapper {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let input = self.component.to_token_stream();
		tokens.append_all(quote! {
			{
				let mut page_output = Page::new(lang.0);
				page_output.template = {
					let page = &mut page_output;
					#input
				};
				page_output
			}

		});
	}
}

pub struct I18nHoist;

impl ToTokens for I18nHoist {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote! { page.lang });
	}
}
