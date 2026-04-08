#![cfg(not(feature = "i18n"))]

use tidos::{head, page, Component, Page};

#[test]
fn renders_basic_html() {
	let result = page! {
		<main><h1>{"Hello"}</h1></main>
	};
	assert_eq!(result.template, "<main><h1>Hello</h1></main>");
}

#[test]
fn empty_page() {
	let result = page! {};
	assert_eq!(result.template, "");
	assert_eq!(result.head, "");
}

#[test]
fn head_is_empty_by_default() {
	let result = page! {
		<p>{"Content"}</p>
	};
	assert_eq!(result.head, "");
}

#[test]
fn renders_expression() {
	let name = "world";
	let result = page! {
		<p>{name}</p>
	};
	assert_eq!(result.template, "<p>world</p>");
}

#[test]
fn renders_component() {
	struct Greeting {
		pub name: String,
	}

	impl Component for Greeting {
		fn to_render(&self, _page: &mut Page) -> String {
			format!("<span>{}</span>", &self.name)
		}
	}

	let result = page! {
		<Greeting name={ String::from("Alice") } />
	};
	assert_eq!(result.template, "<span>Alice</span>");
}

// head! tests

#[test]
fn head_injects_into_page() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	head! { <title>{"My Title"}</title> }
	assert!(page.head.contains("<title>My Title</title>"));
}

#[test]
fn head_deduplicates_same_invocation() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	for _ in 0..3 {
		head! { <title>{"My Title"}</title> }
	}
	assert_eq!(page.head.matches("<title>My Title</title>").count(), 1);
}

#[test]
fn head_multiple_different_invocations_both_inject() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	head! { <link rel="stylesheet" href="/style.css" /> }
	head! { <title>{"My Page"}</title> }
	assert!(page.head.contains("<link"));
	assert!(page.head.contains("<title>My Page</title>"));
}

#[test]
fn head_with_expression() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let title = "Dynamic Title";
	head! { <title>{title}</title> }
	assert!(page.head.contains("<title>Dynamic Title</title>"));
}

#[test]
fn head_injected_via_component_renders_on_page() {
	struct TitleComponent {
		pub title: String,
	}

	impl Component for TitleComponent {
		fn to_render(&self, page: &mut Page) -> String {
			head! { <title>{&self.title}</title> }
			String::new()
		}
	}

	let result = page! {
		<TitleComponent title={ String::from("Hello World") } />
	};
	assert!(result.head.contains("<title>Hello World</title>"));
	assert_eq!(result.template, "");
}
