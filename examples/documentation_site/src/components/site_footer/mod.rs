use tidos::{scoped_css, view, Component, Page};

pub struct SiteFooter;

impl Component for SiteFooter {
    fn to_render(&self, page: &mut Page) {
        view! {
            <footer class={scoped_css!("./site_footer.css")}>
                <span>{"© 2026 Morris Waaijer built with Tidos in Rust"}</span>
                <span class="links">
                    <a href="https://github.com/kaasbroodju/tidos" target="_blank">{"GitHub"}</a>
                    {" · "}<a href="https://crates.io/crates/tidos" target="_blank">{"crates.io"}</a>{" · "}
                    <a href="https://docs.rs/tidos/latest/tidos/" target="_blank">{"docs.rs"}</a>
                </span>
            </footer>
        }
    }
}
