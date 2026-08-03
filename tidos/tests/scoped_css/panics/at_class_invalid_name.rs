use tidos::view;

fn at_class_invalid_name() {
	view! {
		<div @="./scoped_css/test.css"></div>
	};
}

fn main() {}
