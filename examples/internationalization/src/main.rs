mod components;

use crate::components::greeting::Greeting;
use std::{fs, io, vec};
use std::path::PathBuf;
use rocket::{get, routes};
use unic_langid::LanguageIdentifier;
use tidos::{Component, Page, page, view};
use tidos_i18n::{enable_i18n, i18n};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use fluent_resmgr::resource_manager::ResourceManager;
use lazy_static::lazy_static;

#[get("/")]
pub fn index() -> Page {
	page! {
		<main>
			<Greeting />
		</main>
	}
}

enable_i18n!("resources", "en-US", "common.ftl");
#[rocket::main]
async fn main() {
	rocket::build()
		.mount("/", routes![index])
		.launch()
		.await
		.unwrap();
}