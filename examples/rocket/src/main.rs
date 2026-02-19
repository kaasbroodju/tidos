mod components;

use components::leaderboard::Leaderboard;
use components::nav_bar::NavBar;
use rocket::{get, routes};
use tidos::{page, scoped_css, Component, Page};

#[get("/")]
pub fn index() -> Page {
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

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .unwrap();
}
