use tidos::view;

fn double_else() {
	let n = 1;
	view! {
		{#if n == 0}
			<p>{"Zero"}</p>
		{:else}
			<p>{"Not zero"}</p>
		{:else}
			<p>{"Also not zero?"}</p>
		{/if}
	};
}

fn main() {}
