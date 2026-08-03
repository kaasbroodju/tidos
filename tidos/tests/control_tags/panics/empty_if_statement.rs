use tidos::view;

fn empty_if_statement() {
	view! {
		{#if}
			<p>{"Hello world"}</p>
		{/if}
	};
}

fn main() {}
