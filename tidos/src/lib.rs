#![doc = include_str!("lib.md")]

mod components;
mod page;

#[cfg(feature = "i18n")]
mod i18n_config;
#[doc(hidden)]
pub mod internals;

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
///             <p>{i18n!("shared-photos", ("userName", "Anne"), ("photoCount", 3))}</p>
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
	pub use fluent_langneg;
	pub use fluent_resmgr;
	pub use lazy_static;
	pub use unic_langid;

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
		};
	}
	pub use crate::enable_i18n;

	#[doc(inline)]
	pub use tidos_i18n::*;

	pub use crate::i18n_config::TidosI18nConfig;

	pub use crate::page::Lang;
}
