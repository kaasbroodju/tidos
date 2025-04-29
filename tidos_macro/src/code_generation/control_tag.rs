use crate::tokens::{Content, ControlTag};
use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, ToTokens, TokenStreamExt};

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
