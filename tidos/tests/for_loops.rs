use tidos_macro::view;

#[test]
fn a_simple_for_loop() {
	let names = vec!["Bob", "Alice"];

	assert_eq!(
		&view! {
			{#for name in names}
				<p>{name}</p>
			{/for}
		},
		"<p>Bob</p><p>Alice</p>"
	);
}

#[test]
fn an_empty_for_loop() {
	let names: Vec<&str> = vec![];

	assert_eq!(
		&view! {
			<main>
				{#for name in names}
					<p>{name}</p>
				{/for}
			</main>
		},
		"<main></main>"
	);
}

#[test]
fn a_complex_for_loop() {
	let name = String::from("kaasbroodju");

	assert_eq!(
		&view! {
			<main>
				{#for (i, c) in name.chars().enumerate()}
					<span>{format!("{}. {}", i, c)}</span>
				{/for}
			</main>
		},
		"<main><span>0. k</span><span>1. a</span><span>2. a</span><span>3. s</span><span>4. b</span><span>5. r</span><span>6. o</span><span>7. o</span><span>8. d</span><span>9. j</span><span>10. u</span></main>"
	);
}