use tidos::{scoped_css, Page};

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
