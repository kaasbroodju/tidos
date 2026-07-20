use crate::components::nav_bar::NavBar;
use crate::components::code_section::CodeSection;
use crate::components::features_section::FeaturesSection;
use crate::components::hero::Hero;
use crate::components::install_section::InstallSection;
use crate::components::site_footer::SiteFooter;
use rocket::get;
use tidos::{head, page, Page};

#[get("/")]
pub fn index() -> Page {
    let mut page = page! {
        <header>
            <NavBar />
        </header>
        <main>
            <Hero />
            <FeaturesSection />
            <CodeSection />
            
        </main>
        <SiteFooter />
    };

    {
        let page = &mut page;
        let description = "A SSR component framework for Rust that makes it simple to write fast, type-safe, expressive web UIs while playing nicely with the frameworks you already love.";
        head! {
            <meta name="description" content={description} />

            <meta property="og:type" content="website" />
            <meta property="og:site_name" content="Tidos" />
            <meta property="og:title" content="Tidos — Ride the tide." />
            <meta property="og:description" content={description} />
            <meta property="og:url" content="https://tidos.dev/" />
            <meta property="og:image" content="https://tidos.dev/inko.png" />
            <meta property="og:image:width" content="1000" />
            <meta property="og:image:height" content="1000" />
            <meta property="og:image:alt" content="Tidos — the Inko mascot" />

            <meta name="twitter:card" content="summary" />
            <meta name="twitter:title" content="Tidos — Ride the tide." />
            <meta name="twitter:description" content={description} />
            <meta name="twitter:image" content="https://tidos.dev/inko.png" />

            <link rel="preconnect" href="https://fonts.googleapis.com" />
            <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
            <link href="https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" rel="stylesheet" />

            <style>@html{"html,body{margin:0;padding:0;color:#fff;background-color: #0d1f2a;font-family:'Roboto'}"}</style>
            <link rel="stylesheet" href="/transitions.css" />
            <script src="/transitions.js"></script>
            <script type="application/ld+json">@html{include_str!("./index.jsonld")}</script>

            <link rel="prefetch" href="/docs/getting-started" />
            <script type="speculationrules">@html{r#"{"prerender":[{"urls":["/docs/getting-started"]}]}"#}</script>
        }
    }
    page
}