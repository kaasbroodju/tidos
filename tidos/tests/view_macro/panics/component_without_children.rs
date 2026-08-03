use tidos::{view, Component, Page};

struct Widget;

impl Component for Widget {
	fn to_render(&self, _page: &mut Page) {}
}

fn component_without_children() {
	view! {
		<Widget></Widget>
	};
}

fn main() {}
