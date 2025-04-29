use crate::tokens::{Component, Content};
use syn::parse::{Parse, ParseStream};

impl Parse for Component {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		if input.is_empty() {
			return Ok(Component {
				children: Vec::new(),
			});
		}

		let mut children: Vec<Content> = Vec::new();

		while !input.is_empty() {
			let child = input.parse::<Content>()?;
			children.push(child);
		}

		Ok(Component { children })
	}
}
