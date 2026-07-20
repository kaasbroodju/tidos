use crate::components::install_section::InstallPill;
use tidos::{scoped_css, view, Component, Page};

use crate::components::nav_bar::NavBar;

pub struct Hero;

impl Component for Hero {
    fn to_render(&self, page: &mut Page) {
        page.add_elements_to_head(
            "tidos-title",
            String::from("<title>Tidos - SSR Component Framework for Rust</title>"),
        );
        page.add_elements_to_head(
            "google-fonts-preconnect",
            String::from(
                r#"<link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>"#,
            ),
        );
        page.add_elements_to_head(
            "google-fonts",
            String::from(
                r##"<link href="https://fonts.googleapis.com/css2?family=Newsreader:ital,wght@0,300;0,400;0,500;0,600;1,300;1,400&family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">"##,
            ),
        );
        page.add_elements_to_head(
            "global-styles",
            String::from(
                r#"<style>html,body{margin:0;padding:0}body{font-family:'Inter',system-ui,sans-serif;background:#000814}a{cursor:pointer;text-decoration:none}*{box-sizing:border-box}</style>"#,
            ),
        );

        view! {
            <section class={scoped_css!("./hero.css")}>
                // @html{include_str!("./hero.svg")}
                // <NavBar />
                <div class="hero-content">
                    <div class="copy">
                        <h1 class="headline">
                            {"Ride"}
                            <br />
                            {"the "}
                            <em>{"tide"}</em>
                        </h1>
                        <p class="tagline">
                            {"A SSR component framework for Rust that makes it simple to write fast, type-safe, expressive web UIs while playing nicely with the frameworks you already love."}
                        </p>
                        <div class="cta-group">
                            <a href="/docs/getting-started" class="btn-primary">{"Get started"}@html{include_str!("./arrow.svg")}</a>
                            <InstallPill />
                            // <a href="#" class="btn-secondary">{"Read the docs"}</a>
                        </div>
                        // <div class="version-badge">
                        //     {"Latest: "}
                        //     <span class="ver">{"v0.8.0-rc.1"}</span>
                        //     {" \u{2014} first wave released"}
                        // </div>
                    </div>
                </div>
            </section>
        }
    }
}
