//! Parses the top-level body of a `view!`, `page!`, or `head!` invocation:
//! a flat sequence of [`Content`] items with no wrapping syntax of its own.

use crate::tokens::{Component, Content};
use syn::parse::{Parse, ParseStream};

/// Parses input as a sequence of [`Content`] items until the stream is
/// exhausted. This is the macro entry point: `view!`/`page!`/`head!` all
/// parse their input as a `Component`, then hand every child to
/// `Content::parse` in turn.
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
