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

// For some reason, Clippy thinks this is unused.
#[allow(unfulfilled_lint_expectations)]
#[expect(dead_code)]
pub trait IsStatic {
	fn is_static(&self) -> bool;
}
