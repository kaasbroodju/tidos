use tidos::view;

fn at_class_neither_literal_nor_brace() {
	let some_ident = "./scoped_css/test.css";
	view! {
		<div @class=some_ident></div>
	};
}

fn main() {}
