use tidos::view;

fn attribute_value_without_delimiters() {
	view! {
		<p class=my-class>{"Hello world"}</p>
	};
}

fn main() {}
