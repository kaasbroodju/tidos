use tidos_macro::view;

#[allow(unused)]
#[test]
fn a_simple_match_statement() {
	enum Pet {
		Fish,
		Dog,
		Cat,
		Other { name: String }
	}
	use Pet::*;
	let my_pet = Dog;

	assert_eq!(
		&view! {
			{#match my_pet}
				{:case Fish}
					<p>Blub!</p>
				{:case Dog}
					<p>Good boy!</p>
				{:case Cat}
					<p>Give al mortal possessions to cat!</p>
				{:case _}
					<p>Is it a snake or a spider?</p>
			{/match}
		},
		"<p>Good boy!</p>"
	)
}

#[allow(unused)]
#[test]
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
					<p>Blub!</p>
				{:case Dog}
					<p>Good boy!</p>
				{:case Cat}
					<p>Give al mortal possessions to cat!</p>
					<p>Give al mortal possessions to cat!</p>
					<p>Give al mortal possessions to cat!</p>
				{:case Other { name }}
					<p>What is that?</p>
					<p>{format!("It is a {name}!")}</p>
			{/match}
		},
		"<p>What is that?</p><p>It is a spider!</p>"
	)
}

#[test]
fn missing_pattern_in_match_statement() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/control_tags/panics/missing_pattern_in_match_statement.rs");
}