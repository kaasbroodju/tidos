use crate::components::code_block::CodeBlock;
use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news/v0-8-0-rc1")]
pub fn news_v0_8_0_rc1() -> Page {
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
            <title>{"v0.8.0-rc.1: Smarter HTML Generation - Tidos News"}</title>
            <meta name="description" content="Static HTML segments are now merged at compile time, eliminating runtime allocations. First release candidate for v0.8.0." />
        }
    }

    page
}

struct ArticleContent;

impl Component for ArticleContent {
    fn to_render(&self, page: &mut Page) {
        let code_breaking = "// Before (no longer compiles)\nview! { <p>Hello world</p> }\n\n// After\nview! { <p>{\"Hello world\"}</p> }";
        let code_expr = "view! {\n    <p>{\"Hello {}\", name}</p>   // formatted\n    <p>{name}</p>               // expression\n}";
        let code_cargo = "[dependencies]\ntidos = \"0.8.0-rc.1\"";

        view! {
            <div class="article-content">
                <a class="back-link" href="/news">{"← All news"}</a>

                <h1>{"Smarter HTML Generation"}</h1>
                <div class="article-meta">
                    <span class="article-version">{"v0.8.0-rc.1"}</span>
                    <span>{"April 8, 2026"}</span>
                    <span>{"First release candidate"}</span>
                </div>

                <p>{"v0.8.0-rc.1 is the first release candidate on the road to v0.8.0. It focuses on eliminating unnecessary runtime work by moving string concatenation to compile time."}</p>

                <h2>{"Compile-time string concatenation"}</h2>
                <p>{"Tidos now merges adjacent static HTML segments at compile time. Previously, every HTML tag and text node was concatenated at runtime using String::push_str calls. Now, consecutive static strings are folded into a single string literal by the macro, so the generated code produces fewer allocations and runs faster."}</p>
                <p>{"This is a purely internal improvement — your component code does not change. Pages with many static elements will see the most benefit."}</p>

                <h2>{"Breaking change: text must be wrapped in curly braces"}<span class="breaking-badge">{"Breaking"}</span></h2>
                <p>{"To enable compile-time safety and the new optimisation, all text content inside view! must now be explicitly wrapped in curly braces. Raw text between tags is no longer allowed:"}</p>
                <CodeBlock code={code_breaking.to_string()} />
                <p>{"The macro panics at compile time if it encounters unwrapped text, so migration errors are caught immediately. The fix is always mechanical: wrap the text in double-quoted curly braces."}</p>
                <p>{"If you have variables or expressions, keep using the expression form:"}</p>
                <CodeBlock code={code_expr.to_string()} />

                <h2>{"Upgrading"}</h2>
                <p>{"Update your Cargo.toml to use the release candidate:"}</p>
                <CodeBlock code={code_cargo.to_string()} />
                <p>{"Then search your codebase for raw text between HTML tags and wrap each occurrence in curly braces. Most editors can do this with a regex find-and-replace."}</p>
            </div>
        }
    }
}
