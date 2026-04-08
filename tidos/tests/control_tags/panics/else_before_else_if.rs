use tidos::view;

fn else_before_else_if() {
	let n = 1;
	view! {
		{#if n == 0}
			<p>{"Zero"}</p>
		{:else}
			<p>{"Not zero"}</p>
		{:else if n == 1}
			<p>{"One"}</p>
		{/if}
	};
}

fn main() {}
