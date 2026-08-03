use tidos::internals::sanitize;
use tidos::{view, Page};

fn render(f: impl FnOnce(&mut Page)) -> String {
	let mut p = Page::new();
	f(&mut p);
	p.into_html()
}

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
		render(|page| {
			view! { <p>{user_input}</p> }
		}),
		"<p>&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;</p>"
	);
}

#[test]
fn ampersand_in_expression_is_escaped() {
	let text = "Tom & Jerry";
	assert_eq!(
		render(|page| {
			view! { <span>{text}</span> }
		}),
		"<span>Tom &amp; Jerry</span>"
	);
}

#[test]
fn quotes_in_expression_are_escaped() {
	let attr = r#"say "hi""#;
	assert_eq!(
		render(|page| {
			view! { <p>{attr}</p> }
		}),
		"<p>say &quot;hi&quot;</p>"
	);
}

// --- @html{} bypasses sanitization ---

#[test]
fn raw_html_is_not_sanitized() {
	assert_eq!(
		render(|page| {
			view! { @html{"<b>bold</b>"} }
		}),
		"<b>bold</b>"
	);
}

#[test]
fn raw_html_with_variable() {
	let snippet = String::from("<em>italic</em>");
	assert_eq!(
		render(|page| {
			view! { @html{snippet} }
		}),
		"<em>italic</em>"
	);
}

#[test]
fn raw_html_empty_string() {
	assert_eq!(
		render(|page| {
			view! { @html{""} }
		}),
		""
	);
}

#[test]
fn raw_html_alongside_sanitized_expression() {
	let user = "<evil>";
	assert_eq!(
		render(|page| {
			view! { <p>{user}@html{"<b>safe</b>"}</p> }
		}),
		"<p>&lt;evil&gt;<b>safe</b></p>"
	);
}

// --- static literal sanitization (compile-time, `TextContent::sanitize_literal`) ---
//
// Everything above uses a dynamic `{expr}`, which goes through the runtime
// `sanitize()` above. A bare `{"literal text"}` is escaped at macro-expansion
// time instead, by a separate (textually identical) implementation in
// `tidos_macro::code_generation::text_content::TextContent::sanitize_literal`.
// These exercise that code path specifically, one special char at a time.

#[test]
fn static_literal_ampersand_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"fish & chips"}</span> }
		}),
		"<span>fish &amp; chips</span>"
	);
}

#[test]
fn static_literal_less_than_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"1 < 2"}</span> }
		}),
		"<span>1 &lt; 2</span>"
	);
}

#[test]
fn static_literal_greater_than_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"2 > 1"}</span> }
		}),
		"<span>2 &gt; 1</span>"
	);
}

#[test]
fn static_literal_escape_sequences_are_cooked() {
	// Regression test: `sanitize_literal`/`literal_string_value` used to read
	// the literal's raw token source text and merely strip the surrounding
	// quotes, leaving Rust's own escape sequences un-cooked (e.g. `\n` would
	// render as the two characters `\` and `n` instead of an actual
	// newline). Fixed by parsing as `syn::LitStr` and using `.value()`.
	assert_eq!(
		render(|page| {
			view! { <span>{"a\tb\nc"}</span> }
		}),
		"<span>a\tb\nc</span>"
	);
}

#[test]
fn raw_html_escape_sequences_are_cooked() {
	assert_eq!(
		render(|page| {
			view! { @html{"a\tb\nc"} }
		}),
		"a\tb\nc"
	);
}

#[test]
fn static_literal_double_quote_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"say \"hello\""}</span> }
		}),
		"<span>say &quot;hello&quot;</span>"
	);
}

#[test]
fn static_literal_single_quote_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"it's"}</span> }
		}),
		"<span>it&#x27;s</span>"
	);
}

#[test]
fn static_literal_multiple_special_chars_are_escaped() {
	assert_eq!(
		render(|page| {
			view! { <span>{"<b>it's & \"fun\"</b>"}</span> }
		}),
		"<span>&lt;b&gt;it&#x27;s &amp; &quot;fun&quot;&lt;/b&gt;</span>"
	);
}

#[test]
fn static_format_string_literal_part_is_escaped() {
	let name = "Bob";
	assert_eq!(
		render(|page| {
			view! { <span>{"fish & chips, {}", name}</span> }
		}),
		"<span>fish &amp; chips, Bob</span>"
	);
}

// --- static attribute literal sanitization (compile-time, `sanitize_lit_str`) ---
//
// `class="wrapper"`-style plain string attribute values are escaped by yet
// another copy of the same logic, `TextContent::sanitize_lit_str`. Every
// existing attribute test happens to use a value with no special characters,
// so this loop had never actually run.

#[test]
fn static_attribute_literal_ampersand_is_escaped() {
	assert_eq!(
		render(|page| {
			view! { <div title="Tom & Jerry"></div> }
		}),
		r#"<div title="Tom &amp; Jerry" ></div>"#
	);
}

#[test]
fn static_attribute_literal_multiple_special_chars_are_escaped() {
	assert_eq!(
		render(|page| {
			view! { <div title="it's <fun> & \"games\""></div> }
		}),
		r#"<div title="it&#x27;s &lt;fun&gt; &amp; &quot;games&quot;" ></div>"#
	);
}
