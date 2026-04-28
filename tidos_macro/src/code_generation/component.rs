use crate::tokens::{Attribute, AttributeType, Component, Content, HTMLTag, TextContent};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		to_push_stmts(&self.children).to_tokens(tokens);
	}
}

/// Converts a slice of `Content` into a `TokenStream` of push statements
/// that write into a `page: &mut Page` variable in scope.
///
/// Adjacent static literals are grouped into a single `combine!(page, ...)` call
/// so the Rust compiler can merge them at compile time via `concat!`.
/// Dynamic content (control tags, slot renders, component tags) is flushed as
/// its own statement and acts as a boundary between literal groups.
pub(crate) fn to_push_stmts(contents: &[Content]) -> TokenStream {
	let mut result = TokenStream::new();
	let mut flat_args: Vec<TokenStream> = vec![];

	for child in contents {
		process_content(child, &mut flat_args, &mut result);
	}
	flush_flat(&mut flat_args, &mut result);
	result
}

/// Emit `tidos::combine!(page, arg1, arg2, ...)` from the accumulated args and
/// clear the buffer. Does nothing if the buffer is empty.
pub(crate) fn flush_flat(flat_args: &mut Vec<TokenStream>, result: &mut TokenStream) {
	if flat_args.is_empty() {
		return;
	}
	let args: Vec<TokenStream> = std::mem::take(flat_args);
	// let args: Vec<TokenStream> = flat_args.drain(..).collect();
	result.append_all(quote! { tidos::combine!(page, #(#args),*); });
}

/// Process a single `Content` item: either add its value as args to `flat_args`
/// (if it's a literal or expression), or flush `flat_args` and emit a statement
/// (if it's a control tag, slot render, or component).
fn process_content(child: &Content, flat_args: &mut Vec<TokenStream>, result: &mut TokenStream) {
	match child {
		Content::ControlTag(ct) => {
			flush_flat(flat_args, result);
			ct.to_tokens(result);
		}
		Content::SlotRender(expr) => {
			flush_flat(flat_args, result);
			result.append_all(quote! { (#( #expr )*)(page); });
		}
		Content::Tag(tag) if tag.is_component() => {
			flush_flat(flat_args, result);
			tag.to_tokens(result);
		}
		Content::Tag(tag) => {
			process_native_tag(tag, flat_args, result);
		}
		Content::Text(text) => {
			text_to_args(text, flat_args);
		}
		Content::RawHTMLExpression(text) => {
			raw_to_args(text, flat_args);
		}
	}
}

/// Expand a native HTML tag inline — tag open, attributes, children, tag close
/// are all added to `flat_args` so the caller's `combine!` can merge adjacent
/// literals across tag boundaries.
pub(crate) fn process_native_tag(
	tag: &HTMLTag,
	flat_args: &mut Vec<TokenStream>,
	result: &mut TokenStream,
) {
	let tag_name = tag.tag.as_str();

	flat_args.push(quote! { "<" });
	flat_args.push(quote! { #tag_name });

	if !tag.attributes.attributes.is_empty() {
		flat_args.push(quote! { " " });
	}

	for attr in &tag.attributes.attributes {
		process_attribute(attr, flat_args, result);
	}

	if tag.is_self_closing {
		flat_args.push(quote! { "/>" });
	} else {
		flat_args.push(quote! { ">" });
		for child in &tag.children {
			process_content(child, flat_args, result);
		}
		flat_args.push(quote! { "</" });
		flat_args.push(quote! { #tag_name });
		flat_args.push(quote! { ">" });
	}
}

/// Contribute an attribute's value(s) to `flat_args`.
/// Toggle attributes (ImplicitToggle / ExplicitToggle) are always boundaries:
/// they flush `flat_args` and emit a direct `if … { page.push_static(…); }`.
fn process_attribute(attr: &Attribute, flat_args: &mut Vec<TokenStream>, result: &mut TokenStream) {
	let name = attr.name.as_str();
	let name_trimmed = name.trim_start_matches("r#");

	match &attr.attribute_type {
		AttributeType::Constant => {
			flat_args.push(quote! { #name_trimmed });
			flat_args.push(quote! { " " });
		}
		AttributeType::ConstantLiteral { literal } => {
			flat_args.push(quote! { #name_trimmed });
			flat_args.push(quote! { "=\"" });
			flat_args.push(quote! { #literal });
			flat_args.push(quote! { "\" " });
		}
		AttributeType::ConstantGroup { contents } => {
			flat_args.push(quote! { #name_trimmed });
			flat_args.push(quote! { "=\"" });
			flat_args.push(quote! { tidos::sanitize!(#contents) });
			flat_args.push(quote! { "\" " });
		}
		AttributeType::ImplicitToggle => {
			flush_flat(flat_args, result);
			let ident = format_ident!("{}", name);
			result.append_all(quote! {
				if #ident { page.push_static(concat!(#name, " ")); }
			});
		}
		AttributeType::ExplicitToggle { value } => {
			flush_flat(flat_args, result);
			result.append_all(quote! {
				if #value { page.push_static(concat!(#name_trimmed, " ")); }
			});
		}
	}
}

fn text_to_args(text: &TextContent, flat_args: &mut Vec<TokenStream>) {
	match text {
		TextContent::Literal(literal) => {
			let lit = TextContent::sanitize_literal(literal.clone());
			flat_args.push(quote! { #lit });
		}
		TextContent::Formatted(literal, contents) => {
			let lit = TextContent::sanitize_literal(literal.clone());
			flat_args.push(quote! { format!(#lit #( , #( #contents )* )* ) });
		}
		TextContent::Expression(expr) => {
			flat_args.push(quote! { tidos::sanitize!( #( #expr )* ) });
		}
	}
}

fn raw_to_args(text: &TextContent, flat_args: &mut Vec<TokenStream>) {
	match text {
		TextContent::Literal(literal) => {
			flat_args.push(quote! { #literal });
		}
		TextContent::Formatted(literal, contents) => {
			flat_args.push(quote! { format!(#literal #( , #( #contents )* )* ) });
		}
		TextContent::Expression(expr) => {
			flat_args.push(quote! { ::std::string::String::from(#( #expr )*) });
		}
	}
}
