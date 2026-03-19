use tidos::view;

fn misspelled_control_tag() {
	let items = vec!["a", "b"];
	view! {
		{#fore item in items}
			<p>{item}</p>
		{/for}
	};
}

fn main() {}
