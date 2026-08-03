use tidos::view;

fn slot_missing_identifier() {
	view! {
		{#slot}
			<p>{"Hello world"}</p>
		{/slot}
	};
}

fn main() {}
