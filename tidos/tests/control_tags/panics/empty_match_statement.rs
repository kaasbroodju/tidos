use tidos::view;

fn empty_match_statement() {
	view! {
		{#match}
			{:case _}
				<p>{"Hello world"}</p>
		{/match}
	};
}

fn main() {}
