#![cfg(feature = "i18n")]

use tidos::i18n::{enable_i18n, i18n, Lang};
use tidos::{page, view, Page};

enable_i18n!();

#[cfg(feature = "rocket")]
mod rocket;

#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "actix-web")]
mod actix_web;

#[cfg(feature = "warp")]
mod warp;

// --- helpers ---

fn en() -> Lang {
	Lang("en-US".parse().unwrap())
}

fn nl() -> Lang {
	Lang("nl-NL".parse().unwrap())
}

// --- i18n! key lookup ---

#[test]
fn simple_key_en() {
	let lang = en();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	assert_eq!(i18n!("greeting"), "Hello");
}

#[test]
fn simple_key_nl() {
	let lang = nl();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	assert_eq!(i18n!("greeting"), "Hallo");
}

#[test]
fn key_with_variables() {
	let lang = en();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	let result = i18n!(
		"shared-photos",
		("userName", "Anne"),
		("userGender", "female"),
		("photoCount", 5)
	);
	assert!(result.contains("Anne"));
	assert!(result.contains("5"));
	assert!(result.contains("her stream"));
}

#[test]
fn key_with_variables_singular() {
	let lang = en();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	let result = i18n!(
		"shared-photos",
		("userName", "Bob"),
		("userGender", "male"),
		("photoCount", 1)
	);
	assert!(result.contains("a new photo"));
	assert!(result.contains("his stream"));
}

#[test]
fn key_with_variables_nl() {
	let lang = nl();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	let result = i18n!(
		"shared-photos",
		("userName", "Anna"),
		("userGender", "female"),
		("photoCount", 3)
	);
	assert!(result.contains("Anna"));
	assert!(result.contains("haar stream"));
}

// --- i18n! in view! ---

#[test]
fn i18n_inside_view() {
	let lang = en();
	let mut page_output = Page::new(lang.0);
	let page = &mut page_output;
	view! { <h1>{i18n!("greeting")}</h1> }
	assert_eq!(&page.template, "<h1>Hello</h1>");
}

// --- page! lang attribute ---

#[test]
fn page_lang_is_set_to_en() {
	let lang = en();
	let result = page! { <p>{"Hello"}</p> };
	assert_eq!(result.lang.to_string(), "en-US");
}

#[test]
fn page_lang_is_set_to_nl() {
	let lang = nl();
	let result = page! { <p>{"Hallo"}</p> };
	assert_eq!(result.lang.to_string(), "nl-NL");
}

#[test]
fn page_lang_propagates_to_i18n() {
	let lang = nl();
	let result = page! {
		<p>{i18n!("greeting")}</p>
	};
	assert_eq!(result.template, "<p>Hallo</p>");
}
