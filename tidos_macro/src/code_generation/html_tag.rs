use crate::code_generation::component::{flush_flat, process_native_tag, to_push_stmts};
use crate::tokens::{Attribute, ComponentContent, Content, ControlTag, NativeContent};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for NativeContent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		// Fallback for native tags invoked via Content::to_tokens.
		// The primary path goes through process_native_tag directly.
		let mut flat_args = vec![];
		let mut result = TokenStream::new();
		process_native_tag(self, &mut flat_args, &mut result);
		flush_flat(&mut flat_args, &mut result);
		tokens.append_all(result);
	}
}

impl ToTokens for ComponentContent {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let tag = self.tag.as_str();

		let mut attributes = self
			.attributes
			.iter()
			.map(Attribute::to_tokens_custom_element)
			.collect::<Vec<_>>();

		if matches!(
			self.children.first(),
			Some(Content::ControlTag(ControlTag::Slot { .. }))
		) {
			for child in &self.children {
				if let Content::ControlTag(ControlTag::Slot { name, contents }) = child {
					let body = to_push_stmts(contents);
					attributes
						.push(quote! { #name: Box::new(move |page: &mut tidos::Page| { #body }) });
				}
			}
		} else if !self.children.is_empty() {
			let body = to_push_stmts(&self.children);
			attributes.push(quote! {
				0: Box::new(move |page: &mut tidos::Page| { #body })
			});
		}

		let component_name = Ident::new(tag, self.tag_span).to_token_stream();

		let inner = if self.has_default_flag && attributes.is_empty() {
			quote! { #component_name { ..Default::default() }.to_render(page); }
		} else if self.has_default_flag && !attributes.is_empty() {
			quote! { #component_name { #( #attributes ),*, ..Default::default() }.to_render(page); }
		} else {
			quote! { #component_name { #( #attributes ),* }.to_render(page); }
		};

		let content = if let Some(closing_span) = self.closing_tag_span {
			let closing_ident = Ident::new(tag, closing_span);
			quote! { { let _: #closing_ident; #inner } }
		} else {
			inner
		};
		tokens.append_all(content);
	}
}
