use tidos::{view, Component, Page, Slot};

struct Card<'a> {
	pub header: Slot<'a>,
	pub body: Slot<'a>,
}

impl Component for Card<'_> {
	fn to_render(&self, page: &mut Page) {
		view! {
			<div>
				<header>@slot{self.header}</header>
				<main>@slot{self.body}</main>
			</div>
		}
	}
}

#[cfg(not(feature = "i18n"))]
#[test]
fn single_slot() {
	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Card>
			{#slot:header}<h1>{"Title"}</h1>{/slot}
			{#slot:body}<p>{"Content"}</p>{/slot}
		</Card>
	};

	assert_eq!(
		page_output.into_html(),
		"<div><header><h1>Title</h1></header><main><p>Content</p></main></div>"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_with_expression() {
	let mut page_output = Page::new();
	let page = &mut page_output;
	let title = "Hello World";

	view! {
		<Card>
			{#slot:header}<h1>{title}</h1>{/slot}
			{#slot:body}<p>{"Some content"}</p>{/slot}
		</Card>
	};

	assert_eq!(
		page_output.into_html(),
		"<div><header><h1>Hello World</h1></header><main><p>Some content</p></main></div>"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_with_empty_content() {
	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Card>
			{#slot:header}{/slot}
			{#slot:body}{/slot}
		</Card>
	};

	assert_eq!(
		page_output.into_html(),
		"<div><header></header><main></main></div>"
	);
}

#[cfg(not(feature = "i18n"))]
#[test]
fn slot_alongside_prop() {
	struct Banner<'a> {
		pub title: &'static str,
		pub content: Slot<'a>,
	}

	impl Component for Banner<'_> {
		fn to_render(&self, page: &mut Page) {
			view! {
				<section>
					<h1>{self.title}</h1>
					<div>@slot{self.content}</div>
				</section>
			}
		}
	}

	let mut page_output = Page::new();
	let page = &mut page_output;

	view! {
		<Banner title="My Banner">
			{#slot:content}<p>{"Slot content"}</p>{/slot}
		</Banner>
	};

	assert_eq!(
		page_output.into_html(),
		"<section><h1>My Banner</h1><div><p>Slot content</p></div></section>"
	);
}
