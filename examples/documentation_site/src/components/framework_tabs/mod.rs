use tidos::native_element;

/// Tab strip for switching between per-framework code samples; interactivity
/// is implemented client-side in `FrameworkTabs.svelte`.
///
/// `frameworks` is a single comma-separated string rather than a
/// `Vec<String>`, because `#[native_element]` forwards every field straight
/// through as an HTML attribute value.
#[native_element]
pub struct FrameworkTabs {
	pub frameworks: String,
	pub active: String,
}
