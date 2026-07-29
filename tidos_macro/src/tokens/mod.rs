//! The AST produced by `parsing/*` and consumed by `code_generation/*`.
//!
//! These types have no behavior of their own beyond simple derives; they're
//! plain data that mirrors the macro's grammar. [`Component`] is the root
//! node (the whole `view!`/`page!`/`head!` body); it holds a sequence of
//! [`Content`] items, each of which is a [`NativeContent`], [`ComponentContent`],
//! [`ControlTag`], or text/raw-HTML/slot variant. Most fields still hold raw
//! `proc_macro2::TokenTree`s rather than parsed Rust expressions/patterns,
//! because the grammar around them (a `for` binding, an `if` condition, a
//! `match` scrutinee/pattern, an attribute's expression) is only re-parsed as
//! real Rust once `code_generation` splices it into a `quote!` template.
//! See the individual `parsing/*` files for why each case defers parsing.

mod content;
pub use content::*;
mod html_tag;
pub use html_tag::*;
mod component;
pub use component::*;
mod control_tag;
pub use control_tag::*;
mod attribute;
pub use attribute::*;
