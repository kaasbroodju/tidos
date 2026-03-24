use tidos::Component;
use tidos::{Page, page};
use crate::components::hello_world::HelloWorld;

#[get("/")]
pub fn index() -> Page {
    let mut page = page!(
        <main>
            <h1>{"React + Tidos"}</h1>
            <p>{"A React component mounted in a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"React + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
