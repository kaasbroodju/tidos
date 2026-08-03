use tidos::view;

fn at_class_missing_comma_after_path() {
	view! {
		<div @class={"./scoped_css/test.css" "extra"}></div>
	};
}

fn main() {}
