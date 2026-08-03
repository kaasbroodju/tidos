use tidos::{view, Component, Page};

struct Widget;

impl Component for Widget {
	fn to_render(&self, _page: &mut Page) {}
}

fn at_class_on_component() {
	view! {
		<Widget @class="./scoped_css/test.css" />
	};
}

fn main() {}
