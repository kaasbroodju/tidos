use crate::components::code_block::CodeBlock;
use crate::components::docs_layout::DocsLayout;
use crate::components::framework_tabs::FrameworkTabs;
use crate::components::nav_bar::NavBarDocs;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/docs/getting-started?<framework>")]
pub fn getting_started(framework: Option<String>) -> Page {
    let active = framework.unwrap_or_else(|| String::from("Rocket"));

    let mut page = page! {
        <NavBarDocs />
        <DocsLayout>
            {#slot:content}
                <GettingStartedContent active={active.clone()} />
            {/slot}
        </DocsLayout>
    };

    {
        let page = &mut page;
        head! {
            <script type="application/ld+json">@html{include_str!("./getting_started.jsonld")}</script>
        }
    }

    page
}

struct FwDocs {
    name: &'static str,
    cargo: &'static str,
    code: &'static str,
}

struct GettingStartedContent {
    pub active: String,
}

impl Component for GettingStartedContent {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{"Getting Started - Tidos"}</title>
        }

        let frameworks = vec![
            FwDocs {
                name: "Rocket",
                cargo: "[dependencies]\ntidos = { version = \"0.8.0-rc.1\", features = [\"rocket\"] }\nrocket = \"0.5\"",
                code: "use tidos::{page, Page};\nuse rocket::{get, routes};\n\n#[get(\"/\")]\npub fn index() -> Page {\n    page! {\n        <main>\n            <h1>{\"Hello from Tidos!\"}</h1>\n        </main>\n    }\n}\n\n#[rocket::main]\nasync fn main() {\n    rocket::build()\n        .mount(\"/\", routes![index])\n        .launch()\n        .await\n        .unwrap();\n}",
            },
            FwDocs {
                name: "Actix",
                cargo: "[dependencies]\ntidos = { version = \"0.8.0-rc.1\", features = [\"actix-web\"] }\nactix-web = \"4\"",
                code: "use actix_web::{get, App, HttpServer, HttpResponse};\nuse tidos::{page, Page};\n\n#[get(\"/\")]\nasync fn index() -> HttpResponse {\n    let page: Page = page! {\n        <main>\n            <h1>{\"Hello from Tidos!\"}</h1>\n        </main>\n    };\n    HttpResponse::Ok()\n        .content_type(\"text/html; charset=utf-8\")\n        .body(page.to_string())\n}\n\n#[actix_web::main]\nasync fn main() -> std::io::Result<()> {\n    HttpServer::new(|| App::new().service(index))\n        .bind(\"127.0.0.1:8080\")?\n        .run()\n        .await\n}",
            },
            FwDocs {
                name: "Axum",
                cargo: "[dependencies]\ntidos = { version = \"0.8.0-rc.1\", features = [\"axum\"] }\naxum = \"0.8\"\ntokio = { version = \"1\", features = [\"full\"] }",
                code: "use axum::{response::IntoResponse, routing::get, Router};\nuse tidos::{page, Page};\n\nasync fn index() -> impl IntoResponse {\n    page! {\n        <main>\n            <h1>{\"Hello from Tidos!\"}</h1>\n        </main>\n    }\n}\n\n#[tokio::main]\nasync fn main() {\n    let app = Router::new().route(\"/\", get(index));\n    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await.unwrap();\n    axum::serve(listener, app).await.unwrap();\n}",
            },
            FwDocs {
                name: "Warp",
                cargo: "[dependencies]\ntidos = { version = \"0.8.0-rc.1\", features = [\"warp\"] }\nwarp = \"0.4\"\ntokio = { version = \"1\", features = [\"full\"] }",
                code: "use warp::Filter;\nuse tidos::{page, Page};\n\n#[tokio::main]\nasync fn main() {\n    let route = warp::get()\n        .and(warp::path::end())\n        .map(|| {\n            warp::reply::html(page! {\n                <main>\n                    <h1>{\"Hello from Tidos!\"}</h1>\n                </main>\n            }.to_string())\n        });\n    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;\n}",
            },
            FwDocs {
                name: "Standalone",
                cargo: "[dependencies]\ntidos = \"0.8.0-rc.1\"",
                code: "use tidos::{page, Page};\n\nfn main() {\n    let html: Page = page! {\n        <main>\n            <h1>{\"Hello from Tidos!\"}</h1>\n        </main>\n    };\n    println!(\"{}\", html);\n}",
            },
        ];

        let active = self.active.as_str();
        let frameworks_str = String::from("Rocket,Actix,Axum,Warp,Standalone");
        let active_clone = self.active.clone();

        let dev_loop = "# install once\ncargo install watchexec-cli\n\n# rebuild & restart on every save\nwatchexec -r -e rs -- cargo run\n\n# with multiple binaries, name the one to run\nwatchexec -r -e rs -- cargo run --bin my_app";

        view! {
            <div class="doc-content">
                <h1>{"Getting Started"}</h1>
                <p>{"Tidos is a SSR component framework. It lets you write HTML components directly in Rust. Tidos works with your favourite frameworks."}</p>
                <FrameworkTabs frameworks={frameworks_str} active={active_clone} />
                <h2>{"1. Add to Cargo.toml"}</h2>
                {#for fw in &frameworks}
                    <div data-framework={fw.name} style={if fw.name == active { "display:block" } else { "display:none" }}>
                        <CodeBlock code={fw.cargo.to_string()} />
                    </div>
                {/for}
                <h2>{"2. Create your first route"}</h2>
                <p>{"Use the page! macro inside a route handler to return a rendered HTML page:"}</p>
                {#for fw in &frameworks}
                    <div data-framework={fw.name} style={if fw.name == active { "display:block" } else { "display:none" }}>
                        <CodeBlock code={fw.code.to_string()} />
                    </div>
                {/for}
                <h2>{"Live reloading during development"}</h2>
                <p>{"Rust has no native hot module reloading, so retyping cargo run after every change gets tedious. We recommend using "}<a href="https://github.com/watchexec/watchexec">{"Watchexec"}</a>{" as it rebuilds and restarts your server automatically whenever you save a file. We use it for the Tidos documentation website itself."}</p>
                <CodeBlock code={dev_loop.to_string()} />

                <h2>{"Next steps"}</h2>
                <p>{"You now have a page rendering. Next, learn how to build reusable "}<a href="/docs/component">{"Components"}</a>{" with the view! macro, control flow, slots, and scoped CSS."}</p>
            </div>
        }
    }
}
