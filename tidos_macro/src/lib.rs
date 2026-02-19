//! Procedural macros for the [Tidos](https://docs.rs/tidos) SSR framework.
//!
//! This crate is an implementation detail. All macros are re-exported by the
//! `tidos` crate via `#[doc(inline)] pub use tidos_macro::*;`, so you should
//! depend on `tidos` directly rather than on this crate.
//!
//! For usage examples and full documentation see the
//! [`tidos` crate docs](https://docs.rs/tidos).

extern crate proc_macro;

use proc_macro::TokenStream;

use crate::page_wrapper::{I18nHoist, PageWrapper};
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use tokens::Component;
use uuid::Uuid;
use crate::i18n::I18n;

mod code_generation;
mod page_wrapper;
mod parsing;
mod tokens;
mod i18n;

/// Renders an HTML template to a `String`.
///
/// Write HTML directly in Rust. Interpolate Rust expressions with `{ }` and
/// use the control-flow blocks below for loops, conditionals, and pattern
/// matching.
///
/// # Syntax summary
///
/// | Syntax | Meaning |
/// |---|---|
/// | `{expr}` | Interpolate a Rust expression |
/// | `{#for x in iter} … {/for}` | Loop |
/// | `{#if cond} … {:else if cond} … {:else} … {/if}` | Conditional |
/// | `{#match val} {:case Pat} … {/match}` | Pattern match |
/// | `<Component prop={expr} />` | Render a component |
///
/// # Examples
///
/// ## Inline expression
/// ```rust,no_run
/// use tidos_macro::view;
///
/// let name = "Alice";
/// view! {
///     <p>Hello {name}!</p>
/// }
/// ```
///
/// ## For loop
/// ```rust,no_run
/// use tidos_macro::view;
///
/// let names = vec!["Bob", "Alice"];
///
/// view! {
///     {#for name in names}
///         <p>{format!("Hello {}!", name)}</p>
///     {/for}
/// }
/// ```
///
/// ## If / else
/// ```rust,no_run
/// use tidos_macro::view;
///
/// let age = 18;
/// let is_american = false;
///
/// view! {
///     {#if age >= 18 && !is_american}
///         <p>User is allowed to drink.</p>
///     {:else if age >= 21 && is_american}
///         <p>User is allowed to drink.</p>
///     {:else if age >= 18 && age < 21 && is_american}
///         <p>User is probably the designated driver.</p>
///     {:else}
///         <p>User is not allowed to drink.</p>
///     {/if}
/// }
/// ```
///
/// ## Match statement
/// ```rust,no_run
/// use tidos_macro::view;
///
/// enum Pet { Fish, Dog, Cat, Other { name: String } }
/// use Pet::*;
///
/// let my_pet = Dog;
///
/// view! {
///     {#match my_pet}
///         {:case Fish}
///             <p>Blub!</p>
///         {:case Dog}
///             <p>{"Who's a good boy!"}</p>
///         {:case Cat}
///             <p>Give all mortal possessions to cat!</p>
///         {:case _}
///             <p>Is it a snake or a spider?</p>
///     {/match}
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Component);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Renders an HTML template and wraps it in a `Page`.
///
/// Use `page!` in a route handler instead of [`view!`] when you need to
/// return a full page. Internally it creates a `Page`, renders the template
/// into it, and returns the `Page` value.
///
/// Accepts exactly the same template syntax as [`view!`].
///
/// # Example
///
/// ```rust,no_run
/// use tidos::{page, Component, Page};
///
/// pub fn getting_started() -> Page {
///     let x: isize = Default::default();
///
///     page! {
///         <main>
///             <h1>Getting started</h1>
///             <p>{x.to_string()}</p>
///         </main>
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn page(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as PageWrapper);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Injects HTML into the `<head>` of the current page.
///
/// Call `head!` inside `Component::to_render` to add elements such as
/// `<title>`, `<link>`, or `<meta>` tags to the page `<head>`. Each call is
/// deduplicated by a compile-time UUID, so it is safe to call from components
/// rendered inside a loop.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::{head, Component, Page};
///
/// pub struct Title {
///     pub title: String,
/// }
///
/// impl Component for Title {
///     fn to_render(&self, page: &mut Page) -> String {
///         head! {
///             <title>{&self.title}</title>
///         }
///         String::new()
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn head(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as Component);

	let x = Uuid::new_v4().to_string();
	// todo refactor it to &str when I'm comfortable with lifetime annontations
	let input = quote!(
		page.add_elements_to_head(#x, #input);
	);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Injects a scoped CSS file into the page `<head>` and returns the generated class name.
///
/// The path is resolved relative to the calling source file (like
/// `include_str!`). At compile time the macro generates a unique
/// `tidos-<uuid>` class name, wraps the CSS file content as
/// `.tidos-<uuid> { … }`, and injects a `<style>` tag into the page `<head>`
/// via `Page::add_elements_to_head`. Because injection is keyed by UUID,
/// calling `scoped_css!` inside a loop only injects the style once.
///
/// The macro returns a `&'static str` you can bind to a `class` attribute.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::{scoped_css, view, Component, Page};
///
/// pub struct Card { pub title: String }
///
/// impl Component for Card {
///     fn to_render(&self, page: &mut Page) -> String {
///         view! {
///             <div class={scoped_css!("./card.css")}>
///                 <h2>{&self.title}</h2>
///             </div>
///         }
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn scoped_css(input: TokenStream) -> TokenStream {
	let file_name = input.to_string().replace("\"", "");

	let x = format!("tidos-{}", Uuid::new_v4().to_string());
	// todo refactor it to &str when I'm comfortable with lifetime annontations
	let input = quote!(
		{
			page.add_elements_to_head(#x, String::from(concat!("<style>.", #x, " {", include_str!(#file_name), "}</style>")));
			#x
		}
	);

	let expanded = input.to_token_stream();

	expanded.into()
}

/// Looks up a [Fluent](https://projectfluent.org/) translation key and returns the translated `String`.
///
/// Requires the `i18n` feature flag on the `tidos` crate and a call to
/// `enable_i18n!` in `main.rs`.
///
/// The current locale is read from `page.lang`, which is set by the `Lang`
/// Rocket request guard.
///
/// # Syntax
///
/// ```text
/// i18n!("message-key")
/// i18n!("message-key", "variable", value, …)
/// ```
///
/// Variables are passed as alternating key-value pairs after the message key.
/// Keys must be string literals; values can be any Rust expression.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::i18n::i18n;
/// use tidos::{view, Component, Page};
///
/// pub struct Greeting;
///
/// impl Component for Greeting {
///     fn to_render(&self, page: &mut Page) -> String {
///         view! {
///             <h1>{i18n!("greeting")}</h1>
///             <p>{i18n!("shared-photos", "userName", "Anne", "photoCount", 3)}</p>
///         }
///     }
/// }
/// ```
#[allow(clippy::all)]
#[proc_macro]
pub fn i18n(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as I18n);

	let expanded = input.to_token_stream();

	expanded.into()
}