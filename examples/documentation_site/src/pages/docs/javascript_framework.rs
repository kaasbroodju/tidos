use crate::components::code_block::CodeBlock;
use crate::components::docs_layout::DocsLayout;
use crate::components::framework_tabs::FrameworkTabs;
use crate::components::nav_bar::NavBarDocs;
use rocket::get;
use tidos::{head, native_element, page, view, Component, Page};

#[get("/docs/javascript-framework?<framework>")]
pub fn javascript_framework(framework: Option<String>) -> Page {
    let active = framework.unwrap_or_else(|| String::from("Svelte"));

    let mut page = page! {
        <NavBarDocs />
        <DocsLayout>
            {#slot:content}
                <JsFrameworkContent active={active.clone()} />
            {/slot}
        </DocsLayout>
    };

    {
        let page = &mut page;
        head! {
            <script type="application/ld+json">@html{include_str!("./javascript_framework.jsonld")}</script>
        }
    }

    page
}

struct JsFwDocs {
    name: &'static str,
    code: &'static str,
    vite_config: &'static str,
}

struct JsFrameworkContent {
    pub active: String,
}

#[native_element]
pub struct GreetUser {
    pub name: String,
    pub is_admin: bool,
}

impl Component for JsFrameworkContent {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{"Reactivity - Tidos"}</title>
        }
        
        let native_element_rs = "use tidos::native_element;\n\n// GreetUser -> /dist/GreetUser.js -> <greet-user>\n#[native_element]\npub struct GreetUser {\n    pub name: String,\n    pub is_admin: bool,\n}\n\n// Output of macro\nimpl tidos::Component for GreetUser {\n    fn to_render(&self, page: &mut tidos::Page) {\n        tidos::head!{ <script r#type=\"module\" src=\"/dist/GreetUser.js\"></script> };\n        tidos::view!{ <greet-user name={self.name} :is-admin={self.is_admin}></greet-user> };\n    }\n}";

        let serve_dist = "// Rocket — serve compiled JS from the dist folder\n#[get(\"/dist/<file..>\")]\nasync fn dist_files(file: std::path::PathBuf) -> Option<rocket::fs::NamedFile> {\n    rocket::fs::NamedFile::open(std::path::Path::new(\"dist/\").join(file)).await.ok()\n}";

        // The full, working Vite configs, pulled straight from the example
        // projects so they stay in sync and can be copy-pasted as-is.
        let vite_svelte = include_str!("../../../../web_frameworks/svelte/vite.config.js");
        let vite_lit = include_str!("../../../../web_frameworks/lit/vite.config.js");

        let vite_angular = include_str!("../../../../web_frameworks/angular/vite.config.js");

        let vite_vue = include_str!("../../../../web_frameworks/vue/vite.config.js");

        let vite_react = include_str!("../../../../web_frameworks/react/vite.config.js");

        let frameworks = vec![
            JsFwDocs {
                name: "Svelte",
                code: "<!-- src/components/GreetUser.svelte -->\n<svelte:options customElement=\"greet-user\"></svelte:options>\n\n<script>\n    let { name = '', is_admin = false } = $props();\n</script>\n\n<div>\n    <p>Hello, {name}!</p>\n    {#if is_admin}\n        <span class=\"badge\">Admin</span>\n    {/if}\n</div>\n\n<style>\n    :host { display: block; }\n    .badge { background: #4fd1c5; color: #000; padding: 0.2rem 0.5rem; border-radius: 4px; }\n</style>",
                vite_config: vite_svelte,
            },
            JsFwDocs {
                name: "Lit",
                code: "// src/components/GreetUser.ts\nimport { LitElement, html } from 'lit';\nimport { customElement, property } from 'lit/decorators.js';\n\n@customElement('greet-user')\nexport class GreetUser extends LitElement {\n    @property() name = '';\n    @property({ type: Boolean }) isAdmin = false;\n\n    render() {\n        return html`\n            <p>Hello, ${this.name}!</p>\n            ${this.isAdmin ? html`<span class=\"badge\">Admin</span>` : ''}\n        `;\n    }\n}",
                vite_config: vite_lit,
            },
            JsFwDocs {
                name: "Angular",
                code: "// app.module.ts — register as Angular Element\nimport { createCustomElement } from '@angular/elements';\nimport { GreetUserComponent } from './greet-user.component';\n\n@NgModule({ ... })\nexport class AppModule {\n    constructor(private injector: Injector) {}\n    ngDoBootstrap() {\n        const el = createCustomElement(GreetUserComponent, { injector: this.injector });\n        customElements.define('greet-user', el);\n    }\n}",
                vite_config: vite_angular,
            },
            JsFwDocs {
                name: "Vue",
                code: "<!-- src/components/GreetUser.vue -->\n<template>\n    <div>\n        <p>Hello, {{ name }}!</p>\n        <span v-if=\"isAdmin\" class=\"badge\">Admin</span>\n    </div>\n</template>\n\n<script setup>\ndefineProps({\n    name: { type: String, default: '' },\n    isAdmin: { type: Boolean, default: false },\n});\n</script>\n\n<!-- src/components/GreetUser.js — wrapper that registers the SFC as a custom element -->\nimport { defineCustomElement } from 'vue';\nimport GreetUser from './GreetUser.vue';\n\ncustomElements.define('greet-user', defineCustomElement(GreetUser));",
                vite_config: vite_vue,
            },
            JsFwDocs {
                name: "React",
                code: "// src/components/GreetUser.tsx\nimport { createRoot } from 'react-dom/client';\n\nclass GreetUser extends HTMLElement {\n    connectedCallback() {\n        const name = this.getAttribute('name') || '';\n        const isAdmin = this.hasAttribute('is-admin');\n        const root = createRoot(this);\n        root.render(\n            <div>\n                <p>Hello, {name}!</p>\n                {isAdmin && <span className=\"badge\">Admin</span>}\n            </div>\n        );\n    }\n}\n\ncustomElements.define('greet-user', GreetUser);",
                vite_config: vite_react,
            },
        ];

        let active = self.active.as_str();
        let frameworks_str = String::from("Svelte,Lit,Angular,Vue,React");
        let active_clone = self.active.clone();

        view! {
            <div class="doc-content">
                <h1>{"Reactivity"}</h1>
                <p>{"Tidos renders HTML on the server. For client-side interactivity, compile any JavaScript framework component as a "}<a href="https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements">{"Web Component"}</a>{" and embed it using #[native_element]."}</p>

                <h2>{"How It Works"}</h2>
                <ol>
                    <li>{"Write your component in the JS framework of your choice."}</li>
                    <li>{"Compile it as a custom element to dist/ComponentName.js."}</li>
                    <li>{"Create a Rust struct with #[native_element] to create a type safe wrapper."}</li>
                    <li>{"Use it in page! or view! like any other Tidos component."}</li>
                </ol>

                <h2>{"The Rust Wrapper"}</h2>
                <p>{"#[native_element] derives Component for the struct. It injects the script tag and renders the kebab-case custom element with all fields forwarded as HTML attributes:"}</p>
                <CodeBlock code={native_element_rs.to_string()} />

                <h2>{"Framework Example"}</h2>
                <FrameworkTabs frameworks={frameworks_str} active={active_clone} />
                {#for fw in &frameworks}
                    <div data-framework={fw.name} style={if fw.name == active { "display:block" } else { "display:none" }}>
                        <CodeBlock code={fw.code.to_string()} />
                    </div>
                {/for}

                <h2>{"Build Config"}</h2>
                <p>{"Compile the component to dist/ComponentName.js using Vite:"}</p>
                {#for fw in &frameworks}
                    <div data-framework={fw.name} style={if fw.name == active { "display:block" } else { "display:none" }}>
                        <CodeBlock code={fw.vite_config.to_string()} />
                    </div>
                {/for}

                <h2>{"Serving Built Files"}</h2>
                <p>{"Add a route in your Rust server to serve files from the dist directory:"}</p>
                <CodeBlock code={serve_dist.to_string()} />
            </div>
        }
    }
}
