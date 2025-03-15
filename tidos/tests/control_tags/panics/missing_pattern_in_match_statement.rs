use tidos::view;

fn a_complex_match_statement() {
	enum Pet {
		Fish,
		Dog,
		Cat,
		Other { name: String }
	}
	use Pet::*;
	let my_pet = Other { name: String::from("spider") };

	assert_eq!(
		&view! {
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
		},
		"<p>What is that?</p><p>It is a spider!</p>"
	)
}

fn main() {}