use tidos::view;

fn text_literal_missing_comma() {
	view! {
		<p>{"Hello" world}</p>
	};
}

fn main() {}
