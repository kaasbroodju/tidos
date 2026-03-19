use tidos::view;

fn missing_end_for_loop() {
	let items = vec!["a", "b"];
	view! {
		{#for item in items}
			<p>{item}</p>
	};
}

fn main() {}
