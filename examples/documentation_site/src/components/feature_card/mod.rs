use tidos::{scoped_css, view, Component, Page};

pub struct FeatureCard {
    pub title: String,
    pub body: String,
    pub icon: String,
}

fn icon_svg(name: &str) -> String {
    let stroke = r##"stroke="#4fd1c5" stroke-width="2" fill="none" stroke-linecap="round" stroke-linejoin="round""##;
    match name {
        "shield" => format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" height="28px" viewBox="0 -960 960 960" width="28px" fill="#4fd1c5"><path d="m438-338 226-226-57-57-169 169-84-84-57 57 141 141Zm42 258q-139-35-229.5-159.5T160-516v-244l320-120 320 120v244q0 152-90.5 276.5T480-80Z"/></svg>"##
        ),
        "wand" => format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" height="28px" viewBox="0 0 24 24" width="28px" fill="#4fd1c5"><path d="M0 0h24v24H0z" fill="none"/><path d="M7.5 5.6L10 7 8.6 4.5 10 2 7.5 3.4 5 2l1.4 2.5L5 7zm12 9.8L17 14l1.4 2.5L17 19l2.5-1.4L22 19l-1.4-2.5L22 14zM22 2l-2.5 1.4L17 2l1.4 2.5L17 7l2.5-1.4L22 7l-1.4-2.5zm-7.63 5.29c-.39-.39-1.02-.39-1.41 0L1.29 18.96c-.39.39-.39 1.02 0 1.41l2.34 2.34c.39.39 1.02.39 1.41 0L16.7 11.05c.39-.39.39-1.02 0-1.41l-2.33-2.35zm-1.03 5.49l-2.12-2.12 2.44-2.44 2.12 2.12-2.44 2.44z"/></svg>"##
        ),
        "plug" => format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" height="28px" viewBox="0 -960 960 960" width="28px" fill="#4fd1c5"><path d="M240-280 40-480l200-200 56 56-143 144 143 144-56 56Zm178 132-76-24 200-640 76 24-200 640Zm302-132-56-56 143-144-143-144 56-56 200 200-200 200Z"/></svg>"##
        ),
        "bolt" => format!(
            r#"<svg viewBox="0 0 32 32" width="28" height="28"><path d="M 18 4 L 8 18 L 14 18 L 12 28 L 24 12 L 18 12 Z" {stroke}/></svg>"#
        ),
        "speed" => format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" height="28" width="28" viewBox="0 -960 960 960"><path xmlns="http://www.w3.org/2000/svg" d="M418-340q25 25 63 23.5t55-27.5l224-336-336 224q-26 18-28.5 54.5T418-340ZM204-160q-22 0-40.5-9.5T134-198q-26-47-40-97.5T80-400q0-83 31.5-156T197-683q54-54 127-85.5T480-800q82 0 154 31t126 84.5q54 53.5 86 125T879-406q1 55-12.5 107.5T825-198q-11 19-29.5 28.5T755-160H204Z" fill="#4fd1c5"/></svg>"##
        ),
        _ => String::new(),
    }
}

impl Component for FeatureCard {
    fn to_render(&self, page: &mut Page) {
        let icon = icon_svg(&self.icon);
        view! {
            <div class={scoped_css!("./feature_card.css")}>
                <div class="icon-header">
                    <div class="icon-wrap">
                        @html{icon}
                    </div>
                    <h3>{&self.title}</h3>
                </div>

                <p>{&self.body}</p>
            </div>
        }
    }
}
