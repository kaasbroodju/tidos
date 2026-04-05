use tidos::view;

fn toggle_attribute_implicit_variable_name() {
	assert_eq!(
		&view!{
			<img .. />
		},
		r#"<img alt="" />"#
	)
}

fn main() {}