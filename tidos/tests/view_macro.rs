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
fn custom_element() {
	assert_eq!(
		&view!{
			<custom-element></custom-element>
		},
		r#"<custom-element></custom-element>"#
	);

	assert_eq!(
		&view!{
			<custom-element-electric-bogaloo></custom-element-electric-bogaloo>
		},
		r#"<custom-element-electric-bogaloo></custom-element-electric-bogaloo>"#
	);
}

#[test]
fn misspelled_html_in_raw_statement() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/misspelled_html_in_raw_statement.rs");
}

#[test]
fn forgot_to_close_html_tag() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/forgot_to_close_html_tag.rs");
}

#[test]
fn punctuation() {
	assert_eq!(
		&view!{<p>Lorem. Ipsum</p>},
		r#"<p>Lorem. Ipsum</p>"#
	);
}