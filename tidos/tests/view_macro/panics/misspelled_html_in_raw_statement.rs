use tidos::view;

fn misspelled_html_in_raw_statement() {
	assert_eq!(
		&view!{
			@hdmi{"<p>hello world</p>"}
		},
		r#"<p>hello world</p>"#
	);
}

fn main() {}