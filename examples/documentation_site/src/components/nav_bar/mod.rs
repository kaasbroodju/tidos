use tidos::{head, scoped_css, view, Component, Page};

use crate::components::mobile_menu::MobileMenu;

pub struct NavBar;

impl Component for NavBar {
    fn to_render(&self, page: &mut Page) {
        head! {
            <link rel="icon" type="image/svg+xml" href="/inko.svg" />
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet" />
        }
        view! {
            <header class={scoped_css!("./nav_bar.css")}>
                <div class="logo">
                    <span class="inko"><img src="/inko.svg" alt={""} /></span>
                    <span class="name">{"Tidos"}</span>
                </div>
                <MobileMenu controls={String::from("main-nav")} label={String::from("Toggle navigation menu")} />
                <nav id="main-nav">
                    <a href="/docs/getting-started">{"documentation"}</a>
                    <a href="/news">{"news"}</a>
                    <a href="https://github.com/kaasbroodju/tidos" aria-label="Tidos Github repository" class="github-link" target="_blank"><span>@html{include_str!("./github.svg")}</span><span>{"v0.8.0-rc.1"}</span></a>
                </nav>
            </header>
        }
    }
}

pub struct NavBarDocs;

impl Component for NavBarDocs {
    fn to_render(&self, page: &mut Page) {
        head! {
            <link rel="icon" type="image/svg+xml" href="/inko.svg" />
            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet" />
        }
        view! {
            <header class={scoped_css!("./nav_bar_docs.css")} style="background-color: #0d1f2a">
                <a class="logo" href="/">
                    <span class="inko"><img src="/inko.svg" alt={""} /></span>
                    <span class="name">{"Tidos"}</span>
                </a>
                <MobileMenu controls={String::from("docs-nav")} label={String::from("Toggle navigation menu")} />
                <nav id="docs-nav">
                    <a href="/docs/getting-started">{"documentation"}</a>
                    <a href="/news">{"news"}</a>
                    <a href="https://github.com/kaasbroodju/tidos" aria-label="Tidos Github repository" class="github-link" target="_blank"><span>@html{include_str!("./github.svg")}</span><span>{"v0.8.0-rc.1"}</span></a>
                </nav>
            </header>
        }
    }
}
