use tidos::{view, Page};

fn toggle_attribute_implicit_variable_name() {
	let mut page = Page::new();
	let page = &mut page;
	view! {
		<input type="radio" name="day" value="monday" :checked />
	}
}

fn main() {}
