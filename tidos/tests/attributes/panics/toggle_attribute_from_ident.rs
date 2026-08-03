use tidos::view;

fn toggle_attribute_from_ident() {
	let some_var = true;
	view! {
		<input :checked=some_var />
	};
}

fn main() {}
