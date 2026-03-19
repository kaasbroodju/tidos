use tidos::internals::sanitize;
use tidos_macro::view;

// --- sanitize() unit tests ---

#[test]
fn no_special_chars_returns_input_unchanged() {
	assert_eq!(sanitize("hello world"), "hello world");
}

#[test]
fn escapes_ampersand() {
	assert_eq!(sanitize("fish & chips"), "fish &amp; chips");
}

#[test]
fn escapes_less_than() {
	assert_eq!(sanitize("1 < 2"), "1 &lt; 2");
}

#[test]
fn escapes_greater_than() {
	assert_eq!(sanitize("2 > 1"), "2 &gt; 1");
}

#[test]
fn escapes_double_quote() {
	assert_eq!(sanitize(r#"say "hello""#), "say &quot;hello&quot;");
}

#[test]
fn escapes_single_quote() {
	assert_eq!(sanitize("it's"), "it&#x27;s");
}

#[test]
fn escapes_multiple_special_chars() {
	assert_eq!(
		sanitize(r#"<b class="x">it's & "fun"</b>"#),
		"&lt;b class=&quot;x&quot;&gt;it&#x27;s &amp; &quot;fun&quot;&lt;/b&gt;"
	);
}

#[test]
fn empty_string_returns_empty() {
	assert_eq!(sanitize(""), "");
}

// --- view! expression sanitization ---

#[test]
fn expressions_are_sanitized_in_view() {
	let user_input = "<script>alert('xss')</script>";
	assert_eq!(
		&view! { <p>{user_input}</p> },
		"<p>&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;</p>"
	);
}

#[test]
fn ampersand_in_expression_is_escaped() {
	let text = "Tom & Jerry";
	assert_eq!(&view! { <span>{text}</span> }, "<span>Tom &amp; Jerry</span>");
}

#[test]
fn quotes_in_expression_are_escaped() {
	let attr = r#"say "hi""#;
	assert_eq!(&view! { <p>{attr}</p> }, "<p>say &quot;hi&quot;</p>");
}

// --- @html{} bypasses sanitization ---

#[test]
fn raw_html_is_not_sanitized() {
	assert_eq!(
		&view! { @html{"<b>bold</b>"} },
		"<b>bold</b>"
	);
}

#[test]
fn raw_html_with_variable() {
	let snippet = "<em>italic</em>";
	assert_eq!(
		&view! { @html{snippet} },
		"<em>italic</em>"
	);
}

#[test]
fn raw_html_empty_string() {
	assert_eq!(&view! { @html{""} }, "");
}

#[test]
fn raw_html_alongside_sanitized_expression() {
	let user = "<evil>";
	assert_eq!(
		&view! { <p>{user}@html{"<b>safe</b>"}</p> },
		"<p>&lt;evil&gt;<b>safe</b></p>"
	);
}
