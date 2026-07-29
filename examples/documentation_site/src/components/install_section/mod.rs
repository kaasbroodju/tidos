use tidos::{native_element, scoped_css, view, Component, Page};

/// Copy-to-clipboard "cargo add tidos" button, implemented as a Svelte
/// custom element (`InstallPill.svelte`). No props: the command it copies is
/// hardcoded there.
#[native_element]
pub struct InstallPill;

/// Homepage section wrapper that centers and styles a single [`InstallPill`].
pub struct InstallSection;

impl Component for InstallSection {
    fn to_render(&self, page: &mut Page) {
        view! {
            <section class={scoped_css!("./install_section.css")}>
                <InstallPill />
            </section>
        }
    }
}
