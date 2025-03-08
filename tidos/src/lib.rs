//! # Tidos - Documentation
//! Tidos is a html framework that compiles

mod components;
mod page;

#[doc(hidden)]
pub mod internals;

pub use components::Component;
#[doc(hidden)]
pub use internals::sanitize;
pub use page::Page;
pub use tidos_macro::*;

#[doc(hidden)]
#[macro_export]
macro_rules! sanitize {
	($input:expr) => {
		&tidos::internals::sanitize(String::from($input))
	};
}
