mod components;

use crate::components::greeting::Greeting;
use rocket::{get, routes};
use tidos::{Component, Page, page};
use tidos::i18n::{Lang, enable_i18n};

#[get("/<lang>")]
pub fn index(lang: Lang) -> Page {
	page! {
		<main>
			<Greeting />
		</main>
	}
}

enable_i18n!();
#[rocket::main]
async fn main() {
	rocket::build()
		.mount("/", routes![index])
		.launch()
		.await
		.unwrap();
}