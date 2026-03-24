use tidos::Component;
use tidos::{Page, page};
use crate::components::hello_world::HelloWorld;

#[get("/")]
pub fn index() -> Page {
    let mut page = page!(
        <main>
            <h1>{"Vue + Tidos"}</h1>
            <p>{"A Vue component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Vue + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
