//! Recursive-descent parsing: turns the raw macro input `TokenStream` into the
//! `tokens::*` AST.
//!
//! Every file here implements `syn::parse::Parse` for one AST node type from
//! `crate::tokens`. `content.rs` is the entry point most other parsers recurse
//! back into: `Content::parse` looks at the next tokens and dispatches to
//! `html_tag.rs` (native elements / components), `control_tag.rs` (`{#for}`,
//! `{#if}`, `{#match}`, `{#slot:name}`), or `text_content.rs` (`{expr}`,
//! `@html{...}`). `attribute.rs` and `utils.rs` are shared helpers used by the
//! tag parsers. The resulting AST is handed off to `code_generation/*`, which
//! turns it into the final `quote!`-generated Rust code.
mod attribute;
mod component;
mod content;
mod control_tag;
mod html_tag;
mod text_content;
mod utils;
