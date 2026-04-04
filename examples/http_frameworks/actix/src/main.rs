mod components;

use actix_web::{web, App, HttpServer};
use components::leaderboard::Leaderboard;
use components::nav_bar::NavBar;
use tidos::{page, scoped_css, Component, Page};

async fn index() -> Page {
    let nav_links = vec![
        (String::from("Home"), String::from("/")),
        (String::from("Rust"), String::from("https://www.rust-lang.org")),
        (String::from("Tidos"), String::from("https://crates.io/crates/tidos")),
    ];

    page! {
        <NavBar title={String::from("🦀 Ferris Fan Club")} links={nav_links} />
        <main class={scoped_css!("./main.css")}>
            <h1>Welcome, Rustacean!</h1>
            <p>
                The premier leaderboard for crab enthusiasts worldwide.
            </p>
            <Leaderboard />
        </main>
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
