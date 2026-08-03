use tidos::{view, Component, Page};

#[derive(Default)]
struct Widget;

impl Component for Widget {
	fn to_render(&self, _page: &mut Page) {}
}

fn single_dot_default_flag() {
	view! {
		<Widget . />
	};
}

fn main() {}
