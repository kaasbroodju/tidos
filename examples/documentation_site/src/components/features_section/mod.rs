use tidos::{scoped_css, view, Component, Page};

use crate::components::feature_card::FeatureCard;

struct Feature {
    title: &'static str,
    body: &'static str,
    icon: &'static str,
}

const FEATURES: &[Feature] = &[
    Feature {
        title: "Type Safe",
        icon: "shield",
        body: "Rust's type system flows all the way through your components. Mistakes are caught at compile-time, not in production.",
    },
    Feature {
        title: "Macro Powered",
        icon: "wand",
        body: "Components written directly in Rust with intuitive macros. Pattern matching, loops and conditionals right in your UI.",
    },
    Feature {
        title: "Framework Agnostic",
        icon: "plug",
        body: "Drop Tidos into Rocket, Axum, Actix, or anything else. It plays nicely with the frameworks you already love.",
    },
    Feature {
        title: "Fast SSR",
        icon: "speed",
        body: "Tidos attempts to do the most work during compile-time, so that your favourite http framework only needs to serve a single string.",
    },
];

pub struct FeaturesSection;

impl Component for FeaturesSection {
    fn to_render(&self, page: &mut Page) {
        view! {
            <section class={scoped_css!("./features_section.css")}>
                <h2 class="section-title">
                    {"Built for the "}
                    <em>{"deep end"}</em>
                </h2>
                <div class="grid">
                    {#for f in FEATURES}
                        <FeatureCard
                            title={f.title.to_string()}
                            body={f.body.to_string()}
                            icon={f.icon.to_string()}
                        />
                    {/for}
                </div>
            </section>
        }
    }
}
