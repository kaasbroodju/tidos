//! The root AST node for a `view!`/`page!`/`head!` invocation.

use crate::tokens::Content;

/// The whole body of a `view!`, `page!`, or `head!` call: just a flat list of
/// top-level [`Content`] items, produced by `parsing::component`.
#[derive(Debug)]
pub struct Component {
	pub children: Vec<Content>,
}
