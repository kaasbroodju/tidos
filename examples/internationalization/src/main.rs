mod components;

use crate::components::greeting::Greeting;
use rocket::{get, routes};
use tidos::{Component, Page, page, enable_i18n, Lang};

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