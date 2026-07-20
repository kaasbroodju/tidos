use crate::components::code_block::CodeBlock;
use crate::components::docs_layout::DocsLayout;
use crate::components::nav_bar::NavBarDocs;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/docs/component")]
pub fn component_docs() -> Page {
    let mut page = page! {
        <NavBarDocs />
        <DocsLayout>
            {#slot:content}
                <ComponentDocContent />
            {/slot}
        </DocsLayout>
    };

    {
        let page = &mut page;
        head! {
            <script type="application/ld+json">@html{include_str!("./component.jsonld")}</script>
        }
    }

    page
}

struct ComponentDocContent;

impl Component for ComponentDocContent {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{"Component - Tidos"}</title>
        }

        let component_trait = "use tidos::{view, Component, Page};\n\npub struct Alert {\n    pub message: String,\n    pub kind: String,\n}\n\nimpl Component for Alert {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class=\"alert\" data-kind={&self.kind}>\n                <p>{&self.message}</p>\n            </div>\n        }\n    }\n}";

        let view_text = "let count = 42_usize;\nlet name = \"Alice\";\nlet is_loading = true;\n\nview! {\n    // --- Text content ---\n\n    // Static text\n    <p>{\"Hello world\"}</p>\n\n    // Formatted, with {} placeholders\n    <p>{\"Hello {}, you have {} items.\", name, count}</p>\n\n    // Any Rust expression\n    <p>{count.to_string()}</p>\n\n    // Raw HTML (not sanitized) — use with care!\n    <div>@html{\"<em>italic</em>\"}</div>\n\n    // --- Attributes share the exact same content syntax ---\n\n    // Static value\n    <span class={\"rounded\"}></span>\n\n    // Formatted value (here combined with scoped_css!)\n    <span class={\"rounded {}\", scoped_css!(\"./mod.css\")}></span>\n\n    // Any Rust expression\n    <div data-count={count.to_string()}></div>\n\n    // Toggle attribute: present or absent, like `disabled`.\n    // `:disabled` reads a bool; `disabled={true}` would render disabled=\"true\" instead.\n    <button :disabled={is_loading}>{\"Save\"}</button>\n}";

        let conditions = "let logged_in = true;\nlet is_admin = false;\n\nview! {\n    {#if logged_in && is_admin}\n        <span>{\"Admin Panel\"}</span>\n    {:else if logged_in}\n        <span>{\"Dashboard\"}</span>\n    {:else}\n        <a href=\"/login\">{\"Sign in\"}</a>\n    {/if}\n}";

        let loops = "let fruits = vec![\"apple\", \"banana\", \"cherry\"];\n\nview! {\n    <ul>\n        {#for fruit in &fruits}\n            <li>{fruit}</li>\n        {/for}\n    </ul>\n}";

        let pattern_match = "enum Status { Active, Banned, Guest }\nlet status = Status::Active;\n\nview! {\n    {#match status}\n        {:case Status::Active}\n            <span data-status=\"active\">{\"Active\"}</span>\n        {:case Status::Banned}\n            <span data-status=\"banned\">{\"Banned\"}</span>\n        {:case _}\n            <span data-status=\"guest\">{\"Guest\"}</span>\n    {/match}\n}";

        let slots = "// Parent\nview! {\n    <Card title={\"News\".to_string()}>\n        {#slot:body}\n            <p>{\"This content goes into the card body.\"}</p>\n        {/slot}\n    </Card>\n}\n\n// Child component\npub struct Card<'a> {\n    pub title: String,\n    pub body: Slot<'a>,\n}\n\nimpl Component for Card<'_> {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class=\"card\">\n                <h2>{&self.title}</h2>\n                <div class=\"body\">\n                    @slot{self.body}\n                </div>\n            </div>\n        }\n    }\n}";

        let default_props = "use tidos::{view, Component, Page};\n\n#[derive(Default)]\npub struct Button {\n    pub label: String,\n    pub disabled: bool,\n    pub variant: String,\n}\n\nimpl Component for Button {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <button data-variant={&self.variant}>\n                {&self.label}\n            </button>\n        }\n    }\n}\n\n// Only set label — the rest use Default::default()\nview! { <Button label={\"Save\".to_string()} .. /> }\n\n// All fields use Default\nview! { <Button .. /> }";

        let head_macro ="use tidos::{head, view, Component, Page};\n\nimpl Component for MyComponent {\n    fn to_render(&self, page: &mut Page) {\n        head! {\n            <link rel=\"stylesheet\" href=\"/fonts.css\" />\n            <script src=\"/analytics.js\"></script>\n        }\n        view! {\n            <section>{\"Content here\"}</section>\n        }\n    }\n}";

        let scoped_css = "use tidos::{scoped_css, view, Component, Page};\n\nimpl Component for Card {\n    fn to_render(&self, page: &mut Page) {\n        view! {\n            <div class={scoped_css!(\"./card.css\")}>\n                <h2>{&self.title}</h2>\n            </div>\n        }\n    }\n}\n\n// card.css\n// & {\n//     background: #1a1a2e;\n//     border-radius: 8px;\n//     padding: 1.5rem;\n// }\n// & h2 {\n//     color: #a8dadc;\n// }";

        view! {
            <div class="doc-content">
                <h1>{"Component"}</h1>
                <p>{"A component is any Rust struct that implements the Component trait. Components are the building blocks of Tidos, they encapsulate HTML structure, styles, and logic."}</p>

                <h2 id="the-view-macro">{"The view! Macro"}</h2>
                <p>{"view! is the templating macro. Use it inside the to_render function of the Component trait to write HTML with Rust expressions. All text between tags must be in {}. Attribute values use that same {} syntax — literals, formatted strings, or any expression — and a : prefix turns an attribute into a toggle that is either present or absent (like disabled), rather than set to a string value:"}</p>
                <CodeBlock code={view_text.to_string()} />

                <h2>{"The Component Trait"}</h2>
                <p>{"Implement Component on any struct. The to_render method receives a mutable Page reference so child components can inject CSS or head elements:"}</p>
                <CodeBlock code={component_trait.to_string()} />

                <h2 id="control-tags">{"Control Tags"}</h2>
                <p>{"Control tags are special blocks inside view! that map directly to Rust control flow."}</p>

                <h3 id="control-tags-conditions">{"Conditions"}</h3>
                <CodeBlock code={conditions.to_string()} />

                <h3 id="control-tags-loops">{"Loops"}</h3>
                <CodeBlock code={loops.to_string()} />

                <h3 id="control-tags-pattern-matching">{"Pattern Matching"}</h3>
                <CodeBlock code={pattern_match.to_string()} />

                <h3 id="control-tags-slots">{"Slots"}</h3>
                <p>{"Named slots let a parent pass rendered HTML content into a child component. Use {#slot:name}...{/slot} in the parent and @slot{self.name} in the child:"}</p>
                <CodeBlock code={slots.to_string()} />

                <h2 id="default-props">{"Default Properties"}</h2>
                <p>{"Components whose struct derives Default can use .. after the explicit props to fill all remaining fields with their default values. The struct must implement Default, either via #[derive(Default)] or manually:"}</p>
                <CodeBlock code={default_props.to_string()} />

                <h2 id="the-head-macro">{"The head! Macro"}</h2>
                <p>{"Call head! to inject elements into the page head. The call is deduplicated by a compile-time UUID, so it is safe to call from components rendered in loops:"}</p>
                <CodeBlock code={head_macro.to_string()} />

                <h2 id="the-scoped-css-macro">{"The scoped_css! Macro"}</h2>
                <p>{"scoped_css! reads a CSS file at compile time, generates a unique class name, and injects a scoped style tag into the head. Apply the returned class to your root element:"}</p>
                <CodeBlock code={scoped_css.to_string()} />
            </div>
        }
    }
}
