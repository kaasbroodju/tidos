use tidos::view;

fn toggle_attribute_implicit_variable_name() {
	assert_eq!(
		&view!{
			<input type="radio" name="day" value="monday" :checked="not a bool" />
		},
		r#"<input type="radio" name="day" value="monday" checked />"#
	)
}

fn main() {}