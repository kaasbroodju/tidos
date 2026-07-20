use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-8-0-rc2")]
pub fn news_v0_8_0_rc2() -> Page {
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
            <title>{"v0.8.0-rc.2: Components Render Directly Into Page - Tidos News"}</title>
            <meta name="description" content="to_render no longer returns a String, attributes gain the full text-content syntax, Slot's lifetime is fixed so it can finally borrow, and components can now take a single unnamed slot." />
        }
    }

    page
}

struct ArticleContent;

impl Component for ArticleContent {
    fn to_render(&self, page: &mut Page) {
        let code_render_before = "impl Component for Card {\n    fn to_render(&self, page: &mut Page) -> String {\n        view! {\n            <div class=\"card\">{&self.title}</div>\n        }\n    }\n}";
        let code_render_after = "impl Component for Card {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class=\"card\">{&self.title}</div>\n        }\n    }\n}";

        let code_attr_before = "// Before: attribute expressions always went through sanitize!,\n// so they had to already be string-like.\n<div class={class_name}></div>";
        let code_attr_after = "// After: attributes accept the same three forms as text content.\n<div class={\"card\"}></div>                 // literal\n<div class={\"card-{}\", size}></div>        // formatted\n<div data-count={count.to_string()}></div> // any expression";

        let code_slot_lifetime = "// Before: Slot was implicitly 'static, so slot closures\n// could not borrow anything shorter-lived than 'static.\npub type Slot = Box<dyn Fn(&mut Page)>;\n\n// After: Slot carries an explicit lifetime.\npub type Slot<'render> = Box<dyn Fn(&mut Page) + 'render>;\n\npub struct Card<'a> {\n    pub body: Slot<'a>,\n}";

        let code_named_slot = "// Named: use this when a component has more than one slot,\n// or a single slot alongside other props.\npub struct Card<'a> {\n    pub title: String,\n    pub body: Slot<'a>,\n}\n\nimpl Component for Card<'_> {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class=\"card\">\n                <h2>{&self.title}</h2>\n                @slot{self.body}\n            </div>\n        }\n    }\n}\n\n// Parent:\nview! {\n    <Card title={\"My Card\".to_string()}>\n        {#slot:body}<p>{\"Content\"}</p>{/slot}\n    </Card>\n}";

        let code_unnamed_slot = "// Unnamed: use this only when a component takes exactly\n// one slot and has no other props.\npub struct Card<'a>(pub Slot<'a>);\n\nimpl Component for Card<'_> {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class=\"card\">\n                @slot{self.0}\n            </div>\n        }\n    }\n}\n\n// Parent — content passed straight in, no {#slot:name} wrapper:\nview! {\n    <Card>\n        <p>{\"Content\"}</p>\n    </Card>\n}";

        let code_page_import = "// Before\nuse tidos::{page, Component, Page};\n\n// After — page! imports Component for you\nuse tidos::{page, Page};";

        let code_cargo = "[dependencies]\ntidos = \"0.8.0-rc.2\"";

        view! {
            <div class="article-content">
                <a class="back-link" href="/news">{"← All news"}</a>

                <h1>{"Components Render Directly Into Page"}</h1>
                <div class="article-meta">
                    <span class="article-version">{"v0.8.0-rc.2"}</span>
                    <span>{"July 20, 2026"}</span>
                    <span>{"Second release candidate"}</span>
                </div>

                <p>{"v0.8.0-rc.2 continues the work started in rc.1: cutting out runtime overhead and unnecessary intermediate allocations from HTML generation. The biggest change is how components hand off their output — they no longer build and return a String, they write straight into the page buffer."}</p>

                <h2>{"Breaking change: to_render no longer returns String"}<span class="breaking-badge">{"Breaking"}</span></h2>
                <p>{"Component::to_render used to build a String and return it, which every parent then had to copy into its own buffer. It now renders directly into the page: &mut Page argument and returns nothing — one buffer for the whole page, not one per component:"}</p>
                <CodeBlock code={code_render_before.to_string()} />
                <CodeBlock code={code_render_after.to_string()} />
                <p>{"Drop the -> String from every Component impl you've written, and remove any explicit return value — view! and head! already push into page for you. Nothing about the view! syntax itself changes."}</p>

                <h2>{"Attributes now share the text-content syntax"}</h2>
                <p>{"Attribute values used to be piped straight through sanitize!, so an attribute expression had to already produce a string. Attributes now go through the same parser as text content between tags, so they accept a literal, a format string, or any expression:"}</p>
                <CodeBlock code={code_attr_before.to_string()} />
                <CodeBlock code={code_attr_after.to_string()} />
                <p>{"This applies equally to native HTML attributes and to props on your own components."}</p>

                <h2>{"Slot's lifetime"}<span class="fix-badge">{"Fix"}</span></h2>
                <p>{"Slot was defined as Box<dyn Fn(&mut Page)>, which is implicitly 'static — a slot closure could never borrow anything shorter-lived than the whole program. That was never the intent; slots were always meant to be able to borrow from their surrounding scope. Slot now carries an explicit lifetime, fixing that:"}</p>
                <CodeBlock code={code_slot_lifetime.to_string()} />

                <h2>{"New: unnamed slots"}</h2>
                <p>{"Components that take a single slot and no other props no longer need a named field and a {#slot:name} wrapper. Declare the component as a tuple struct with Slot<'a> as field 0, and the parent passes content directly as children:"}</p>
                <CodeBlock code={code_unnamed_slot.to_string()} />
                <p>{"Named slots are still there for the cases unnamed slots don't cover — multiple slots on one component, or a single slot alongside other props:"}</p>
                <CodeBlock code={code_named_slot.to_string()} />
                <p>{"The two forms don't mix within a single tag: if a component tag's first child is a {#slot:name} block, the whole tag is treated as named-slot mode, and any other top-level children are silently ignored. Pick one form per component."}</p>

                <h2>{"Smaller quality-of-life changes"}<span class="qol-badge">{"Quality of life"}</span></h2>
                <p>{"page! now imports the Component trait for you, so you no longer need it in scope just to call page!:"}</p>
                <CodeBlock code={code_page_import.to_string()} />
                <p>{"Component structs and their attributes also keep their original source spans through macro expansion, so 'go to definition' and ctrl-click navigation in your IDE now jump to the right place on both a component tag and its individual attributes."}</p>

                <h2>{"Under the hood"}</h2>
                <p>{"A criterion + allocation-tracking benchmark suite was added (tidos/benches/page_rendering.rs) covering flat lists, static pages, borrowed data, and conditional rendering, so future changes to the code generator can be checked against real numbers instead of guesswork. The IsStatic trait used by the compile-time HTML folding introduced in rc.1 was also unified across all content and attribute kinds — it isn't wired into a new optimization yet, but it's the groundwork for folding fully-static components into a single concat! at compile time."}</p>

                <h2>{"New documentation site"}</h2>
                <p>{"The site you're reading this on is new this cycle — a Tidos site, written in Tidos, covering the view! and page! syntax, components, slots, scoped CSS, and internationalization."}</p>

                <h2>{"Upgrading"}</h2>
                <p>{"Update your Cargo.toml to the second release candidate:"}</p>
                <CodeBlock code={code_cargo.to_string()} />
                <p>{"Then, for every Component you've implemented: remove -> String from to_render's signature, and remove any explicit return of the rendered string — view!/head! already write into page. If you pass attribute values that aren't already string-like, you can now drop the manual .to_string()/format!() wrapping in most cases, since attributes accept expressions directly."}</p>
            </div>
        }
    }
}
