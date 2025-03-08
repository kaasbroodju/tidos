mod components;
mod page;
pub mod internals;

pub use components::{Component};
pub use page::Page;
pub use tidos_macro::{view, page, head, scoped_css};
pub use internals::{sanitize};

#[macro_export]
macro_rules! sanitize {
    ($input:expr) => {&tidos::internals::sanitize(String::from($input))};
}