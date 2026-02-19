use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub struct I18n {
    pub key: Literal,
    pub params: Vec<(Literal, TokenTree)>,
}

impl Parse for I18n {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let result = input.parse::<Literal>().expect("Expected a string literal as key");

        let mut params = vec![];
        while !input.is_empty() && input.peek(Token![,]) {
            _ = input.parse::<Token![,]>()?;
            let param_key = input.parse::<Literal>().expect("Expected param key as string literal");
            _ = input.parse::<Token![,]>()?;
            let param_value = input.parse::<TokenTree>().expect("Expected param value");
            params.push((param_key, param_value));
        }

        Ok(I18n { key: result, params })
    }
}

impl ToTokens for I18n {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let key = &self.key;

        if self.params.is_empty() {
            tokens.append_all(quote! {
                {
                    let res_path = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                        .join(&crate::TIDOS_I18N_CONFIGURATION.resource_location)
                        .join("{locale}")
                        .join("{res_id}");
                    // ResourceManager should be static
                    let mgr = tidos::i18n::fluent_resmgr::resource_manager::ResourceManager::new(res_path.to_str().unwrap().to_string());


                    // dynamic part per request
                    let resolved_locales = tidos::i18n::fluent_langneg::negotiate_languages(
                            &[&page.lang], // This needs to be detemined once upon request of a page
                            &crate::TIDOS_I18N_CONFIGURATION.get_available_locales(), // could be static
                            Some(&crate::TIDOS_I18N_CONFIGURATION.get_default_locale()), // could be determined during compile time
                            tidos::i18n::fluent_langneg::NegotiationStrategy::Filtering,
                        )
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(); // This needs to be detemined once upon request of a page

                    let bundle = mgr
                        .get_bundle(resolved_locales, crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                        .expect("Could not get bundle");

                    let msg = bundle.get_message(#key)
                        .expect("Message doesn't exist.");

                    let mut errors = std::vec![];
                    let pattern = msg.value()
                        .expect("Message has no value.");

                    let mut output = String::new();

                    bundle.write_pattern(&mut output, &pattern, None, &mut errors)
                        .expect("Could not write.");

                    output
                }
		    });
        } else {
            let params = &self.params.iter().fold(TokenStream::new(), |mut acc, (key, value)| {
                acc.append_all(quote! { args.set(#key, #value); });
                acc
            });
            tokens.append_all(quote! {
                {
                    let res_path = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                        .join(&crate::TIDOS_I18N_CONFIGURATION.resource_location)
                        .join("{locale}")
                        .join("{res_id}");
                    // ResourceManager should be static
                    let mgr = tidos::i18n::fluent_resmgr::resource_manager::ResourceManager::new(res_path.to_str().unwrap().to_string());


                    // dynamic part per request
                    let resolved_locales = tidos::i18n::fluent_langneg::negotiate_languages(
                            &[&page.lang], // This needs to be detemined once upon request of a page
                            &crate::TIDOS_I18N_CONFIGURATION.get_available_locales(), // could be static
                            Some(&crate::TIDOS_I18N_CONFIGURATION.get_default_locale()), // could be determined during compile time
                            tidos::i18n::fluent_langneg::NegotiationStrategy::Filtering,
                        )
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect(); // This needs to be detemined once upon request of a page

                    let bundle = mgr
                        .get_bundle(resolved_locales, crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                        .expect("Could not get bundle");

                    let msg = bundle.get_message(#key)
                        .expect("Message doesn't exist.");

                    let mut args = tidos::i18n::fluent::FluentArgs::new();
                    #params

                    let mut errors = std::vec![];
                    let pattern = msg.value()
                        .expect("Message has no value.");

                    let mut output = String::new();

                    bundle.write_pattern(&mut output, &pattern, Some(&args), &mut errors)
                        .expect("Could not write.");

                    output
                }
		    });
        }
    }
}