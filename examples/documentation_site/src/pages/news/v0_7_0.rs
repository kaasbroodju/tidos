use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-7-0")]
pub fn news_v0_7_0() -> Page {
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
            <title>{"v0.7.0: Internationalization - Tidos News"}</title>
            <meta name="description" content="Built-in i18n support powered by Fluent. Translate your pages with the i18n! macro and .ftl translation files." />
        }
    }

    page
}

struct ArticleContent;

impl Component for ArticleContent {
    fn to_render(&self, page: &mut Page) {
        let code_cargo = "[dependencies]\ntidos = { version = \"0.7.0\", features = [\"rocket\", \"i18n\"] }";
        let code_tidos_toml = "[default]\nresource_location = \"translations\"\ndefault_locale = \"en-US\"\nresources = [\"common.ftl\"]";
        let code_enable = "use tidos::i18n::enable_i18n;\n\nenable_i18n!();\n\n#[rocket::main]\nasync fn main() { /* ... */ }";
        let code_simple = "use tidos::i18n::i18n;\n\nimpl Component for Greeting {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <h1>{i18n!(\"greeting\")}</h1>\n        }\n    }\n}";
        let code_vars = "// Fluent message with variable:\n// welcome = Welcome back, { $name }!\n\nview! {\n    <p>{i18n!(\"welcome\", \"name\", &self.username)}</p>\n}";
        let code_ftl_dir = "translations/\n  en-US/\n    common.ftl\n  nl-NL/\n    common.ftl";
        let code_plural = "new-messages =\n    { $count ->\n        [one]  You have one new message.\n       *[other] You have { $count } new messages.\n    }";
        let code_lang = "use tidos::i18n::Lang;\n\n#[get(\"/<lang>/dashboard\")]\npub fn dashboard(lang: Lang) -> Page {\n    page! {\n        <main>\n            <h1>{i18n!(\"dashboard-title\")}</h1>\n        </main>\n    }\n}";
        let code_upgrade = "[dependencies]\ntidos = \"0.7.0\"  # existing usage unaffected";

        view! {
            <div class="article-content">
                <a class="back-link" href="/news">{"← All news"}</a>

                <h1>{"Internationalization"}</h1>
                <div class="article-meta">
                    <span class="article-version">{"v0.7.0"}</span>
                    <span>{"February 19, 2026"}</span>
                </div>

                <p>{"v0.7.0 brings first-class internationalization to Tidos. Powered by Mozilla's Fluent localization system, you can now translate any string in any component with a single macro call."}</p>

                <h2>{"Enabling i18n"}</h2>
                <p>{"Add the i18n feature flag to your Cargo.toml:"}</p>
                <CodeBlock code={code_cargo.to_string()} />
                <p>{"Create a Tidos.toml in your project root to configure the locale directory and default language:"}</p>
                <CodeBlock code={code_tidos_toml.to_string()} />
                <p>{"Then call enable_i18n! once at the top of main.rs to load the translation files at startup:"}</p>
                <CodeBlock code={code_enable.to_string()} />

                <h2>{"Translating strings with i18n!"}</h2>
                <p>{"Inside any component, import and call i18n! with a Fluent message key:"}</p>
                <CodeBlock code={code_simple.to_string()} />
                <p>{"Variables are passed as alternating key-value pairs after the message key:"}</p>
                <CodeBlock code={code_vars.to_string()} />

                <h2>{"Translation files"}</h2>
                <p>{"Fluent .ftl files live under your resource_location, organised by locale:"}</p>
                <CodeBlock code={code_ftl_dir.to_string()} />
                <p>{"Fluent supports plural forms, gender variants, and other selectors out of the box, so you can handle complex grammatical rules without hand-written conditionals:"}</p>
                <CodeBlock code={code_plural.to_string()} />

                <h2>{"Locale routing with Lang"}</h2>
                <p>{"Add a Lang parameter to your route to accept a locale prefix in the URL. Tidos picks up the active locale automatically inside page!:"}</p>
                <CodeBlock code={code_lang.to_string()} />
                <p>{"The lang parameter doubles as a Rocket request guard and as the active locale context for all i18n! calls on that page."}</p>

                <h2>{"Upgrading"}</h2>
                <p>{"No breaking changes for existing Tidos users. The i18n feature is entirely opt-in:"}</p>
                <CodeBlock code={code_upgrade.to_string()} />
            </div>
        }
    }
}
