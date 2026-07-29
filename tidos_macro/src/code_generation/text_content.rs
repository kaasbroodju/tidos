use crate::code_generation::component::{FlatArg, ToFlatArgs};
use crate::tokens::TextContent;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::LitStr;

impl TextContent {
	pub(crate) fn sanitize_literal(literal: Literal) -> Literal {
		let input = &literal.to_string();
		let input = &input[1..input.len() - 1];

		if !input.contains(['&', '<', '>', '"', '\'']) {
			return literal;
		}

		let mut result = String::new();

		for c in input.chars() {
			match c {
				'&' => result.push_str("&amp;"),
				'<' => result.push_str("&lt;"),
				'>' => result.push_str("&gt;"),
				'"' => result.push_str("&quot;"),
				'\'' => result.push_str("&#x27;"),
				_ => result.push(c),
			}
		}

		Literal::string(result.as_str())
	}

	pub(crate) fn sanitize_lit_str(literal: LitStr) -> String {
		let input = literal.value();

		if !input.contains(['&', '<', '>', '"', '\'']) {
			return input;
		}

		let mut result = String::new();

		for c in input.chars() {
			match c {
				'&' => result.push_str("&amp;"),
				'<' => result.push_str("&lt;"),
				'>' => result.push_str("&gt;"),
				'"' => result.push_str("&quot;"),
				'\'' => result.push_str("&#x27;"),
				_ => result.push(c),
			}
		}

		result
	}
}

/// Sanitized (HTML-escaped) rendering of a [`TextContent`], used for `{...}`
/// text expressions.
impl ToFlatArgs for TextContent {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, _result: &mut TokenStream) {
		match self {
			TextContent::Literal(literal) => {
				let lit = TextContent::sanitize_literal(literal.clone());
				flat_args.push(FlatArg::Literal(literal_string_value(&lit)));
			}
			TextContent::Formatted(literal, contents) => {
				let lit = TextContent::sanitize_literal(literal.clone());
				flat_args.push(FlatArg::Expr(
					quote! { tidos::sanitize!(format!(#lit #( , #( #contents )* )* )) },
				));
			}
			TextContent::Expression(expr) => {
				flat_args.push(FlatArg::Expr(quote! { tidos::sanitize!( #( #expr )* ) }));
			}
		}
	}
}

/// Marks a [`TextContent`] as raw (unsanitized), used for `@html{...}`
/// expressions, where the caller is asserting the content is already-trusted
/// HTML. A separate wrapper type rather than a second inherent method, since
/// a type can only implement a given trait once.
pub(crate) struct RawTextContent<'a>(pub &'a TextContent);

impl ToFlatArgs for RawTextContent<'_> {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, _result: &mut TokenStream) {
		match self.0 {
			TextContent::Literal(literal) => {
				flat_args.push(FlatArg::Literal(literal_string_value(literal)));
			}
			TextContent::Formatted(literal, contents) => {
				flat_args.push(FlatArg::Expr(
					quote! { format!(#literal #( , #( #contents )* )* ) },
				));
			}
			TextContent::Expression(expr) => {
				flat_args.push(FlatArg::Expr(quote! { #( #expr )* }));
			}
		}
	}
}

/// Extract the string value out of a literal that is known to be a plain
/// string literal (e.g. a sanitized text literal). Uses the same
/// quote-stripping convention as [`TextContent::sanitize_literal`].
pub(crate) fn literal_string_value(literal: &Literal) -> String {
	let s = literal.to_string();
	s[1..s.len() - 1].to_string()
}
