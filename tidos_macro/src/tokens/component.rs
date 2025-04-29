use crate::tokens::Content;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct Component {
	pub children: Vec<Content>,
}
