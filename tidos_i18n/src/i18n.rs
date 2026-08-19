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
		let result = input.parse::<Literal>().map_err(|error| {
			syn::Error::new(
				error.span(),
				"i18n!: expected a string literal as the message key, e.g. i18n!(\"greeting\")",
			)
		})?;

		let mut params = vec![];
		while !input.is_empty() && input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			let content;
			syn::parenthesized!(content in input);
			let param_key = content.parse::<Literal>().map_err(|error| {
				syn::Error::new(
					error.span(),
					"i18n!: expected a string literal as the parameter name, e.g. (\"userName\", value)",
				)
			})?;
			content.parse::<Token![,]>()?;
			let param_value = content.parse::<TokenTree>().map_err(|error| {
				syn::Error::new(
					error.span(),
					"i18n!: expected a value expression after the parameter name",
				)
			})?;
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
/// If `key` is missing from the negotiated locale's bundle (or present but
/// has no value), the negotiated locale's bundle is *not* consulted further
/// for it: `fluent-resmgr`'s `ResourceManager::get_bundle` only ever loads
/// resources for the first (negotiated) locale and explicitly does not fall
/// back across locales for a missing message. So on a miss, this expands to
/// a second, explicit lookup against the default locale's own bundle before
/// giving up; if that also misses, it panics with the key, both locales, and
/// the resource path that was searched.
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
                    let default_locale = crate::TIDOS_I18N_CONFIGURATION
                        .get_default_locale()
                        .unwrap_or_else(|error| panic!("{error}"));
                    let available_locales = crate::TIDOS_I18N_CONFIGURATION
                        .get_available_locales()
                        .unwrap_or_else(|error| panic!("{error}"));

                    // dynamic part per request
                    let resolved_locales: std::vec::Vec<_> = tidos::i18n::fluent_langneg::negotiate_languages(
                            &[&page.lang],
                            &available_locales,
                            Some(&default_locale),
                            tidos::i18n::fluent_langneg::NegotiationStrategy::Filtering,
                        )
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect();
                    let primary_locale = resolved_locales
                        .first()
                        .cloned()
                        .unwrap_or_else(|| default_locale.clone());

                    let res_path = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                        .join(&crate::TIDOS_I18N_CONFIGURATION.resource_location)
                        .join("{locale}")
                        .join("{res_id}");
                    let res_path_str = res_path.to_str().unwrap_or_else(|| {
                        panic!("Tidos i18n: resource path '{}' is not valid UTF-8", res_path.display())
                    });
                    // ResourceManager should be static
                    let mgr = tidos::i18n::fluent_resmgr::resource_manager::ResourceManager::new(res_path_str.to_string());

                    let bundle = mgr
                        .get_bundle(resolved_locales, crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                        .unwrap_or_else(|errors| panic!(
                            "Tidos i18n: failed to load translation bundle for locale '{primary_locale}': {}",
                            errors.iter().map(|error| error.to_string()).collect::<std::vec::Vec<_>>().join("; "),
                        ));

                    let resolved = bundle
                        .get_message(#key)
                        .and_then(|msg| msg.value())
                        .map(|pattern| {
                            let mut output = String::new();
                            let mut errors = std::vec![];
                            bundle
                                .write_pattern(&mut output, pattern, None, &mut errors)
                                .unwrap_or_else(|error| panic!(
                                    "Tidos i18n: failed to format message '{}' for locale '{primary_locale}': {error}",
                                    #key,
                                ));
                            output
                        });

                    if let Some(output) = resolved {
                        output
                    } else if primary_locale == default_locale {
                        panic!(
                            "Tidos i18n: message '{}' not found in the default locale '{default_locale}' (resource path: '{}')",
                            #key, res_path.display(),
                        );
                    } else {
                        let default_bundle = mgr
                            .get_bundle(std::vec![default_locale.clone()], crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                            .unwrap_or_else(|errors| panic!(
                                "Tidos i18n: failed to load default-locale translation bundle for locale '{default_locale}': {}",
                                errors.iter().map(|error| error.to_string()).collect::<std::vec::Vec<_>>().join("; "),
                            ));

                        let pattern = default_bundle
                            .get_message(#key)
                            .and_then(|msg| msg.value())
                            .unwrap_or_else(|| panic!(
                                "Tidos i18n: message '{}' not found in locale '{primary_locale}' nor in the default locale '{default_locale}' (resource path: '{}')",
                                #key, res_path.display(),
                            ));

                        let mut output = String::new();
                        let mut errors = std::vec![];
                        default_bundle
                            .write_pattern(&mut output, pattern, None, &mut errors)
                            .unwrap_or_else(|error| panic!(
                                "Tidos i18n: failed to format message '{}' for default locale '{default_locale}': {error}",
                                #key,
                            ));
                        output
                    }
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
                    let default_locale = crate::TIDOS_I18N_CONFIGURATION
                        .get_default_locale()
                        .unwrap_or_else(|error| panic!("{error}"));
                    let available_locales = crate::TIDOS_I18N_CONFIGURATION
                        .get_available_locales()
                        .unwrap_or_else(|error| panic!("{error}"));

                    // dynamic part per request
                    let resolved_locales: std::vec::Vec<_> = tidos::i18n::fluent_langneg::negotiate_languages(
                            &[&page.lang],
                            &available_locales,
                            Some(&default_locale),
                            tidos::i18n::fluent_langneg::NegotiationStrategy::Filtering,
                        )
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect();
                    let primary_locale = resolved_locales
                        .first()
                        .cloned()
                        .unwrap_or_else(|| default_locale.clone());

                    let res_path = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                        .join(&crate::TIDOS_I18N_CONFIGURATION.resource_location)
                        .join("{locale}")
                        .join("{res_id}");
                    let res_path_str = res_path.to_str().unwrap_or_else(|| {
                        panic!("Tidos i18n: resource path '{}' is not valid UTF-8", res_path.display())
                    });
                    // ResourceManager should be static
                    let mgr = tidos::i18n::fluent_resmgr::resource_manager::ResourceManager::new(res_path_str.to_string());

                    let bundle = mgr
                        .get_bundle(resolved_locales, crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                        .unwrap_or_else(|errors| panic!(
                            "Tidos i18n: failed to load translation bundle for locale '{primary_locale}': {}",
                            errors.iter().map(|error| error.to_string()).collect::<std::vec::Vec<_>>().join("; "),
                        ));

                    let mut args = tidos::i18n::fluent::FluentArgs::new();
                    #params

                    let resolved = bundle
                        .get_message(#key)
                        .and_then(|msg| msg.value())
                        .map(|pattern| {
                            let mut output = String::new();
                            let mut errors = std::vec![];
                            bundle
                                .write_pattern(&mut output, pattern, Some(&args), &mut errors)
                                .unwrap_or_else(|error| panic!(
                                    "Tidos i18n: failed to format message '{}' for locale '{primary_locale}': {error}",
                                    #key,
                                ));
                            output
                        });

                    if let Some(output) = resolved {
                        output
                    } else if primary_locale == default_locale {
                        panic!(
                            "Tidos i18n: message '{}' not found in the default locale '{default_locale}' (resource path: '{}')",
                            #key, res_path.display(),
                        );
                    } else {
                        let default_bundle = mgr
                            .get_bundle(std::vec![default_locale.clone()], crate::TIDOS_I18N_CONFIGURATION.resources.clone())
                            .unwrap_or_else(|errors| panic!(
                                "Tidos i18n: failed to load default-locale translation bundle for locale '{default_locale}': {}",
                                errors.iter().map(|error| error.to_string()).collect::<std::vec::Vec<_>>().join("; "),
                            ));

                        let pattern = default_bundle
                            .get_message(#key)
                            .and_then(|msg| msg.value())
                            .unwrap_or_else(|| panic!(
                                "Tidos i18n: message '{}' not found in locale '{primary_locale}' nor in the default locale '{default_locale}' (resource path: '{}')",
                                #key, res_path.display(),
                            ));

                        let mut output = String::new();
                        let mut errors = std::vec![];
                        default_bundle
                            .write_pattern(&mut output, pattern, Some(&args), &mut errors)
                            .unwrap_or_else(|error| panic!(
                                "Tidos i18n: failed to format message '{}' for default locale '{default_locale}': {error}",
                                #key,
                            ));
                        output
                    }
                }
		    });
		}
	}
}
