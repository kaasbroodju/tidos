pub mod v0_7_0;
pub mod v0_7_5;
pub mod v0_7_6;
pub mod v0_8_0_rc1;
pub mod v0_8_0_rc2;

use crate::components::nav_bar::NavBarDocs;
use crate::components::news_layout::NewsLayout;
use rocket::get;
use tidos::{head, page, view, Component, Page};

#[get("/news")]
pub fn news_index() -> Page {
    let mut page = page! {
        <NavBarDocs />
        <NewsLayout>
            {#slot:content}
                <NewsIndexContent />
            {/slot}
        </NewsLayout>
    };

    {
        let page = &mut page;
        head! {
            <title>{"News - Tidos"}</title>
            <meta name="description" content="Latest releases and updates from the Tidos Rust SSR component framework." />
        }
    }

    page
}

struct NewsIndexContent;

impl Component for NewsIndexContent {
    fn to_render(&self, page: &mut Page) {
        view! {
            <h1 class="news-index-title">{"News"}</h1>
            <p class="news-intro">{"Latest releases and updates from the Tidos project."}</p>

            <div class="news-list">
                <a class="news-card" href="/news/v0-8-0-rc2">
                    <div class="news-card-header">
                        <span class="news-version">{"v0.8.0-rc.2"}</span>
                        <span class="news-date">{"July 20, 2026"}</span>
                    </div>
                    <h2>{"Components Render Directly Into Page"}</h2>
                    <p>{"to_render no longer returns a String, attributes gain the full text-content syntax, Slot's lifetime is fixed so it can finally borrow, and components can now take a single unnamed slot."}</p>
                </a>

                <a class="news-card" href="/news/v0-8-0-rc1">
                    <div class="news-card-header">
                        <span class="news-version">{"v0.8.0-rc.1"}</span>
                        <span class="news-date">{"April 8, 2026"}</span>
                    </div>
                    <h2>{"Smarter HTML Generation"}</h2>
                    <p>{"Static HTML segments are now merged at compile time, eliminating runtime string allocations. Includes a breaking change: all text in view! must be wrapped in curly braces."}</p>
                </a>

                <a class="news-card" href="/news/v0-7-6">
                    <div class="news-card-header">
                        <span class="news-version">{"v0.7.6"}</span>
                        <span class="news-date">{"April 5, 2026"}</span>
                    </div>
                    <h2>{"Default Props"}</h2>
                    <p>{"Components that implement the Default trait can now use .. syntax to fill unspecified props with their default values, making optional configuration easier."}</p>
                </a>

                <a class="news-card" href="/news/v0-7-5">
                    <div class="news-card-header">
                        <span class="news-version">{"v0.7.5"}</span>
                        <span class="news-date">{"April 5, 2026"}</span>
                    </div>
                    <h2>{"Expanded Framework Support"}</h2>
                    <p>{"Feature flags for Axum, Actix-web, and Warp. New native_element attribute macro for custom element wrappers, plus examples for Svelte, Lit, Vue, React, and Angular."}</p>
                </a>

                <a class="news-card" href="/news/v0-7-0">
                    <div class="news-card-header">
                        <span class="news-version">{"v0.7.0"}</span>
                        <span class="news-date">{"February 19, 2026"}</span>
                    </div>
                    <h2>{"Internationalization"}</h2>
                    <p>{"Built-in i18n support powered by Fluent. Translate your Tidos pages with the i18n! macro, locale routing, and .ftl translation files."}</p>
                </a>
            </div>
        }
    }
}
