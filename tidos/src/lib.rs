//! # Tidos - Documentation
//! Tidos is a high-performance Rust-based component framework that seamlessly integrates with any web framework, enabling developers to build dynamic web applications with ease. With Tidos’ powerful macros, you can intuitively create components directly within your Rust code. It even allows you to leverage Rust's pattern matching, loops, and conditionals inside your components—making your UI logic more expressive and maintainable.
//!
//! ## Usage
//! ```toml
//! tidos = "0.6.8"
//! ```
//! ## Example
//! ```rust,no_run
//! use tidos::{Component, Page};
//!
//! pub struct Greet {
//!     pub name: String,
//! }
//!
//! impl Component for Greet {
//!     fn to_render(&self, page: &mut Page) -> String {
//!         view! {
//!             <h1>Hello {&self.name}</h1>
//!         }
//!     }
//! }
//!
//!
//! // Example route from Rocket, but you can use any framework you want.
//! #[get("/")]
//! pub fn index() -> Page {
//!     page! {
//!         <main>
//!             <Greet name={ String::from("kaasbroodju") } />
//!         </main>
//!     }
//! }
//! //! ```

mod components;
mod page;

#[doc(hidden)]
pub mod internals;
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

#[cfg(feature = "i18n")]
pub mod i18n {

	#[macro_export]
	macro_rules! enable_i18n {
		() => {
			lazy_static::lazy_static! {
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
