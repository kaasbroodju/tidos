use tidos::view;

fn at_class_wrong_attribute() {
	view! {
		<div @id="./scoped_css/test.css"></div>
	};
}

fn main() {}
