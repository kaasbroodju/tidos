use tidos::Component;
use tidos::{Page, page};
use crate::components::hello_world::HelloWorld;

#[get("/")]
pub fn index() -> Page {
    let mut page = page!(
        <main>
            <h1>{"Angular + Tidos"}</h1>
            <p>{"An Angular component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Angular + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
