use crate::tokens::{Attribute, Content, ControlTag, HTMLTag, IsStatic};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

impl ToTokens for HTMLTag {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		if self.is_component() {
			tokens.append_all(custom_element_to_tokens(self));
		} else {
			tokens.append_all(native_html_tag_to_tokenstream(self))
		}
	}
}

fn native_html_tag_to_tokenstream(html_tag: &HTMLTag) -> TokenStream {
	let tag = html_tag.tag.as_str();

	let mut static_attributes = vec![];
	let mut dynamic_attributes = vec![];
	let attributes = html_tag.attributes.attributes.iter();
	for attribute in attributes {
		if attribute.is_static() {
			static_attributes.push(attribute);
		} else {
			dynamic_attributes.push(attribute);
		}
	}

	if html_tag.is_self_closing {
		quote! {
			"<", #tag, " " #( , #static_attributes )* #( , #dynamic_attributes )* , "/>"
		}
	} else {
		let children = &html_tag.children;
		if static_attributes.is_empty() && dynamic_attributes.is_empty() {
			quote! {
				"<", #tag, ">"
				#( , #children )*
				, "</", #tag, ">"
			}
		} else {
			quote! {
				"<", #tag, " " #( , #static_attributes )* #( , #dynamic_attributes )* , ">"
				#( , #children )*
				, "</", #tag, ">"
			}
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
			attributes.push(quote! { #name: tidos::combine!(String::new() #( , #contents )* ) });
		}
	}

	let component_name = Ident::new(tag, html_tag.tag_span).to_token_stream();

	let inner = if html_tag.attributes.has_default_flag && attributes.is_empty() {
		quote! { #component_name { ..Default::default() }.to_render(page) }
	} else if html_tag.attributes.has_default_flag && !attributes.is_empty() {
		quote! { #component_name { #( #attributes ),*, ..Default::default() }.to_render(page) }
	} else {
		quote! { #component_name { #( #attributes ),* }.to_render(page) }
	};

	if let Some(closing_span) = html_tag.closing_tag_span {
		let closing_ident = Ident::new(tag, closing_span);
		quote! { { let _: #closing_ident; #inner } }
	} else {
		inner
	}
}
