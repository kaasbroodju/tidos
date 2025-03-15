use tidos::view;

fn missing_closing_if_statement() {
	view! {
		{#if true}
			<p>Hello world</p>
	}
}

fn main() {}