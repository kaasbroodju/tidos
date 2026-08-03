use tidos::view;

fn toggle_attribute_from_reference() {
	let is_checked = true;
	view! {
		<input :checked=&is_checked />
	};
}

fn main() {}
