mod components;

use components::leaderboard::Leaderboard;
use components::nav_bar::NavBar;
use tidos::{page, scoped_css, Component, Page};
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::get().and(warp::path::end()).map(|| {
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
    });

    warp::serve(route).run(([0, 0, 0, 0], 8000)).await;
}
