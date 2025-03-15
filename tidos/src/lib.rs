//! # Tidos - Documentation
//! Tidos is a high-performance Rust-based component framework that seamlessly integrates with any web framework, enabling developers to build dynamic web applications with ease. With Tidos’ powerful macros, you can intuitively create components directly within your Rust code. It even allows you to leverage Rust's pattern matching, loops, and conditionals inside your components—making your UI logic more expressive and maintainable.

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
