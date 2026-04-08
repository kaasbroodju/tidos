use tidos::view;

fn wrong_closing_tag() {
	view! {
		<div>
			<p>{"Hello world"}</p>
		</p>
	}
}

fn main() {}
