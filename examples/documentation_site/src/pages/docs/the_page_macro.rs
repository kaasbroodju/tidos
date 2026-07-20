use crate::components::code_block::CodeBlock;
use crate::components::docs_layout::DocsLayout;
use crate::components::nav_bar::NavBarDocs;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/docs/the-page-macro")]
pub fn the_page_macro() -> Page {
    let mut page = page! {
        <NavBarDocs />
        <DocsLayout>
            {#slot:content}
                <PageMacroContent />
            {/slot}
        </DocsLayout>
    };

    {
        let page = &mut page;
        head! {
            <script type="application/ld+json">@html{include_str!("./the_page_macro.jsonld")}</script>
        }
    }

    page
}

struct PageMacroContent;

impl Component for PageMacroContent {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{"The page! Macro - Tidos"}</title>
        }

        let basic_example = "use tidos::{page, Page};\nuse rocket::{get, routes};\n\n#[get(\"/\")]\npub fn index() -> Page {\n    page! {\n        <main>\n            <h1>{\"Hello, world!\"}</h1>\n            <p>{\"Built with Tidos.\"}</p>\n        </main>\n    }\n}\n\n#[rocket::main]\nasync fn main() {\n    rocket::build()\n        .mount(\"/\", routes![index])\n        .launch()\n        .await\n        .unwrap();\n}";

        let with_component = "use tidos::{page, view, Component, Page};\nuse rocket::get;\n\npub struct Greeting {\n    pub name: String,\n}\n\nimpl Component for Greeting {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <h1>{\"Hello, {}!\", &self.name}</h1>\n        }\n    }\n}\n\n#[get(\"/\")]\npub fn index() -> Page {\n    page! {\n        <main>\n            <Greeting name={\"World\".to_string()} />\n        </main>\n    }\n}";

        let head_example = "use tidos::{page, head, Page};\nuse rocket::get;\n\n#[get(\"/\")]\npub fn index() -> Page {\n    let page = page! {\n        <main>\n            <h1>{\"Hello!\"}</h1>\n        </main>\n    }\n\n    head! {\n        <title>{\"My App\"}</title>\n        <meta name=\"description\" content=\"Built with Tidos\" />\n        <link rel=\"stylesheet\" href=\"/style.css\" />\n    }\n}";

        view! {
            <div class="doc-content">
                <h1>{"The page! Macro"}</h1>
                <p>{"The page! macro is the entry point for rendering a full HTML page. Use it inside route handlers to produce a Page struct that your framework serves directly."}</p>

                <h2>{"Basic Usage"}</h2>
                <p>{"page! accepts the same template syntax as "}<a href="/docs/component#the-view-macro">{"the view! macro"}</a>{": HTML tags, Rust expressions, and control-flow blocks. It creates and returns a Page struct."}</p>
                <CodeBlock code={basic_example.to_string()} />

                <h2>{"Using Components"}</h2>
                <p>{"Any struct implementing the Component trait can be used in your pages."}</p>
                <CodeBlock code={with_component.to_string()} />
            </div>
        }
    }
}
