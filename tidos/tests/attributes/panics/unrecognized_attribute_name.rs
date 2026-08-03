use tidos::view;

fn unrecognized_attribute_name() {
	view! {
		<div 5></div>
	};
}

fn main() {}
