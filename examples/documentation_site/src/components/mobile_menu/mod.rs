use tidos::native_element;

/// Accessible hamburger toggle (WAI-ARIA disclosure pattern), implemented as a
/// Svelte 5 custom element. It renders a real `<button>` with `aria-expanded`
/// and `aria-controls`, and toggles the controlled `<nav>` by id.
///
/// - `controls`: the `id` of the element to show/hide (the `<nav>`).
/// - `label`: accessible name for the button.
#[native_element]
pub struct MobileMenu {
    pub controls: String,
    pub label: String,
}
