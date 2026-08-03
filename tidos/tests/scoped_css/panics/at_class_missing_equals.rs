use tidos::view;

fn at_class_missing_equals() {
	view! {
		<div @class "./scoped_css/test.css"></div>
	};
}

fn main() {}
