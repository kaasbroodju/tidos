use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-7-6")]
pub fn news_v0_7_6() -> Page {
    let mut page = page! {
        <NavBarDocs />
        <NewsLayout>
            {#slot:content}
                <ArticleContent />
            {/slot}
        </NewsLayout>
    };

    {
        let page = &mut page;
        head! {
            <title>{"v0.7.6: Default Props - Tidos News"}</title>
            <meta name="description" content="Components can now use the Default trait and .. syntax to fill unspecified props with default values." />
        }
    }

    page
}

struct ArticleContent;

impl Component for ArticleContent {
    fn to_render(&self, page: &mut Page) {
        let code_alert = "#[derive(Default)]\npub struct Alert {\n    pub message: String,\n    pub kind: AlertKind,  // defaults to AlertKind::Info\n    pub dismissible: bool, // defaults to false\n}\n\n// Only set what you care about\nview! {\n    <Alert message={\"Saved!\".into()} .. />\n}";
        let code_table = "#[derive(Default)]\npub struct DataTable {\n    pub rows: Vec<Row>,\n    pub striped: bool,       // false\n    pub bordered: bool,      // false\n    pub page_size: usize,    // 0 = show all\n}\n\nview! {\n    // Use all defaults except rows\n    <DataTable rows={data} .. />\n\n    // Override a couple of options\n    <DataTable rows={data} striped={true} page_size={25} .. />\n}";
        let code_cargo = "[dependencies]\ntidos = \"0.7.6\"";

        view! {
            <div class="article-content">
                <a class="back-link" href="/news">{"← All news"}</a>

                <h1>{"Default Props"}</h1>
                <div class="article-meta">
                    <span class="article-version">{"v0.7.6"}</span>
                    <span>{"April 5, 2026"}</span>
                </div>

                <p>{"v0.7.6 adds support for the Default trait in component syntax, letting you omit props that have sensible defaults."}</p>

                <h2>{"The .. syntax"}</h2>
                <p>{"When a component struct derives or implements Default, you can now use .. after any explicit props to fill the remaining fields with their default values — identical to Rust's struct update syntax:"}</p>
                <CodeBlock code={code_alert.to_string()} />
                <p>{"The .. shorthand must appear after all explicit props and before the closing />"}</p>

                <h2>{"Useful for configuration components"}</h2>
                <p>{"This pattern works well for components with many optional settings, such as modals, toasts, tables, or form inputs. Define your defaults once on the struct, and callers only need to specify what differs:"}</p>
                <CodeBlock code={code_table.to_string()} />

                <h2>{"Upgrading"}</h2>
                <p>{"No breaking changes. Update your dependency and derive Default on any component struct you want to use the shorthand with:"}</p>
                <CodeBlock code={code_cargo.to_string()} />
            </div>
        }
    }
}
