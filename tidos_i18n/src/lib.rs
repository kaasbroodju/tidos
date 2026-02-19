//! Procedural macro crate providing the `i18n!` macro for the
//! [Tidos](https://docs.rs/tidos) SSR framework.
//!
//! This crate is an implementation detail. The macro is re-exported by the
//! `tidos` crate as `tidos::i18n::i18n!` when the `i18n` feature flag is
//! enabled. Depend on `tidos` directly rather than on this crate.
//!
//! For usage examples and full documentation see the
//! [`tidos::i18n` module docs](https://docs.rs/tidos/latest/tidos/i18n/index.html).

mod i18n;

use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::ToTokens;
use crate::i18n::I18n;

/// Looks up a [Fluent](https://projectfluent.org/) translation key and returns the translated `String`.
///
/// Requires the `i18n` feature flag on the `tidos` crate and a call to
/// `enable_i18n!` in `main.rs`.
///
/// The current locale is read from `page.lang`, which is set by the `Lang`
/// Rocket request guard on the route handler.
///
/// # Syntax
///
/// ```text
/// i18n!("message-key")
/// i18n!("message-key", "variable", value, …)
/// ```
///
/// Variables are passed as alternating key-value pairs after the message key.
/// Keys must be string literals; values can be any Rust expression.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::i18n::i18n;
/// use tidos::{view, Component, Page};
///
/// pub struct Greeting;
///
/// impl Component for Greeting {
///     fn to_render(&self, page: &mut Page) -> String {
///         view! {
///             <h1>{i18n!("greeting")}</h1>
///             <p>{i18n!("shared-photos", "userName", "Anne", "photoCount", 3)}</p>
///         }
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as I18n);

    let expanded = input.to_token_stream();

    expanded.into()
}