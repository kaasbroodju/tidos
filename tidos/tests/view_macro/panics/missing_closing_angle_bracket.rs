use tidos::view;

fn missing_closing_angle_bracket() {
	view! {
		<p class="greeting"
	};
}

fn main() {}
