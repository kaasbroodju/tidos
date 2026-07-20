use crate::components::code_block::CodeBlock;
use crate::components::docs_layout::DocsLayout;
use crate::components::nav_bar::NavBarDocs;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/docs/internationalization")]
pub fn internationalization() -> Page {
    let mut page = page! {
        <NavBarDocs />
        <DocsLayout>
            {#slot:content}
                <I18nContent />
            {/slot}
        </DocsLayout>
    };

    {
        let page = &mut page;
        head! {
            <script type="application/ld+json">@html{include_str!("./internationalization.jsonld")}</script>
        }
    }

    page
}

struct I18nContent;

impl Component for I18nContent {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{"Internationalization - Tidos"}</title>
        }

        let cargo_toml = "[dependencies]\ntidos = { version = \"0.8.0-rc.1\", features = [\"rocket\", \"i18n\"] }";

        let tidos_toml = "[default]\nresource_location = \"translations\"\ndefault_locale = \"en-US\"\nresources = [\"common.ftl\"]";

        let file_tree = "translations/\n\u{251C}\u{2500}\u{2500} en-US/\n\u{2502}   \u{2514}\u{2500}\u{2500} common.ftl\n\u{2514}\u{2500}\u{2500} nl-NL/\n    \u{2514}\u{2500}\u{2500} common.ftl";

        let ftl_example = "# translations/en-US/common.ftl\ngreeting = Hello, { $name }!\n\nwelcome-message =\n    Welcome back, { $name }.\n    You have { $count ->\n        [one] one new message\n       *[other] { $count } new messages\n    }.";

        let enable_i18n = "// main.rs\nuse tidos::i18n::enable_i18n;\n\nenable_i18n!();\n\n#[rocket::main]\nasync fn main() {\n    rocket::build()\n        .mount(\"/\", routes![index])\n        .launch()\n        .await\n        .unwrap();\n}";

        let route_example = "use tidos::i18n::Lang;\nuse tidos::{page, Page};\nuse rocket::get;\n\n// The locale is the first path segment: /en-US or /nl-NL\n#[get(\"/<lang>\")]\npub fn index(lang: Lang) -> Page {\n    page! {\n        <main>\n            <Greeting />\n        </main>\n    }\n}";

        let component_example = "use tidos::{view, Component, Page};\nuse tidos::i18n::i18n;\n\npub struct Greeting;\n\nimpl Component for Greeting {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <section>\n                // Simple lookup\n                <h1>{i18n!(\"greeting\", (\"name\", \"Alice\"))}</h1>\n\n                // Lookup with plural variable\n                <p>{i18n!(\"welcome-message\", (\"name\", \"Alice\"), (\"count\", 3))}</p>\n            </section>\n        }\n    }\n}";

        view! {
            <div class="doc-content">
                <h1>{"Internationalization"}</h1>
                <p>{"Tidos provides built-in i18n support via "}<a href="https://projectfluent.org/">{"Fluent"}</a>{". Enable it with the i18n feature flag to get locale-aware translations, pluralization, and gender variants."}</p>

                <h2>{"1. Enable the Feature"}</h2>
                <CodeBlock code={cargo_toml.to_string()} />

                <h2>{"2. Create Tidos.toml"}</h2>
                <p>{"Place Tidos.toml in your project root to configure translation file locations:"}</p>
                <CodeBlock code={tidos_toml.to_string()} />

                <h2>{"3. Add Translation Files"}</h2>
                <p>{"Organize .ftl files by locale under the translations directory:"}</p>
                <CodeBlock code={file_tree.to_string()} />
                <CodeBlock code={ftl_example.to_string()} />

                <h2 id="the-i18n-macro">{"4. Initialize in main.rs"}</h2>
                <p>{"Call enable_i18n! once at startup to load the translation configuration:"}</p>
                <CodeBlock code={enable_i18n.to_string()} />

                <h2>{"5. Add Lang to Routes"}</h2>
                <p>{"The locale is expected as the first URL path segment. Add lang: Lang to your route handler and page! picks it up automatically:"}</p>
                <CodeBlock code={route_example.to_string()} />

                <h2>{"6. Translate with i18n!"}</h2>
                <p>{"Call i18n! inside any component with a Fluent message key and optional variable pairs:"}</p>
                <CodeBlock code={component_example.to_string()} />
            </div>
        }
    }
}
