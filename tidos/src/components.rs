pub(crate) use crate::page::Page;

/// Type alias for a slot closure — a boxed function that pushes rendered HTML
/// into a [`Page`] when called.
///
/// Use this as the field type on components that accept named slot content via
/// `{#slot:name} … {/slot}`. Inside `to_render`, invoke the slot with
/// `@slot{self.field_name}` (which the `view!` macro expands to a call of the
/// closure).
///
/// # Example
///
/// ```rust,no_run
/// use tidos::{view, Component, Page, Slot};
///
/// pub struct Card {
///     pub body: Slot,
/// }
///
/// impl Component for Card {
///     fn to_render(&self, page: &mut Page) {
///         view! {
///             <div class="card">
///                 @slot{self.body}
///             </div>
///         }
///     }
/// }
/// ```
pub type Slot = Box<dyn Fn(&mut Page)>;

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
///     fn to_render(&self, page: &mut Page) {
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
	fn to_render(&self, page: &mut Page);
}
