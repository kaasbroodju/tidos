use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use tidos::i18n::Lang;
use tidos::{i18n::i18n, page, Page};

#[rocket::get("/<lang>")]
fn index(lang: Lang) -> Page {
	page! { <p>{i18n!("greeting")}</p> }
}

fn client() -> Client {
	let rocket = rocket::build().mount("/", rocket::routes![index]);
	Client::tracked(rocket).unwrap()
}

#[test]
fn response_status_ok() {
	let client = client();
	let response = client.get("/en-US").dispatch();
	assert_eq!(response.status(), Status::Ok);
}

#[test]
fn response_content_type_is_html() {
	let client = client();
	let response = client.get("/en-US").dispatch();
	assert_eq!(response.content_type(), Some(ContentType::HTML));
}

#[test]
fn response_body_contains_lang_en() {
	let client = client();
	let body = client.get("/en-US").dispatch().into_string().unwrap();
	assert!(body.contains(r#"<html lang="en-US">"#));
}

#[test]
fn response_body_contains_lang_nl() {
	let client = client();
	let body = client.get("/nl-NL").dispatch().into_string().unwrap();
	assert!(body.contains(r#"<html lang="nl-NL">"#));
}

#[test]
fn response_body_contains_translated_content() {
	let client = client();
	let body = client.get("/en-US").dispatch().into_string().unwrap();
	assert!(body.contains("Hello"));
}

#[test]
fn response_body_translated_nl() {
	let client = client();
	let body = client.get("/nl-NL").dispatch().into_string().unwrap();
	assert!(body.contains("Hallo"));
}
