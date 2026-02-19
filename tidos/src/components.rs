pub(crate) use crate::page::Page;

/// A reusable UI component.
///
/// Implement this trait on any struct to make it renderable inside
/// [`view!`](macro@crate::view), [`page!`](macro@crate::page), or other
/// components. The [`to_render`](Component::to_render) method receives a
/// mutable reference to the current [`Page`] so that child components can
/// inject CSS or `<head>` elements.
///
/// # Example
///
/// ```rust,no_run
/// use tidos::{view, Component, Page};
///
/// pub struct Alert {
///     pub message: String,
/// }
///
/// impl Component for Alert {
///     fn to_render(&self, page: &mut Page) -> String {
///         view! {
///             <div class="alert">
///                 <p>{&self.message}</p>
///             </div>
///         }
///     }
/// }
/// ```
pub trait Component {
	/// Render the component to an HTML string.
	///
	/// Use [`view!`](macro@crate::view) to build the HTML. The `page`
	/// parameter gives access to the current [`Page`] so you can call
	/// [`Page::add_elements_to_head`] or forward it to child components via
	/// their own `to_render(page)` calls.
	fn to_render(&self, page: &mut Page) -> String;
}