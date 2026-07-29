//! Dispatches each [`Content`] variant to the code generator for its
//! underlying type (native tag, component, control tag, slot render, text).
//! This is the code-generation-side counterpart to `parsing::content`: where
//! that file decides what to parse next, this one decides how to render what
//! was parsed.

use crate::code_generation::component::{flush_flat, FlatArg, ToFlatArgs};
use crate::code_generation::text_content::RawTextContent;
use crate::tokens::Content;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

/// Process a single `Content` item: either add its value as args to `flat_args`
/// (if it's a literal or expression), or flush `flat_args` and emit a statement
/// (if it's a control tag, slot render, or component).
impl ToFlatArgs for Content {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, result: &mut TokenStream) {
		match self {
			Content::NativeElement(content) => {
				content.to_flat_args(flat_args, result);
			}
			Content::CustomComponent(content) => {
				flush_flat(flat_args, result);
				content.to_tokens(result);
			}
			Content::ControlTag(ct) => {
				flush_flat(flat_args, result);
				ct.to_tokens(result);
			}
			Content::SlotRender(expr) => {
				flush_flat(flat_args, result);
				result.append_all(quote! { (#( #expr )*)(page); });
			}
			Content::Text(text) => {
				text.to_flat_args(flat_args, result);
			}
			Content::RawHTMLExpression(text) => {
				RawTextContent(text).to_flat_args(flat_args, result);
			}
		}
	}
}
