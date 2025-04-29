use tidos_macro::view;

#[test]
fn empty_body() {
	assert_eq!(
		&view!{},
		""
	);
}

#[test]
fn raw_html() {
	assert_eq!(
		&view!{
			@html{"<p>hello world</p>"}
		},
		r#"<p>hello world</p>"#
	);
}

#[test]
fn misspelled_html_in_raw_statement() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/misspelled_html_in_raw_statement.rs");
}

#[test]
fn punctuation() {
	assert_eq!(
		&view!{<p>Lorem. Ipsum</p>},
		r#"<p>Lorem. Ipsum</p>"#
	);
}