use tidos::view;

fn for_loop_missing_binding() {
	let items = vec!["a", "b"];
	view! {
		{#for in items}
			<p>{item}</p>
		{/for}
	};
}

fn main() {}
