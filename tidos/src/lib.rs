//! # Tidos
//!
//! Tidos is a Rust **server-side rendering (SSR)** component framework.
//! Write type-safe HTML components directly in Rust using the [`view!`] and
//! [`page!`] macros — with full support for loops, conditionals, and pattern
//! matching inside your templates.
//!
//! ## Getting started
//!
//! ```toml
//! [dependencies]
//! tidos = "0.7.2"
//!
//! # With Rocket integration:
//! # tidos = { version = "0.7.2", features = ["rocket"] }
//!
//! # With Rocket + internationalization support:
//! # tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
//! ```
//!
//! ## Core concepts
//!
//! | Item | Description |
//! |---|---|
//! | [`view!`] | Renders a fragment of HTML. Returns a `String`. |
//! | [`page!`] | Wraps a full page. Returns a [`Page`] ready to return from a route. |
//! | [`Component`] | Trait for reusable components; implement [`to_render`](Component::to_render). |
//! | [`Page`] | Collects rendered HTML and `<head>` elements for a full page response. |
//! | [`scoped_css!`] | Injects a scoped `<style>` into `<head>` and returns the generated class name. |
//! | [`head!`] | Injects arbitrary HTML into the page `<head>`. |
//! | [`i18n::i18n!`] | *(feature: `i18n`)* Looks up a Fluent translation key. |
//! | [`i18n::enable_i18n!`] | *(feature: `i18n`)* Initialises the translation system in `main.rs`. |
//!
//! ## Defining a component
//!
//! Implement [`Component`] on any struct and use [`view!`] inside
//! [`to_render`](Component::to_render):
//!
//! ```rust,no_run
//! use tidos::{view, Component, Page};
//!
//! pub struct Card {
//!     pub title: String,
//!     pub body: String,
//! }
//!
//! impl Component for Card {
//!     fn to_render(&self, page: &mut Page) -> String {
//!         view! {
//!             <div class="card">
//!                 <h2>{&self.title}</h2>
//!                 <p>{&self.body}</p>
//!             </div>
//!         }
//!     }
//! }
//! ```
//!
//! ## Building a page
//!
//! Use [`page!`] in a route handler to produce a full [`Page`] response.
//! Embed components with JSX-like self-closing tags:
//!
//! ```rust,no_run
//! use tidos::{page, Component, Page};
//!
//! #[get("/")]
//! pub fn index() -> Page {
//!     page! {
//!         <main>
//!             <Card title={ String::from("News") } body={ String::from("Hello world!") } />
//!         </main>
//!     }
//! }
//! ```
//!
//! ## Template syntax
//!
//! ### Interpolating Rust expressions
//!
//! Wrap any Rust expression in `{ }` to interpolate it:
//!
//! ```rust,no_run
//! use tidos::view;
//!
//! let name = "Alice";
//! let count = 42_usize;
//!
//! view! {
//!     <p>Hello {name}, you have {count.to_string()} messages.</p>
//! }
//! ```
//!
//! ### `{#for}` — loops
//!
//! ```rust,no_run
//! use tidos::view;
//!
//! let fruits = vec!["apple", "banana", "cherry"];
//!
//! view! {
//!     <ul>
//!         {#for fruit in fruits}
//!             <li>{fruit}</li>
//!         {/for}
//!     </ul>
//! }
//! ```
//!
//! ### `{#if}` — conditionals
//!
//! ```rust,no_run
//! use tidos::view;
//!
//! let age = 20_u32;
//! let is_american = false;
//!
//! view! {
//!     {#if age >= 18 && !is_american}
//!         <p>Allowed to drink.</p>
//!     {:else if age >= 21 && is_american}
//!         <p>Allowed to drink (US rules).</p>
//!     {:else}
//!         <p>Not allowed to drink.</p>
//!     {/if}
//! }
//! ```
//!
//! ### `{#match}` — pattern matching
//!
//! ```rust,no_run
//! use tidos::view;
//!
//! enum Status { Active, Banned, Guest }
//! let status = Status::Active;
//!
//! view! {
//!     {#match status}
//!         {:case Status::Active}
//!             <span class="green">Active</span>
//!         {:case Status::Banned}
//!             <span class="red">Banned</span>
//!         {:case _}
//!             <span class="gray">Guest</span>
//!     {/match}
//! }
//! ```
//!
//! ## Scoped CSS
//!
//! [`scoped_css!`] reads a CSS file at compile time, generates a unique class
//! name, and injects a `<style>` block into the page `<head>`. The macro
//! returns the class name as a `&'static str`; apply it to the component's
//! root element. [`Page::add_elements_to_head`] deduplicates by UUID, so
//! calling `scoped_css!` inside a loop is safe.
//!
//! ```rust,no_run
//! use tidos::{scoped_css, view, Component, Page};
//!
//! pub struct Card {
//!     pub title: String,
//! }
//!
//! impl Component for Card {
//!     fn to_render(&self, page: &mut Page) -> String {
//!         view! {
//!             <div class={scoped_css!("./card.css")}>
//!                 <h2>{&self.title}</h2>
//!             </div>
//!         }
//!     }
//! }
//! ```
//!
//! Use CSS nesting so all styles live in one file:
//!
//! ```css
//! & {
//!     background: #16213e;
//!     border-radius: 8px;
//! }
//!
//! & h2 {
//!     color: #a8dadc;
//! }
//! ```
//!
//! ## Injecting `<head>` elements
//!
//! Use [`head!`] inside `to_render` to add arbitrary HTML to the page
//! `<head>` (e.g. a `<title>` or a `<link rel="stylesheet">`):
//!
//! ```rust,no_run
//! use tidos::{head, Component, Page};
//!
//! pub struct Title {
//!     pub title: String,
//! }
//!
//! impl Component for Title {
//!     fn to_render(&self, page: &mut Page) -> String {
//!         head! {
//!             <title>{&self.title}</title>
//!         }
//!         String::new()
//!     }
//! }
//! ```
//!
//! ## Internationalization
//!
//! See the [`mod@i18n`] module for full details. Enable the feature flag and call
//! [`i18n::enable_i18n!`] once in `main.rs`:
//!
//! ```toml
//! tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
//! ```
//!
//! ```rust,no_run
//! use tidos::i18n::{enable_i18n, i18n, Lang};
//! use tidos::{view, page, Component, Page};
//!
//! enable_i18n!();
//!
//! pub struct Greeting;
//!
//! impl Component for Greeting {
//!     fn to_render(&self, page: &mut Page) -> String {
//!         view! {
//!             <h1>{i18n!("greeting")}</h1>
//!         }
//!     }
//! }
//!
//! #[get("/<lang>")]
//! pub fn index(lang: Lang) -> Page {
//!     page! { <Greeting /> }
//! }
//! ```

