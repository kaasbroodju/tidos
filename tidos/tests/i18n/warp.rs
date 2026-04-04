use hyper::body::to_bytes;
use tidos::i18n::Lang;
use tidos::{i18n::i18n, page, Page};
use warp::http::StatusCode;
use warp::reply::Reply;

fn make_page(lang: Lang) -> Page {
	page! { <p>{i18n!("greeting")}</p> }
}

async fn body_string(page: Page) -> String {
	let bytes = to_bytes(page.into_response().into_body()).await.unwrap();
	String::from_utf8(bytes.to_vec()).unwrap()
}

#[tokio::test]
async fn response_status_ok() {
	let lang: Lang = "en-US".parse().unwrap();
	let response = make_page(lang).into_response();
	assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn response_content_type_is_html() {
	let lang: Lang = "en-US".parse().unwrap();
	let response = make_page(lang).into_response();
	assert_eq!(
		response.headers()["content-type"],
		"text/html; charset=utf-8"
	);
}

#[tokio::test]
async fn response_body_contains_lang_en() {
	let lang: Lang = "en-US".parse().unwrap();
	let html = body_string(make_page(lang)).await;
	assert!(html.contains(r#"<html lang="en-US">"#));
}

#[tokio::test]
async fn response_body_contains_lang_nl() {
	let lang: Lang = "nl-NL".parse().unwrap();
	let html = body_string(make_page(lang)).await;
	assert!(html.contains(r#"<html lang="nl-NL">"#));
}

#[tokio::test]
async fn response_body_contains_translated_content() {
	let lang: Lang = "en-US".parse().unwrap();
	let html = body_string(make_page(lang)).await;
	assert!(html.contains("Hello"));
}

#[tokio::test]
async fn response_body_translated_nl() {
	let lang: Lang = "nl-NL".parse().unwrap();
	let html = body_string(make_page(lang)).await;
	assert!(html.contains("Hallo"));
}
