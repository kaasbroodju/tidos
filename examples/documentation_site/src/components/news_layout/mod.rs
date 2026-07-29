use tidos::{head, scoped_css, view, Component, Page, Slot};

/// Shared chrome for `/news` and `/news/*` pages: sets base page styles and
/// renders whichever article (or the news index) is passed in via `content`.
pub struct NewsLayout<'a> {
    pub content: Slot<'a>,
}

impl Component for NewsLayout<'_> {
    fn to_render(&self, page: &mut Page) {
        head! {
            <style>@html{"html,body{margin:0;padding:0;color:#fff;font-family:'Roboto',sans-serif;background-color:#0d1117}"}</style>
        }

        view! {
            <div class={scoped_css!("./news_layout.css")}>
                <div class="content">
                    @slot{self.content}
                </div>
            </div>
        }
    }
}
