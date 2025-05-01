use tidos::view;

fn misspelled_html_in_raw_statement() {
	assert_eq!(
		&view!{
			<div>
		},
		r#"macro should panic"#
	);
}

fn main() {}