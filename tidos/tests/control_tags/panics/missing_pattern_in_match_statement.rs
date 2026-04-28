use tidos::{view, Page};

fn a_complex_match_statement() {
	enum Pet {
		Fish,
		Dog,
		Cat,
		Other { name: String }
	}
	use Pet::*;
	let my_pet = Other { name: String::from("spider") };

	let mut page = Page::new();
	let page = &mut page;
	view! {
		{#match my_pet}
			{:case Fish}
				<p>{"Blub!"}</p>
			{:case Dog}
				<p>{"Good boy!"}</p>
			{:case Cat}
				<p>{"Give al mortal possessions to cat!"}</p>
				<p>{"Give al mortal possessions to cat!"}</p>
				<p>{"Give al mortal possessions to cat!"}</p>
		{/match}
	}
}

fn main() {}
