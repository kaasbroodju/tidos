mod control_tags;

use tidos::{view, Page};

fn render(f: impl FnOnce(&mut Page)) -> String {
	let mut p = Page::new();
	f(&mut p);
	p.into_html()
}

#[test]
fn a_simple_conditional() {
	assert_eq!(
		render(|page| view! {
			{#if true}
				<p>{"Hello world"}</p>
			{/if}
		}),
		"<p>Hello world</p>"
	);

	assert_eq!(
		render(|page| view! {
			{#if false}
				<p>{"Hello world"}</p>
			{/if}
		}),
		""
	);
}

#[test]
fn a_simple_conditional_with_an_else() {
	assert_eq!(
		render(|page| view! {
			{#if true}
				<p>{"Hello world"}</p>
			{:else}
				<p>{"Hello mars"}</p>
			{/if}
		}),
		"<p>Hello world</p>"
	);

	assert_eq!(
		render(|page| view! {
			{#if false}
				<p>{"Hello world"}</p>
			{:else}
				<p>{"Hello mars"}</p>
			{/if}
		}),
		"<p>Hello mars</p>"
	);
}

#[test]
fn a_simple_conditional_with_if_else_chain() {
	fn view_component(n: usize) -> String {
		render(|page| {
			view! {
				{#if n == 0}
					<p>{"0"}</p>
				{:else if n == 1}
					<p>{"1"}</p>
				{/if}
			}
		})
	}

	for i in 0..=1 {
		assert_eq!(view_component(i), format!("<p>{}</p>", i));
	}
	assert_eq!(view_component(2), "");

	fn view_component_multiple_if_else(n: usize) -> String {
		render(|page| {
			view! {
				{#if n == 0}
					<p>{"0"}</p>
				{:else if n == 1}
					<p>{"1"}</p>
				{:else if n == 2}
					<p>{"2"}</p>
				{/if}
			}
		})
	}

	for i in 0..=2 {
		assert_eq!(view_component_multiple_if_else(i), format!("<p>{}</p>", i));
	}
	assert_eq!(view_component_multiple_if_else(3), "");

	fn view_component_with_else(n: usize) -> String {
		render(|page| {
			view! {
				{#if n == 0}
					<p>{"0"}</p>
				{:else if n == 1}
					<p>{"1"}</p>
				{:else}
					<p>{"Hello world"}</p>
				{/if}
			}
		})
	}

	for i in 0..=1 {
		assert_eq!(view_component_with_else(i), format!("<p>{}</p>", i));
	}
	assert_eq!(view_component_with_else(2), "<p>Hello world</p>");

	fn view_component_multiple_if_else_with_else(n: usize) -> String {
		render(|page| {
			view! {
				{#if n == 0}
					<p>{"0"}</p>
				{:else if n == 1}
					<p>{"1"}</p>
				{:else if n == 2}
					<p>{"2"}</p>
				{:else}
					<p>{"Hello world"}</p>
				{/if}
			}
		})
	}

	for i in 0..=2 {
		assert_eq!(
			view_component_multiple_if_else_with_else(i),
			format!("<p>{}</p>", i)
		);
	}
	assert_eq!(
		view_component_multiple_if_else_with_else(3),
		"<p>Hello world</p>"
	);
}

#[test]
fn missing_closing_if_statement() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/control_tags/panics/missing_end_control_tag.rs");
}

#[test]
fn else_before_else_if() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/control_tags/panics/else_before_else_if.rs");
}

#[test]
fn double_else() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/control_tags/panics/double_else.rs");
}
