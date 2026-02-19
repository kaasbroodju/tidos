use tidos::{scoped_css, view, Component, Page};

pub struct NavBar {
    pub title: String,
    pub links: Vec<(String, String)>,
}

impl Component for NavBar {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <nav class={scoped_css!("./nav_bar.css")}>
                <span>{&self.title}</span>
                <ul>
                    {#for (label, href) in &self.links}
                        <li>
                            <a href={href}>{label}</a>
                        </li>
                    {/for}
                </ul>
            </nav>
        }
    }
}
