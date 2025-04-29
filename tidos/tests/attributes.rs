use tidos_macro::view;

#[test]
fn a_simple_toggle_attribute() {

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" checked />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked={true} />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked={false} />
		},
		r#"<input type="radio" name="day" value="monday" />"#
	);
}

// This test is for performance.
#[test]
fn a_simple_toggle_attribute_reordered() {
	assert_eq!(
		&view!{
			<input :checked={true} type="radio" name="day" value="monday" />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);
}

#[test]
fn toggle_attribute_implicit_variable_name() {

	let checked = true;
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	);

	let checked = false;
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked />
		},
		r#"<input type="radio" name="day" value="monday" />"#
	);
}

#[test]
fn missing_variable_defined_toggle() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/attributes/panics/missing_variable_defined_toggle.rs");
}

#[test]
fn data_attribute() {
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" data-octo={ String::from("css-420")} />
		},
		r#"<input type="radio" name="day" value="monday" data-octo="css-420" />"#
	);
}