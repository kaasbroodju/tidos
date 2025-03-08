# Tidos
[![Download](https://img.shields.io/crates/v/tidos)](https://crates.io/crates/tidos/)
[![API Docs](https://img.shields.io/badge/documentation-tidos-blue)](https://docs.rs/tidos/latest/tidos/)

Tidos is a high-performance Rust-based component framework that seamlessly integrates with any web framework, enabling developers to build dynamic web applications with ease. With Tidos’ powerful macros, you can intuitively create components directly within your Rust code. It even allows you to leverage Rust's pattern matching, loops, and conditionals inside your components—making your UI logic more expressive and maintainable.

```rust
use tidos_macro::view;
let names = vec!["Bob", "Alice"];

view! {
   {#for name in names}
   	<p>{format!("Hello {}!", name)}</p>
   {/for}
}
```

## Examples
### A simple example
```rust
use tidos::{Component, Page};

pub struct Greet {
    pub name: String,
}

impl Component for Greet {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <h1>Hello {&self.name}</h1>
        }
    }
}


// Example route from Rocket, but you can use any framework you want.
#[get("/")]
pub fn index() -> Page {
	page! {
        <main>
            <Greet name={ String::from("kaasbroodju") } />
        </main>
    }
}
```
### More examples
For more examples visit [our documentation](https://docs.rs/tidos/latest/tidos/).

## Getting help
If you're stuck or need help, reach out to the community via [our Github discussions](https://github.com/kaasbroodju/tidos/discussions).

## Contributing
Contributions are absolutely, positively welcomed and encouraged! If you're
interested in contributing code or documentation, please first read the [code of conduct].

Additionally, you could:
1. Submit a feature request or bug report as an [issue].
2. Ask for improved documentation as an [issue].
3. Answers questions in [GitHub discussions questions].
4. Share a project in [GitHub discussions show & tell].

[issue]: https://github.com/kaasbroodju/tidos/issues
[code of conduct]: CODE_OF_CONDUCT.md
[GitHub discussions questions]: https://github.com/kaasbroodju/tidos/discussions/categories/q-a
[GitHub discussions show & tell]: https://github.com/kaasbroodju/tidos/discussions/categories/show-and-tell