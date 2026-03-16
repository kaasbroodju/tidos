use tidos::view;

fn nested_wrong_closing_tag() {
	view! {
		<div>
			<p>
		</div>
	}
}

fn main() {}
