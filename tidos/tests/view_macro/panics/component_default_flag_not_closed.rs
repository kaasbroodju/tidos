use tidos::{view, Component, Page};

#[derive(Default)]
struct Widget;

impl Component for Widget {
	fn to_render(&self, _page: &mut Page) {}
}

fn component_default_flag_not_closed() {
	view! {
		<Widget .. garbage>
	};
}

fn main() {}
