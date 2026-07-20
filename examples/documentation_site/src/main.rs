#![recursion_limit = "512"]

mod cache_control;
mod components;
mod pages;
mod server_timing;

use std::path::PathBuf;

use rocket::fs::{FileServer, NamedFile};
use rocket::routes;

use crate::pages::docs::component::component_docs;
use crate::pages::docs::getting_started::getting_started;
use crate::pages::docs::internationalization::internationalization;
use crate::pages::docs::javascript_framework::javascript_framework;
use crate::pages::docs::the_page_macro::the_page_macro;
use crate::pages::index::index;
use crate::pages::news::news_index;
use crate::pages::news::v0_7_0::news_v0_7_0;
use crate::pages::news::v0_7_5::news_v0_7_5;
use crate::pages::news::v0_7_6::news_v0_7_6;
use crate::pages::news::v0_8_0_rc1::news_v0_8_0_rc1;

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(server_timing::ServerTimingFairing)
        .attach(cache_control::CacheControlFairing)
        .mount(
            "/",
            routes![
                index,
                getting_started,
                the_page_macro,
                component_docs,
                javascript_framework,
                internationalization,
                news_index,
                news_v0_8_0_rc1,
                news_v0_7_6,
                news_v0_7_5,
                news_v0_7_0,
            ],
        )
        .mount("/dist", FileServer::from("./dist"))
        .mount("/", FileServer::from("./public").rank(1))
        .launch()
        .await
        .unwrap();

}
