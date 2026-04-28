use tidos::{view, Page};

fn render(f: impl FnOnce(&mut Page)) -> String {
	let mut p = Page::new();
	f(&mut p);
	p.into_html()
}

#[test]
fn empty_body() {
	assert_eq!(
		render(|page| {
			view! {}
		}),
		""
	);
}

#[test]
fn raw_html() {
	assert_eq!(
		render(|page| {
			view! {
				@html{"<p>hello world</p>"}
			}
		}),
		r#"<p>hello world</p>"#
	);
}

#[test]
fn custom_element() {
	assert_eq!(
		render(|page| {
			view! {
				<custom-element></custom-element>
			}
		}),
		r#"<custom-element></custom-element>"#
	);

	assert_eq!(
		render(|page| {
			view! {
				<custom-element-electric-bogaloo></custom-element-electric-bogaloo>
			}
		}),
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
fn wrong_closing_tag() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/wrong_closing_tag.rs");
}

#[test]
fn nested_wrong_closing_tag() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/nested_wrong_closing_tag.rs");
}

#[test]
fn missing_closing_angle_bracket() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/missing_closing_angle_bracket.rs");
}

#[test]
fn tag_name_ends_with_hyphen() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/tag_name_ends_with_hyphen.rs");
}

#[test]
fn empty_closing_tag() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/view_macro/panics/empty_closing_tag.rs");
}

#[test]
fn text_literal() {
	assert_eq!(
		render(|page| {
			view! { <p>{"Hello world"}</p> }
		}),
		"<p>Hello world</p>"
	);
}

#[test]
fn text_literal_with_sanitation() {
	assert_eq!(
		render(|page| {
			view! { <p>{"Hello>world"}</p> }
		}),
		"<p>Hello&gt;world</p>"
	);
}

#[test]
fn text_literal_with_params() {
	let a = "world";
	let b = "mars";
	assert_eq!(
		render(|page| {
			view! { <p>{"Hello {} and {}", a, b}</p> }
		}),
		"<p>Hello world and mars</p>"
	);
}

#[test]
fn text_expression() {
	let greeting = String::from("hello");
	assert_eq!(
		render(|page| {
			view! { <p>{greeting.to_uppercase()}</p> }
		}),
		"<p>HELLO</p>"
	);
}

#[test]
fn raw_html_literal() {
	assert_eq!(
		render(|page| {
			view! { @html{"<b>bold</b>"} }
		}),
		"<b>bold</b>"
	);
}

#[test]
fn raw_html_literal_with_params() {
	let a = "bold";
	let b = "italic";
	assert_eq!(
		render(|page| {
			view! { @html{"<b>{}</b><i>{}</i>", a, b} }
		}),
		"<b>bold</b><i>italic</i>"
	);
}

#[test]
fn raw_html_expression() {
	let html = String::from("<em>emphasized</em>");
	assert_eq!(
		render(|page| {
			view! { @html{html} }
		}),
		"<em>emphasized</em>"
	);
}
