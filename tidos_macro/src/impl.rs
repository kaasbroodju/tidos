extern crate proc_macro;

use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub enum ControlTag {
	// {#if ... } ... {:else if ... } ... {:else} ... {/if}
	IfChain {
		if_statement: Vec<TokenTree>,
		if_content: Vec<Content>,
		if_else_chain: Vec<(Vec<TokenTree>, Vec<Content>)>,
		else_content: Option<Vec<Content>>,
	},
	// {#for ... in ... } ... {/for}
	For {
		left_side: Vec<TokenTree>,
		right_side: Vec<TokenTree>,
		contents: Vec<Content>,
	},
	// {#match ... } ... {/match}
	Match {
		match_statement: Vec<TokenTree>,
		cases: Vec<(Vec<TokenTree>, Vec<Content>)>,
	},
}

impl ToTokens for ControlTag {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			ControlTag::IfChain {
				if_statement,
				if_content,
				if_else_chain,
				else_content,
			} => {
				ControlTag::to_tokens_if_chain(
					if_statement,
					if_content,
					if_else_chain,
					else_content,
					tokens,
				);
			}
			ControlTag::For {
				left_side,
				right_side,
				contents,
			} => {
				ControlTag::to_tokens_for_loop(left_side, right_side, contents, tokens);
			}
			ControlTag::Match {
				match_statement,
				cases,
			} => {
				ControlTag::to_tokens_match(match_statement, cases, tokens);
			}
		}
	}
}

impl ControlTag {
	fn to_tokens_for_loop(
		left_side: &Vec<TokenTree>,
		right_side: &Vec<TokenTree>,
		contents: &Vec<Content>,
		tokens: &mut TokenStream,
	) {
		let tokens_children = contents
			.iter()
			.fold(&mut TokenStream::new(), |acc, child| {
				child.to_tokens(acc);
				acc
			})
			.to_owned();

		let output = quote! {

			&( #( #right_side )* ).into_iter().fold(String::new(), |acc, ( #( #left_side )* )| { acc + #tokens_children })
		};

		tokens.append_all(output);
	}

	fn to_tokens_if_chain(
		if_statement: &Vec<TokenTree>,
		if_content: &Vec<Content>,
		if_else_chain: &Vec<(Vec<TokenTree>, Vec<Content>)>,
		else_content: &Option<Vec<Content>>,
		tokens: &mut TokenStream,
	) {
		let if_content_tokens = if_content
			.iter()
			.fold(&mut TokenStream::new(), |acc, child| {
				child.to_tokens(acc);
				acc
			})
			.to_owned();

		let if_else_chain_tokens = if_else_chain
			.iter()
			.fold(&mut TokenStream::new(), |acc, (statement, contents)| {
				let chain_contents_tokens = contents
					.iter()
					.fold(&mut TokenStream::new(), |acc, child| {
						child.to_tokens(acc);
						acc
					})
					.to_owned();

				let chain = quote! {
					else if #( #statement )* { String::new() + #chain_contents_tokens }
				};

				chain.to_tokens(acc);
				acc
			})
			.to_owned();

		let output = if let Some(else_content) = else_content {
			let else_content_tokens = else_content
				.iter()
				.fold(&mut TokenStream::new(), |acc, child| {
					child.to_tokens(acc);
					acc
				})
				.to_owned();

			quote! {
				&if #( #if_statement )* { String::new() + #if_content_tokens } #if_else_chain_tokens else { String::new() + #else_content_tokens }
			}
		} else {
			quote! {
				&if #( #if_statement )* { String::new() + #if_content_tokens } #if_else_chain_tokens else { String::new() }
			}
		};

		tokens.append_all(output);
	}

	fn to_tokens_match(
		match_statement: &Vec<TokenTree>,
		cases: &Vec<(Vec<TokenTree>, Vec<Content>)>,
		tokens: &mut TokenStream,
	) {
		let cases = cases
			.iter()
			.map(|(case_statement, case_content)| {
				// todo static islands
				quote! {
					#( #case_statement )* => {
						String::new() + #( #case_content )+*
					}
				}
			})
			.collect::<Vec<_>>();

		let output = quote! {
			&match #( #match_statement )* {
				#( #cases )*
			}
		};

		tokens.append_all(output);
	}
}

#[derive(Debug)]
pub enum Content {
	// <p>...</p>
	Tag(HTMLTag),

	// {#if x > 10} ... {/if}
	// {#for x in numbers} ... {/for}
	// {#match x} ... {/match}
	ControlTag(ControlTag),

	// // <Custom></Custom>
	// Custom,

	// // <tidos:self></tidos:self>
	// Instruction,

	// text
	Literal(String),

	// expression <p>{ format!("Hello {}", name) }</p>
	Expression(Group),

	// <p>@html{"<p>potential danger"}</p>
	RawHTMLExpression(Group),
}

