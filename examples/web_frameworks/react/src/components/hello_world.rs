use tidos::{Component, Page, view, scoped_css};

pub struct HelloWorld;

impl Component for HelloWorld {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head!(
            <script r#type="module" src="/dist/HelloWorld.js"></script>
        );

        view!(
            <div class={scoped_css!("./hello_world.css")}>
                <hello-world></hello-world>
            </div>
        )
    }
}
