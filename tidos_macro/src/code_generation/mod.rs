//! Turns the `tokens::*` AST produced by `parsing/*` into the final
//! `quote!`-generated Rust code.
//!
//! There is no public re-export here: every file implements `ToTokens` (or
//! the internal `ToFlatArgs`, see `component.rs`) directly on a type from
//! `crate::tokens`, so the impl is picked up anywhere that type and the
//! `quote::ToTokens` trait are both in scope, so there's no need to route
//! through this module. `component.rs` defines the shared machinery (`FlatArg`,
//! `to_push_stmts`, `flush_flat`, `ToFlatArgs`) that every other file in this
//! module builds on; read it first.
mod attribute;
mod component;
mod content;
mod control_tag;
mod html_tag;
mod text_content;
