mod i18n;


use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::ToTokens;
use crate::i18n::I18n;

#[allow(clippy::all)]
#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as I18n);

    let expanded = input.to_token_stream();

    expanded.into()
}
