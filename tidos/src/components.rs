pub(crate) use crate::page::Page;

/// Type alias for a slot closure — a boxed function that pushes rendered HTML
/// into a [`Page`] when called.
///
/// Use this as a field type on components that accept slot content passed in
/// by their parent. Inside `to_render`, invoke the slot with `@slot{...}`
/// (which the `view!` macro expands to a call of the closure). A component
/// can take its slot content in one of two shapes:
///
/// - **Unnamed** — a single slot, tuple struct with `Slot<'render>` as field
///   `0`. The parent passes children directly, with no `{#slot:name}`
///   wrapper. Use this only when the component needs exactly one slot and
///   has no other props.
/// - **Named** — a regular struct field, filled by a `{#slot:name} … {/slot}`
///   block on the parent side. Use this when the component has more than one
///   slot, or a single slot alongside other props/attributes (an unnamed
///   tuple field can't coexist with named fields in the same tag).
///
/// # Example — unnamed slot
///
/// ```rust,no_run
/// use tidos::{view, Component, Page, Slot};
///
/// pub struct Card<'a>(pub Slot<'a>);
///
/// impl Component for Card<'_> {
///     fn to_render(&self, page: &mut Page) {
///         view! {
///             <div class="card">
///                 @slot{self.0}
///             </div>
///         }
///     }
/// }
///
/// // Parent side — children are passed straight in, no {#slot:name} needed:
/// // view! {
/// //     <Card>
/// //         <p>{"Card content"}</p>
/// //     </Card>
/// // }
/// ```
///
/// # Example — named slots
///
/// ```rust,no_run
/// use tidos::{view, Component, Page, Slot};
///
/// pub struct Card<'a> {
///     pub header: Slot<'a>,
///     pub body: Slot<'a>,
/// }
///
/// impl Component for Card<'_> {
///     fn to_render(&self, page: &mut Page) {
///         view! {
///             <div class="card">
///                 <header>@slot{self.header}</header>
///                 <main>@slot{self.body}</main>
///             </div>
///         }
///     }
/// }
///
/// // Parent side:
/// // view! {
/// //     <Card>
/// //         {#slot:header}<h1>{"Title"}</h1>{/slot}
/// //         {#slot:body}<p>{"Content"}</p>{/slot}
/// //     </Card>
/// // }
/// ```
pub type Slot<'render> = Box<dyn Fn(&mut Page) + 'render>;

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
