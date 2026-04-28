use crate::code_generation::component::{flush_flat, process_native_tag, to_push_stmts};
use crate::tokens::{Attribute, Content, ControlTag, HTMLTag};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for HTMLTag {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		if self.is_component() {
			tokens.append_all(custom_element_to_tokens(self));
		} else {
			// Fallback for native tags invoked via Content::to_tokens.
			// The primary path goes through process_native_tag directly.
			let mut flat_args = vec![];
			let mut result = TokenStream::new();
			process_native_tag(self, &mut flat_args, &mut result);
			flush_flat(&mut flat_args, &mut result);
			tokens.append_all(result);
		}
	}
}

fn custom_element_to_tokens(html_tag: &HTMLTag) -> TokenStream {
	let tag = html_tag.tag.as_str();

	let mut attributes = html_tag
		.attributes
		.attributes
		.iter()
		.map(Attribute::to_tokens_custom_element)
		.collect::<Vec<_>>();

	for child in &html_tag.children {
		if let Content::ControlTag(ControlTag::Slot { name, contents }) = child {
			let body = to_push_stmts(contents);
			attributes.push(quote! {
				#name: Box::new(move |page: &mut tidos::Page| { #body })
			});
		}
	}

	let component_name = Ident::new(tag, html_tag.tag_span).to_token_stream();

	let inner = if html_tag.attributes.has_default_flag && attributes.is_empty() {
		quote! { #component_name { ..Default::default() }.to_render(page); }
	} else if html_tag.attributes.has_default_flag && !attributes.is_empty() {
		quote! { #component_name { #( #attributes ),*, ..Default::default() }.to_render(page); }
	} else {
		quote! { #component_name { #( #attributes ),* }.to_render(page); }
	};

	if let Some(closing_span) = html_tag.closing_tag_span {
		let closing_ident = Ident::new(tag, closing_span);
		quote! { { let _: #closing_ident; #inner } }
	} else {
		inner
	}
}
