use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use crate::tokens::{AttributeType, HTMLTag};

impl ToTokens for HTMLTag {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let is_component = self.tag.chars().next().unwrap().is_ascii_uppercase();
		if is_component {
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
	for attribute in &html_tag.attributes {
		if attribute.is_static() {
			static_attributes.push(attribute.to_token_stream());
		} else {
			dynamic_attributes.push(attribute.to_token_stream());
		}
	}
	let has_only_static_attributes = dynamic_attributes.is_empty();

	if html_tag.is_self_closing {
		if has_only_static_attributes {
			quote! {
				concat!("<", #tag, " ", #( #static_attributes, )* "/>")
			}
		} else {
			quote! {
				concat!("<", #tag, " " #( , #static_attributes )* ) #( + #dynamic_attributes )* + "/>"
			}
		}
	} else {
		let mut islands = vec![];
		let mut island = vec![];
		let mut unclean = false;
		for element in &html_tag.children {
			if element.is_static() {
				island.push(element);
				unclean = true;
			} else if unclean {
				islands.push((true, island.clone()));
				unclean = false;
				island = vec![];
				islands.push((false, vec![element]))
			} else {
				islands.push((false, vec![element]))
			}
		}

		if unclean {
			islands.push((true, island.clone()));
		}

		let has_only_static_children = islands.iter().all(|&(x, _)| x);
		let children = islands
			.iter()
			.map(|(is_static, island)| {
				if *is_static {
					quote! { concat!( #( #island ),* ) }
				} else {
					quote! { #( #island )* }
				}
			})
			.collect::<Vec<_>>();

		match (has_only_static_attributes, has_only_static_children) {
			(true, true) => {
				if html_tag.attributes.is_empty() {
					quote! {
						concat!("<", #tag, ">"
							#( , #children )*
							, "</", #tag, ">")
					}
				} else {
					quote! {
						concat!("<", #tag, " " #( , #static_attributes )*
							, ">"
							#( , #children )*
							, "</", #tag, ">")
					}
				}
			}
			(true, false) => {
				if html_tag.attributes.is_empty() {
					quote! {
						concat!("<", #tag, ">")
						#( + #children )*
						+ concat!("</", #tag, ">")
					}
				} else {
					quote! {
						concat!("<", #tag, " " #( , #static_attributes )* , ">")
						#( + #children )*
						+ concat!("</", #tag, ">")
					}
				}
			}
			(false, true) => {
				quote! {
					concat!("<", #tag, " " #( , #static_attributes )* ) #( + #dynamic_attributes )* + concat!(">"
					#( , #children )*
					, "</", #tag, ">")
				}
			}
			(false, false) => {
				quote! {
					concat!("<", #tag, " " #( , #static_attributes )* )
					#( + #dynamic_attributes )*
					+ ">"
					#( + #children )*
					+ concat!("</", #tag, ">")
				}
			}
		}
	}
}

fn custom_element_to_tokens(html_tag: &HTMLTag) -> TokenStream {
	let tag = html_tag.tag.as_str();
	let mut attributes = vec![];
	for attribute in &html_tag.attributes {
		let name = format_ident!("{}", &attribute.name);
		let value = &attribute.value;
		match value {
			None => panic!("Empty attribute"),
			Some(value) => {
				match value {
					AttributeType::Literal(value) => attributes.push(quote! { #name: #value }),
					AttributeType::Group(value) => attributes.push(quote! { #name: #value }),
				}
			}
		}
	}

	let component_name = Ident::new(tag, Span::call_site()).to_token_stream();

	quote! { &#component_name { #( #attributes ),* }.to_render(page) }
}
