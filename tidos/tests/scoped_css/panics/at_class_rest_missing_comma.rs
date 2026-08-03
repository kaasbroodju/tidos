use tidos::view;

fn at_class_rest_missing_comma() {
	let foo = "bar";
	view! {
		<div @class={"./scoped_css/test.css", "extra" foo}></div>
	};
}

fn main() {}
