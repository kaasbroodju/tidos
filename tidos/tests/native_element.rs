#![cfg(not(feature = "i18n"))]

use tidos::{native_element, Component, Page};

// --- structs used across multiple tests ---

#[native_element]
pub struct GreetUser {
	pub name: String,
	pub is_shiny: bool,
}

#[native_element]
pub struct ClickCounter {
	pub count: String,
}

#[native_element]
pub struct MyToggleSwitch;

// --- tag name derivation ---

#[test]
fn pascal_case_struct_name_becomes_kebab_case_tag() {
	let component = GreetUser {
		name: String::from("Alice"),
		is_shiny: false,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert!(html.starts_with("<greet-user"));
}

#[test]
fn multi_word_pascal_case_becomes_kebab_tag() {
	let component = MyToggleSwitch;
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert!(html.starts_with("<my-toggle-switch"));
}

// --- attribute rendering ---

#[test]
fn string_field_renders_as_attribute() {
	let component = GreetUser {
		name: String::from("Alice"),
		is_shiny: false,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert_eq!(html, r#"<greet-user name="Alice" ></greet-user>"#);
}

#[test]
fn bool_true_renders_toggle_attribute() {
	let component = GreetUser {
		name: String::from("Alice"),
		is_shiny: true,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert_eq!(html, r#"<greet-user name="Alice" is-shiny ></greet-user>"#);
}

#[test]
fn bool_false_omits_toggle_attribute() {
	let component = GreetUser {
		name: String::from("Alice"),
		is_shiny: false,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert_eq!(html, r#"<greet-user name="Alice" ></greet-user>"#);
	assert!(!html.contains("is-shiny"));
}

#[test]
fn snake_case_field_name_becomes_kebab_case_attribute() {
	let component = GreetUser {
		name: String::from("Bob"),
		is_shiny: true,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	// is_shiny → is-shiny
	assert!(html.contains("is-shiny"));
	assert!(!html.contains("is_shiny"));
}

#[test]
fn unit_struct_renders_empty_element() {
	let component = MyToggleSwitch;
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert_eq!(html, "<my-toggle-switch></my-toggle-switch>");
}

// --- head injection ---

#[test]
fn injects_script_module_into_head() {
	let component = GreetUser {
		name: String::from("Alice"),
		is_shiny: false,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	assert!(page.head.contains(r#"src="/dist/GreetUser.js""#));
	assert!(page.head.contains(r#"type="module""#));
}

#[test]
fn script_src_uses_original_pascal_case_struct_name() {
	let component = ClickCounter {
		count: String::from("0"),
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	// filename keeps PascalCase: ClickCounter.js, not counter.js
	assert!(page.head.contains("/dist/ClickCounter.js"));
}

#[test]
fn script_is_deduplicated_when_rendered_multiple_times() {
	let mut page = Page::new();
	for i in 0..3 {
		let component = ClickCounter {
			count: i.to_string(),
		};
		component.to_render(&mut page);
	}
	assert_eq!(page.head.matches("ClickCounter.js").count(), 1);
}

// --- compile-time errors ---

#[test]
fn single_word_struct_is_rejected_at_compile_time() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/native_element/panics/single_word_struct_has_no_hyphen.rs");
}

#[test]
fn attribute_value_is_sanitized() {
	let component = GreetUser {
		name: String::from("<script>"),
		is_shiny: false,
	};
	let mut page = Page::new();
	component.to_render(&mut page);
	let html = page.into_html();
	assert!(!html.contains("<script>"));
	assert!(html.contains("&lt;script&gt;"));
}