mod components;
mod page;

#[doc(hidden)]
pub mod internals;
#[cfg(feature = "i18n")]
mod i18n_config;

pub use components::Component;
#[doc(hidden)]
pub use internals::sanitize;
pub use page::Page;

#[doc(inline)]
pub use tidos_macro::*;

#[doc(hidden)]
#[macro_export]
macro_rules! sanitize {
	($input:expr) => {
		&tidos::internals::sanitize(&$input)
	};
}

/// Internationalization support for Tidos, backed by [Fluent](https://projectfluent.org/).
///
/// Enable with `features = ["i18n"]` in `Cargo.toml`.
///
/// # Setup
///
/// **1.** Call [`enable_i18n!`] once at the top level of `main.rs`.
///
/// **2.** Create a `Tidos.toml` in the project root:
///
/// ```toml
/// [default]
/// resource_location = "translations"
/// default_locale    = "en-US"
/// resources         = ["common.ftl"]
/// ```
///
/// **3.** Place `.ftl` files under `resource_location`, grouped by locale:
///
/// ```text
/// translations/
/// ├── en-US/
/// │   └── common.ftl
/// └── nl-NL/
///     └── common.ftl
/// ```
///
/// **4.** Add `lang: Lang` to route handlers (locale is the first path
/// segment, e.g. `/en-US`), then call [`i18n!`] inside any component.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::i18n::{enable_i18n, i18n, Lang};
/// use tidos::{view, page, Component, Page};
///
/// enable_i18n!();
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
///
/// #[get("/<lang>")]
/// pub fn index(lang: Lang) -> Page {
///     page! { <Greeting /> }
/// }
/// ```
#[cfg(feature = "i18n")]
pub mod i18n {
	pub use fluent;
	pub use unic_langid;
	pub use fluent_langneg;
	pub use fluent_resmgr;
	pub use lazy_static;

	/// Initialises the Tidos i18n translation system.
	///
	/// Call this macro **once**, at the top level of `main.rs`, before
	/// launching your web server. It creates a lazily-initialised global that
	/// reads [`TidosI18nConfig`] from `Tidos.toml` (or environment variables
	/// prefixed with `TIDOS_I18N_`).
	///
	/// # Example
	///
	/// ```rust,no_run
	/// use tidos::i18n::enable_i18n;
	///
	/// enable_i18n!();
	///
	/// #[rocket::main]
	/// async fn main() {
	///     // launch Rocket here
	/// }
	/// ```
	#[macro_export]
	macro_rules! enable_i18n {
		() => {
			tidos::i18n::lazy_static::lazy_static! {
				static ref TIDOS_I18N_CONFIGURATION: tidos::i18n::TidosI18nConfig = {
					tidos::i18n::TidosI18nConfig::figment()
						.extract()
						.unwrap()
				};
			}
		}
	}
	pub use crate::enable_i18n;

	#[doc(inline)]
	pub use tidos_i18n::*;

	pub use crate::i18n_config::TidosI18nConfig;

	pub use crate::page::Lang;
}