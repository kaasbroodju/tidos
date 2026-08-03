use tidos::{scoped_css, view, Page};

#[cfg(not(feature = "i18n"))]
#[test]
fn returns_scoped_class_name_and_injects_style() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let class = scoped_css!("./scoped_css/test.css");
	assert!(class.starts_with("tidos-"));
	assert!(page.head.contains(&format!("<style>.{class}")));
	assert!(page.head.contains("color: red"));
}

#[cfg(not(feature = "i18n"))]
#[test]
fn deduplicates_in_for_loop() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	for _ in 0..5 {
		scoped_css!("./scoped_css/test.css");
	}
	assert_eq!(page.head.matches("<style>").count(), 1);
}

#[test]
fn file_not_found() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/file_not_found.rs");
}

#[cfg(not(feature = "i18n"))]
#[test]
fn at_class_attribute_bare_literal() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	view! {
		<div @class="./scoped_css/test.css"></div>
	}
	let html = page.template.clone();
	assert!(html.contains(r#"<div class="tidos-"#));
	assert!(page.head.contains("color: red"));
}

#[cfg(not(feature = "i18n"))]
#[test]
fn at_class_attribute_braced_literal() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	view! {
		<div @class={"./scoped_css/test.css"}></div>
	}
	let html = page.template.clone();
	assert!(html.contains(r#"<div class="tidos-"#));
	assert!(page.head.contains("color: red"));
}

#[cfg(not(feature = "i18n"))]
#[test]
fn at_class_attribute_with_literal_rest() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	view! {
		<div @class={"./scoped_css/test.css", "extra"}></div>
	}
	let html = page.template.clone();
	let class_start = html.find("class=\"").unwrap() + "class=\"".len();
	let class_end = html[class_start..].find('"').unwrap() + class_start;
	let class_value = &html[class_start..class_end];

	let mut parts = class_value.split(' ');
	assert!(parts.next().unwrap().starts_with("tidos-"));
	assert_eq!(parts.next(), Some("extra"));
	assert_eq!(parts.next(), None);
	assert!(page.head.contains("color: red"));
}

#[cfg(not(feature = "i18n"))]
#[test]
fn at_class_attribute_with_formatted_rest() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let expr = "extralong";
	view! {
		<div @class={"./scoped_css/test.css", "extra {}", expr}></div>
	}
	let html = page.template.clone();
	let class_start = html.find("class=\"").unwrap() + "class=\"".len();
	let class_end = html[class_start..].find('"').unwrap() + class_start;
	let class_value = &html[class_start..class_end];

	let mut parts = class_value.split(' ');
	assert!(parts.next().unwrap().starts_with("tidos-"));
	assert_eq!(parts.next(), Some("extra"));
	assert_eq!(parts.next(), Some("extralong"));
	assert_eq!(parts.next(), None);
	assert!(page.head.contains("color: red"));
}

#[cfg(not(feature = "i18n"))]
#[test]
fn at_class_attribute_with_expression_rest() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let extra_class = String::from("extra");
	view! {
		<div @class={"./scoped_css/test.css", extra_class}></div>
	}
	let html = page.template.clone();
	let class_start = html.find("class=\"").unwrap() + "class=\"".len();
	let class_end = html[class_start..].find('"').unwrap() + class_start;
	let class_value = &html[class_start..class_end];

	let mut parts = class_value.split(' ');
	assert!(parts.next().unwrap().starts_with("tidos-"));
	assert_eq!(parts.next(), Some("extra"));
	assert_eq!(parts.next(), None);
	assert!(page.head.contains("color: red"));
}

#[test]
fn at_class_invalid_name() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_invalid_name.rs");
}

#[test]
fn at_class_wrong_attribute() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_wrong_attribute.rs");
}

#[test]
fn at_class_missing_equals() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_missing_equals.rs");
}

#[test]
fn at_class_non_literal_braced() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_non_literal_braced.rs");
}

#[test]
fn at_class_missing_comma_after_path() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_missing_comma_after_path.rs");
}

#[test]
fn at_class_neither_literal_nor_brace() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_neither_literal_nor_brace.rs");
}

#[test]
fn at_class_non_string_literal() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_non_string_literal.rs");
}

#[test]
fn at_class_rest_missing_comma() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_rest_missing_comma.rs");
}

#[test]
fn at_class_on_component() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/scoped_css/panics/at_class_on_component.rs");
}
