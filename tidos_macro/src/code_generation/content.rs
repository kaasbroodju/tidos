use crate::tokens::HTMLTag;
use crate::tokens::{Content, ControlTag};
use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::token::{Brace, Token};
use syn::Token;

impl ToTokens for Content {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Content::Tag(html_tag) => {
				let is_component = html_tag.tag.chars().next().unwrap().is_ascii_uppercase();
				if is_component {
					tokens.append_all(custom_element_to_tokens(html_tag));
				} else {
					tokens.append_all(native_html_tag_to_tokenstream(html_tag))
				}
			}
			Content::ControlTag(control_tag) => {
				control_tag.to_tokens(tokens);
			}
			Content::Literal(literal) => {
				literal.to_tokens(tokens);
			}
			Content::Expression(expr) => quote!(tidos::sanitize!(#expr)).to_tokens(tokens),
			Content::RawHTMLExpression(expr) => quote!(&#expr).to_tokens(tokens),
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
		attributes.push(quote! { #name: #value })
	}

	let component_name = Ident::new(tag, Span::call_site()).to_token_stream();

	quote! { &#component_name { #( #attributes ),* }.to_render(page) }
}

pub enum TypeOfCommandTag {
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
	},
	If(Vec<TokenTree>),
	Match(Vec<TokenTree>),
}

impl Parse for TypeOfCommandTag {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		input.parse::<Token![#]>()?;

		if input.peek(Token![for]) {
			input.parse::<Token![for]>()?;

			let left_side = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					if tt.to_string().as_str() == "in" {
						return Ok((output, next));
					} else {
						output.push(tt);
						rest = next;
					}
				}

				Err(cursor.error("No `in` found in for loop "))
			})?;

			let right_side = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("Empty right side of `in`."));
				}

				return Ok((output, rest));
			})?;

			// let x = input.parse::<Group>()?;
			return Ok(TypeOfCommandTag::For {
				left_side,
				right_side,
			});
		} else if input.peek(Token![match]) {
			input.parse::<Token![match]>()?;

			let match_content = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("No variable to match against."));
				}

				return Ok((output, rest));
			})?;

			return Ok(TypeOfCommandTag::Match(match_content));
		} else if input.peek(Token![if]) {
			input.parse::<Token![if]>()?;

			let if_content = input.step(|cursor| {
				let mut rest = *cursor;
				let mut output = Vec::new();
				while let Some((tt, next)) = rest.token_tree() {
					output.push(tt);
					rest = next;
				}

				if output.is_empty() {
					return Err(cursor.error("If statement is empty."));
				}

				return Ok((output, rest));
			})?;

			return Ok(TypeOfCommandTag::If(if_content));
		} else {
			panic!("Unknown command tag");
		}
	}
}
