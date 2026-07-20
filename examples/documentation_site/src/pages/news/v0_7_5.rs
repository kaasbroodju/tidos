use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-7-5")]
pub fn news_v0_7_5() -> Page {
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
            <title>{"v0.7.5: Expanded Framework Support - Tidos News"}</title>
            <meta name="description" content="Feature flags for Axum, Actix-web, and Warp. New native_element macro and JS framework examples." />
        }
    }

    page
}

struct ArticleContent;

impl Component for ArticleContent {
    fn to_render(&self, page: &mut Page) {
        let code_cargo_frameworks = "# Axum\ntidos = { version = \"0.7.5\", features = [\"axum\"] }\n\n# Actix-web\ntidos = { version = \"0.7.5\", features = [\"actix-web\"] }\n\n# Warp\ntidos = { version = \"0.7.5\", features = [\"warp\"] }\n\n# Rocket (unchanged)\ntidos = { version = \"0.7.5\", features = [\"rocket\"] }";
        let code_native_element = "use tidos::native_element;\n\n#[native_element]\npub struct MyCounter {\n    pub initial: i32,\n    pub step: i32,\n}\n\n// Renders: <my-counter initial=\"0\" step=\"1\"></my-counter>\n// and injects: <script src=\"/dist/my-counter.js\"></script>";
        let code_cargo = "# Feature flag for your framework of choice\ntidos = { version = \"0.7.5\", features = [\"rocket\"] }";

        view! {
            <div class="article-content">
                <a class="back-link" href="/news">{"← All news"}</a>

                <h1>{"Expanded Framework Support"}</h1>
                <div class="article-meta">
                    <span class="article-version">{"v0.7.5"}</span>
                    <span>{"April 5, 2026"}</span>
                </div>

                <p>{"v0.7.5 broadens Tidos beyond Rocket, adds the native_element macro for custom HTML elements, and ships working examples for five JavaScript frameworks."}</p>

                <h2>{"Axum, Actix-web, and Warp"}</h2>
                <p>{"Tidos now ships feature flags for three additional HTTP frameworks. Add the flag for your framework of choice to Cargo.toml:"}</p>
                <CodeBlock code={code_cargo_frameworks.to_string()} />
                <p>{"Each flag implements the appropriate response trait so a Page can be returned directly from a route handler. See the Getting Started page for full examples."}</p>

                <h2>{"The #[native_element] macro"}</h2>
                <p>{"Tidos now ships a #[native_element] attribute macro that generates a Component implementation for any struct that wraps a custom HTML element. It injects the required script tag, renders the kebab-case element name, and maps every struct field to an HTML attribute:"}</p>
                <CodeBlock code={code_native_element.to_string()} />
                <p>{"This bridges the gap between Tidos SSR and client-side web components: define the component once in Rust and let the browser hydrate it."}</p>

                <h2>{"JavaScript framework examples"}</h2>
                <p>{"The repository now includes complete working examples showing how to pair Tidos SSR with the most popular JS frameworks for client-side interactivity:"}</p>
                <ul>
                    <li>{"Svelte — embed compiled Svelte components via native_element"}</li>
                    <li>{"Lit Element — lightweight web components with reactive properties"}</li>
                    <li>{"Vue — Vue custom elements with defineCustomElement"}</li>
                    <li>{"React — React components mounted inside a custom element shell"}</li>
                    <li>{"Angular — Angular elements via createCustomElement"}</li>
                </ul>
                <p>{"Each example lives under examples/ in the repository and shows a full Cargo.toml, route handler, and client bundle setup."}</p>

                <h2>{"Upgrading"}</h2>
                <p>{"No breaking changes. Switch the feature flag in your Cargo.toml to the framework you use:"}</p>
                <CodeBlock code={code_cargo.to_string()} />
            </div>
        }
    }
}
