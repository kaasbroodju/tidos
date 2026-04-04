use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use actix_web::test as actweb_test;
use actix_web::{App, Responder};
use tidos::i18n::Lang;
use tidos::{i18n::i18n, page, Page};

fn make_page(lang: Lang) -> Page {
	page! { <p>{i18n!("greeting")}</p> }
}

fn body_string(page: Page) -> String {
	let req = actweb_test::TestRequest::default().to_http_request();
	let response = page.respond_to(&req);
	let bytes = response.into_body().try_into_bytes().unwrap();
	String::from_utf8(bytes.to_vec()).unwrap()
}

// --- Responder ---

#[test]
fn response_status_ok() {
	let lang: Lang = "en-US".parse().unwrap();
	let req = actweb_test::TestRequest::default().to_http_request();
	let response = make_page(lang).respond_to(&req);
	assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn response_content_type_is_html() {
	let lang: Lang = "en-US".parse().unwrap();
	let req = actweb_test::TestRequest::default().to_http_request();
	let response = make_page(lang).respond_to(&req);
	assert_eq!(
		response
			.headers()
			.get(actix_web::http::header::CONTENT_TYPE)
			.unwrap(),
		"text/html; charset=utf-8"
	);
}

#[test]
fn response_body_contains_lang_en() {
	let lang: Lang = "en-US".parse().unwrap();
	assert!(body_string(make_page(lang)).contains(r#"<html lang="en-US">"#));
}

#[test]
fn response_body_contains_lang_nl() {
	let lang: Lang = "nl-NL".parse().unwrap();
	assert!(body_string(make_page(lang)).contains(r#"<html lang="nl-NL">"#));
}

#[test]
fn response_body_contains_translated_content() {
	let lang: Lang = "en-US".parse().unwrap();
	assert!(body_string(make_page(lang)).contains("Hello"));
}

#[test]
fn response_body_translated_nl() {
	let lang: Lang = "nl-NL".parse().unwrap();
	assert!(body_string(make_page(lang)).contains("Hallo"));
}

// --- FromRequest (Lang extractor) ---

async fn lang_route(lang: Lang) -> Page {
	page! { <p>{i18n!("greeting")}</p> }
}

#[actix_web::test]
async fn from_request_extracts_lang_en() {
	let app = actweb_test::init_service(
		App::new().route("/{lang}", actix_web::web::get().to(lang_route)),
	)
	.await;
	let req = actweb_test::TestRequest::get().uri("/en-US").to_request();
	let response = actweb_test::call_service(&app, req).await;
	assert_eq!(response.status(), StatusCode::OK);
	let body = actweb_test::read_body(response).await;
	let html = String::from_utf8(body.to_vec()).unwrap();
	assert!(html.contains(r#"<html lang="en-US">"#));
	assert!(html.contains("Hello"));
}

#[actix_web::test]
async fn from_request_extracts_lang_nl() {
	let app = actweb_test::init_service(
		App::new().route("/{lang}", actix_web::web::get().to(lang_route)),
	)
	.await;
	let req = actweb_test::TestRequest::get().uri("/nl-NL").to_request();
	let body = actweb_test::read_body(actweb_test::call_service(&app, req).await).await;
	let html = String::from_utf8(body.to_vec()).unwrap();
	assert!(html.contains(r#"<html lang="nl-NL">"#));
	assert!(html.contains("Hallo"));
}

#[actix_web::test]
async fn from_request_rejects_invalid_lang() {
	let app = actweb_test::init_service(
		App::new().route("/{lang}", actix_web::web::get().to(lang_route)),
	)
	.await;
	let req = actweb_test::TestRequest::get()
		.uri("/not-a-valid-locale!!!")
		.to_request();
	let response = actweb_test::call_service(&app, req).await;
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
