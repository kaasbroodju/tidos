use tidos::view;

fn at_class_non_literal_braced() {
	let some_var = "./scoped_css/test.css";
	view! {
		<div @class={some_var}></div>
	};
}

fn main() {}