impl Content {
	fn is_static(&self) -> bool {
		match self {
			Content::Tag(element) => element.is_static(),
			Content::ControlTag(_) => false,
			Content::Literal(_) => true,
			Content::Expression(_) => false,
			Content::RawHTMLExpression(_) => false,
		}
	}
}

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
				concat!("<", #tag #(, " ", #static_attributes)* , " />")
			}
		} else {
			quote! {
				concat!("<", #tag #(, " ", #static_attributes)* ) #(+ " " + #dynamic_attributes)* + " />"
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
				quote! {
					concat!("<", #tag #(, " ", #static_attributes)*
						, ">"
						#(, #children)*
						, "</", #tag, ">")
				}
			}
			(true, false) => {
				quote! {
					concat!("<", #tag #(, " ", #static_attributes)* , ">")
					#( + #children )*
					+ concat!("</", #tag, ">")
				}
			}
			(false, true) => {
				quote! {
					concat!("<", #tag #(, " ", #static_attributes)* )
					#( + " " + #dynamic_attributes )*
					+ concat!(">" #(, #children)* , "</", #tag, ">")
				}
			}
			(false, false) => {
				quote! {
					concat!("<", #tag #(, " ", #static_attributes)* )
					#( + " " + #dynamic_attributes )*
					+ ">"
					#( + #children )*
					+ concat!("</", #tag, ">")
				}
			}
		}
	}
}

fn custom_element_to_tokens(
	html_tag: &HTMLTag
) -> TokenStream {
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

#[derive(Debug)]
pub struct HTMLTag {
	pub tag: String,
	pub attributes: Vec<Attribute>,
	pub children: Vec<Content>,
	pub is_self_closing: bool,
}

impl HTMLTag {
	fn is_static(&self) -> bool {
		let is_component = self.tag.chars().next().unwrap().is_ascii_uppercase();
		if is_component {
			return false;
		}

		let has_only_static_attributes = self
			.attributes
			.iter()
			.all(|attribute| attribute.is_static());

		let has_only_static_children =
			self.children.iter().all(|child| child.is_static());

		has_only_static_attributes && has_only_static_children
	}
}

#[derive(Debug)]
pub struct Attribute {
	pub is_toggle_attribute: bool,
	pub name: String,
	pub value: Option<TokenTree>,
}

impl Attribute {
	fn is_static(&self) -> bool {
		if self.is_toggle_attribute {
			return false;
		}
		match &self.value {
			None => true,
			Some(token) => {
				match token {
					TokenTree::Group(_) => false,
					// todo identifier of scoped css is static
					TokenTree::Literal(_) => true,
					_ => {
						panic!("Tidos macro error: expected group or ident")
					}
				}
			}
		}
	}
}

impl ToTokens for Attribute {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match (&self.value, &self.is_toggle_attribute) {
			// :disabled
			(None, true) => {
				let ident = format_ident!("{}", &self.name);
				let attribute_name = &self.name.to_string();

				tokens.append_all(quote! {
					if #ident { #attribute_name } else { "" }
				});
			}
			// disabled
			(None, false) => {
				let attribute_name = &self.name.to_string();
				tokens.append_all(quote! {
					#attribute_name
				});
			}
			// :disabled={ true }
			(Some(value), true) => {
				let attribute_name = &self.name.to_string();
				tokens.append_all(quote! {
					if #value { #attribute_name } else { "" }
				});
			}
			// class="wrapper" or value={ person.name }
			(Some(value), false) => {
				let attribute_name = &(&self.name)
					.clone()
					.to_string()
					.trim_start_matches("r#")
					.to_string();

				match value {
					TokenTree::Group(group) => {
						tokens.append_all(quote! {
							concat!(#attribute_name, "=\"") + &tidos::sanitize!(#group) + "\""

							//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
						});
					}
					TokenTree::Literal(literal) => {
						tokens.append_all(quote! {
							concat!(#attribute_name, "=\"", #literal, "\"")

							//format!("{}=\"{}\"", #attribute_name, tidos::sanitize!(#value.to_string()) )
						});
					}
					_ => {
						panic!("Tidos macro error: expected group or ident")
					}
				}
			}
		}
	}
}

#[derive(Debug)]
pub struct Component {
	pub children: Vec<Content>,
}

impl ToTokens for Component {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let children = &self.children;
		let binding = "{}".repeat(children.len());
		let format_string = binding.as_str();
		let x = { String::from(String::new()) + &String::from("Hello") };
		tokens.append_all(quote! {
			String::new()
				#(
					+ #children
				)*

		});
	}
}

pub struct PageWrapper {
	component: Component,
}

impl Parse for PageWrapper {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let component = Component::parse(input)?;
		Ok(PageWrapper { component })
	}
}

impl ToTokens for PageWrapper {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let input = self.component.to_token_stream();
		tokens.append_all(quote! {
			{
				let mut page_output = Page::new();
				page_output.template = {
					let page = &mut page_output;
					#input
				};
				page_output
			}

		});
	}
}
