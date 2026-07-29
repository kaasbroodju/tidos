use crate::code_generation::component::{to_push_stmts, FlatArg, ToFlatArgs};
use crate::tokens::{Attribute, ComponentContent, Content, ControlTag, NativeContent};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

/// Expand a native HTML tag inline — tag open, attributes, children, tag close
/// are all added to `flat_args` so [`flush_flat`] can merge adjacent literals
/// across tag boundaries.
impl ToFlatArgs for NativeContent {
	fn to_flat_args(&self, flat_args: &mut Vec<FlatArg>, result: &mut TokenStream) {
		let tag_name = self.tag.as_str();

		flat_args.push(FlatArg::Literal("<".to_string()));
		flat_args.push(FlatArg::Literal(tag_name.to_string()));

		if !self.attributes.is_empty() {
			flat_args.push(FlatArg::Literal(" ".to_string()));
		}

		for attr in &self.attributes {
			attr.to_flat_args(flat_args, result);
		}

		if self.is_self_closing {
			flat_args.push(FlatArg::Literal("/>".to_string()));
		} else {
			flat_args.push(FlatArg::Literal(">".to_string()));
			for child in &self.children {
				child.to_flat_args(flat_args, result);
			}
			flat_args.push(FlatArg::Literal("</".to_string()));
			flat_args.push(FlatArg::Literal(tag_name.to_string()));
			flat_args.push(FlatArg::Literal(">".to_string()));
		}
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
