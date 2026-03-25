use tidos::{view, Component, Page};

struct Card {
	pub header: String,
	pub body: String,
}

impl Component for Card {
	fn to_render(&self, _page: &mut Page) -> String {
		view! {
			<div>
				<header>@html{&self.header}</header>
				<main>@html{&self.body}</main>
			</div>
		}
	}
}

#[cfg(not(feature = "i18n"))]
#[test]
fn single_slot() {
	let mut page_output = Page::new();
	let page = &mut page_output;

	let result = view! {
		<Card>
			{#slot:header}<h1>Title</h1>{/slot}
			{#slot:body}<p>Content</p>{/slot}
		</Card>
	};

	assert_eq!(
		result,
		"<div><header><h1>Title</h1></header><main><p>Content</p></main></div>"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_with_expression() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let title = "Hello World";

	let result = view! {
		<Card>
			{#slot:header}<h1>{title}</h1>{/slot}
			{#slot:body}<p>{"Some content"}</p>{/slot}
		</Card>
	};

	assert_eq!(
		result,
		"<div><header><h1>Hello World</h1></header><main><p>Some content</p></main></div>"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_with_empty_content() {
	let mut page_output = Page::new();
	let page = &mut page_output;

	let result = view! {
		<Card>
			{#slot:header}{/slot}
			{#slot:body}{/slot}
		</Card>
	};

	assert_eq!(result, "<div><header></header><main></main></div>");
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_alongside_prop() {
	struct Banner {
		pub title: &'static str,
		pub content: String,
	}

	impl Component for Banner {
		fn to_render(&self, _page: &mut Page) -> String {
			view! {
				<section>
					<h1>{self.title}</h1>
					<div>@html{&self.content}</div>
				</section>
			}
		}
	}

	let mut page_output = Page::new();
	let page = &mut page_output;

	let result = view! {
		<Banner title="My Banner">
			{#slot:content}<p>Slot content</p>{/slot}
		</Banner>
	};

	assert_eq!(
		result,
		"<section><h1>My Banner</h1><div><p>Slot content</p></div></section>"
	);
}
