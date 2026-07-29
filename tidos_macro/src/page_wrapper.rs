//! Backs the `page!` macro: parses the same [`Component`] grammar as
//! `view!`/`head!`, but wraps the generated push-statements in the
//! boilerplate that creates a `Page`, renders into it, and returns it.
//!
//! The only difference from `view!`'s expansion is *what* gets returned
//! (`Page` instead of the accumulated `String`) and *how* the page is
//! constructed, which is why this lives in its own file rather than being
//! folded into `tokens::Component`'s own `ToTokens` impl.

use crate::tokens::Component;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

/// Thin wrapper around [`Component`] so `page!` can have its own `ToTokens`
/// impl distinct from the one used by `view!`/`head!`.
pub struct PageWrapper {
	component: Component,
}

/// Parses exactly like a plain [`Component`]; the wrapping only affects code
/// generation, not the accepted grammar.
impl Parse for PageWrapper {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let component = Component::parse(input)?;
		Ok(PageWrapper { component })
	}
}

/// Without the `i18n` feature, `Page::new()` takes no locale.
#[cfg(not(feature = "i18n"))]
impl ToTokens for PageWrapper {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let input = self.component.to_token_stream();
		tokens.append_all(quote! {
			{
				use tidos::Component as _;
				let mut page_output = Page::new();
				{
					let page = &mut page_output;
					#input
				}
				page_output
			}
		});
	}
}

/// With the `i18n` feature, `Page::new(lang)` needs a locale, read from the
/// `lang: Lang` route parameter that the caller's route handler is expected
/// to have in scope (`lang.0`).
#[cfg(feature = "i18n")]
impl ToTokens for PageWrapper {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let input = self.component.to_token_stream();
		tokens.append_all(quote! {
			{
				use tidos::Component as _;
				let mut page_output = Page::new(lang.0);
				{
					let page = &mut page_output;
					#input
				}
				page_output
			}
		});
	}
}
