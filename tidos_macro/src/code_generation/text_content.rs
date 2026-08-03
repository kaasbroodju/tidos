use crate::code_generation::component::{FlatArg, ToFlatArgs};
use crate::tokens::TextContent;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::LitStr;

impl TextContent {
	/// Escapes a text-content literal for direct, unescaped embedding into the
	/// generated HTML output. Parses `literal` as a [`syn::LitStr`] and reads
	/// its *cooked* value (`LitStr::value()`), not the raw token source text:
	/// a naive "strip the surrounding quotes" approach would leave Rust's own
	/// escape sequences (`\n`, `\t`, `\"`, ...) un-cooked, e.g. `{"a\nb"}`
	/// would render the two characters `\`+`n` instead of an actual newline.
	pub(crate) fn sanitize_literal(literal: Literal) -> Literal {
		let syn::Lit::Str(lit_str) = syn::Lit::new(literal.clone()) else {
			unreachable!("TextContent literals are always string literals")
		};
		let input = lit_str.value();

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
				// Not pre-escaped via `sanitize_literal` like the `Literal`
				// arm above: the whole `format!` result, literal part
				// included, already goes through the runtime `sanitize!`
				// below. Escaping the literal part here too would
				// double-escape it (e.g. `&` -> `&amp;` -> `&amp;amp;`).
				flat_args.push(FlatArg::Expr(
					quote! { tidos::sanitize!(format!(#literal, #contents )) },
				));
			}
			TextContent::Expression(expr) => {
				flat_args.push(FlatArg::Expr(quote! { tidos::sanitize!( #expr ) }));
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
				flat_args.push(FlatArg::Expr(quote! { format!(#literal, #contents ) }));
			}
			TextContent::Expression(expr) => {
				flat_args.push(FlatArg::Expr(quote! { #expr }));
			}
		}
	}
}

/// Extract the string value out of a literal that is known to be a plain
/// string literal (e.g. a sanitized text literal). Parses as a
/// [`syn::LitStr`] and reads the *cooked* value, same as
/// [`TextContent::sanitize_literal`] and for the same reason: naively
/// stripping the surrounding quotes off the raw token text would leave
/// Rust's own escape sequences (`\n`, `\t`, `\"`, ...) un-cooked.
pub(crate) fn literal_string_value(literal: &Literal) -> String {
	let syn::Lit::Str(lit_str) = syn::Lit::new(literal.clone()) else {
		unreachable!("TextContent literals are always string literals")
	};
	lit_str.value()
}
