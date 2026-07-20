use tidos::{native_element, scoped_css, view, Component, Page};

#[native_element]
pub struct InstallPill;

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
