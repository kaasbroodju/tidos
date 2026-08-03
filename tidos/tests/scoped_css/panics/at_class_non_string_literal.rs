use tidos::view;

fn at_class_non_string_literal() {
	view! {
		<div @class=123></div>
	};
}

fn main() {}
