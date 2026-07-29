use crate::tokens::{Component, Content};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		to_push_stmts(&self.children).to_tokens(tokens);
	}
}

/// A single pending contribution to a `combine!` call.
///
/// `Literal` values are known as plain Rust strings at macro-expansion time,
/// so adjacent ones can be merged into one string right here — no need to
/// defer that merging to `concat!` in the generated code. `Expr` is anything
/// that can only be evaluated at runtime (control flow, sanitized/formatted
/// values, attribute value literals of unknown literal kind).
pub(crate) enum FlatArg {
	Literal(String),
	Expr(TokenStream),
}

/// Converts a slice of `Content` into a `TokenStream` of push statements
/// that write into a `page: &mut Page` variable in scope.
///
/// Adjacent static literals are merged into a single Rust `String` here, at
/// macro-expansion time, before being handed to one `combine!(page, ...)`
/// call — see [`flush_flat`]. This keeps the number of arguments `combine!`
/// itself ever has to process small and bounded by the number of *dynamic*
/// pieces in a template, regardless of how much static markup surrounds
/// them, which is what let arbitrarily large static templates blow the
/// macro recursion limit before.
pub(crate) fn to_push_stmts(contents: &[Content]) -> TokenStream {
	let mut result = TokenStream::new();
	let mut flat_args: Vec<FlatArg> = vec![];

	for child in contents {
		child.to_flat_args(&mut flat_args, &mut result);
	}
	flush_flat(&mut flat_args, &mut result);
	result
}

/// Emit `tidos::combine!(page, arg1, arg2, ...)` from the accumulated args and
/// clear the buffer. Adjacent `FlatArg::Literal`s are merged into a single
/// string argument first. Does nothing if the buffer is empty.
pub(crate) fn flush_flat(flat_args: &mut Vec<FlatArg>, result: &mut TokenStream) {
	if flat_args.is_empty() {
		return;
	}

	let mut args: Vec<TokenStream> = vec![];
	let mut pending = String::new();

	for arg in std::mem::take(flat_args) {
		match arg {
			FlatArg::Literal(lit) => pending.push_str(&lit),
			FlatArg::Expr(expr) => {
				if !pending.is_empty() {
					args.push(quote! { #pending });
					pending = String::new();
				}
				args.push(expr);
			}
		}
	}
	if !pending.is_empty() {
		args.push(quote! { #pending });
	}

	result.append_all(quote! { tidos::combine!(page, #(#args),*); });
}

/// Like [`syn::parse::Parse`], but for the code-generation side: instead of
/// turning tokens into a `Self`, this turns a `Self` into its contribution to
/// the surrounding `combine!` call. Implementors push their static pieces
/// onto `flat_args` (merged later by [`flush_flat`]) and/or flush + write
/// directly into `result` for anything that can't be a flat argument at all
/// (control flow, toggle attributes).
pub(crate) trait ToFlatArgs {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, result: &mut TokenStream);
}
