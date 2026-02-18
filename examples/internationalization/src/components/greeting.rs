use tidos::{view, Component, Page};
use tidos_i18n::i18n;

pub struct Greeting;

impl Component for Greeting {
    fn to_render(&self, _page: &mut Page) -> String {
        view! {
			<section>
				<h1>{i18n!("greeting")}</h1>
				<p>{i18n!("shared-photos", ("userName", "Anne"), ("userGender", "female"), ("photoCount", 3) )}</p>
			</section>
		}
    }
}