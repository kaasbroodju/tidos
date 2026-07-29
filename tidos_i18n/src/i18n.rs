//! Parsing and code generation for the `i18n!` macro, in one file since both
//! sides are small. Unlike `tidos_macro`, there's no separate AST module:
//! [`I18n`] is parsed directly from the macro input and, at the same time,
//! is the type whose `ToTokens` impl generates the Fluent lookup code.

use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::Token;

/// The parsed arguments to `i18n!("key", ("var", value), ...)`: the message
/// key plus zero or more `(name, value)` Fluent variable pairs.
pub struct I18n {
	pub key: Literal,
	pub params: Vec<(Literal, TokenTree)>,
}

/// Parses `"key"` followed by zero or more `, ("param-name", value)` pairs.
impl Parse for I18n {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let result = input
			.parse::<Literal>()
			.expect("Expected a string literal as key");

		let mut params = vec![];
		while !input.is_empty() && input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			let content;
			syn::parenthesized!(content in input);
			let param_key = content
				.parse::<Literal>()
				.expect("Expected param key as string literal");
			content.parse::<Token![,]>()?;
			let param_value = content.parse::<TokenTree>().expect("Expected param value");
			params.push((param_key, param_value));
		}

		Ok(I18n {
			key: result,
			params,
		})
	}
}

/// Expands to a block that loads the Fluent resource bundle, resolves the
/// caller's locale, looks up `key`, and writes the resolved message to a
/// `String`. Requires a `page` binding in scope (relies on `page.lang` for
/// locale negotiation), just like `scoped_css!`/`head!` in `tidos_macro` rely
/// on `page` being bound by `view!`/`page!`.
///
/// The with-params and without-params cases are generated as two separate
/// (near-duplicate) blocks rather than one block with an `Option<&FluentArgs>`
/// built conditionally, so that the no-params call site never has to
/// construct an empty `FluentArgs` at all.
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
			let params = &self
				.params
				.iter()
				.fold(TokenStream::new(), |mut acc, (key, value)| {
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
