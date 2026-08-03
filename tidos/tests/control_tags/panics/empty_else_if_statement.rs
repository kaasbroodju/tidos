use tidos::view;

fn empty_else_if_statement() {
	let n = 1;
	view! {
		{#if n == 0}
			<p>{"Zero"}</p>
		{:else if}
			<p>{"Not zero"}</p>
		{/if}
	};
}

fn main() {}
