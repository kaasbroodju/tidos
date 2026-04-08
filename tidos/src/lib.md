# Tidos

Tidos is a Rust **server-side rendering (SSR)** component framework.
Write type-safe HTML components directly in Rust using the [`view!`] and
[`page!`] macros — with full support for loops, conditionals, and pattern
matching inside your templates.

## Getting started

```toml
[dependencies]
tidos = "0.7.2"

# With Rocket integration:
# tidos = { version = "0.7.2", features = ["rocket"] }

# With Rocket + internationalization support:
# tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
```

## Core concepts

| Item | Description |
|---|---|
| [`view!`] | Renders a fragment of HTML. Returns a `String`. |
| [`page!`] | Wraps a full page. Returns a [`Page`] ready to return from a route. |
| [`Component`] | Trait for reusable components; implement [`to_render`](Component::to_render). |
| [`Page`] | Collects rendered HTML and `<head>` elements for a full page response. |
| [`scoped_css!`] | Injects a scoped `<style>` into `<head>` and returns the generated class name. |
| [`head!`] | Injects arbitrary HTML into the page `<head>`. |
| [`i18n::i18n!`] | *(feature: `i18n`)* Looks up a Fluent translation key. |
| [`i18n::enable_i18n!`] | *(feature: `i18n`)* Initialises the translation system in `main.rs`. |

## Defining a component

Implement [`Component`] on any struct and use [`view!`] inside
[`to_render`](Component::to_render):

```rust,no_run
use tidos::{view, Component, Page};

pub struct Card {
    pub title: String,
    pub body: String,
}

impl Component for Card {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class="card">
                <h2>{&self.title}</h2>
                <p>{&self.body}</p>
            </div>
        }
    }
}
```

## Building a page

Use [`page!`] in a route handler to produce a full [`Page`] response.
Embed components with JSX-like self-closing tags:

```rust,no_run
use tidos::{page, Component, Page};

#[get("/")]
pub fn index() -> Page {
    page! {
        <main>
            <Card title={ String::from("News") } body={ String::from("Hello world!") } />
        </main>
    }
}
```

## Template syntax

### Text content

All text between HTML tags must be wrapped in `{ }`. Raw text without
braces causes a compile-time panic. There are four forms:

| Form | When to use | Sanitized? |
|---|---|---|
| `{"literal"}` | Static string | No (emitted as-is) |
| `{"Hello {}", name}` | Format string with comma-separated params | Yes |
| `{expr}` | Any Rust expression | Yes |
| `@html{expr}` | Trusted raw HTML — not escaped | No |

```rust,no_run
use tidos::view;

let name = "Alice";
let count = 42_usize;

view! {
    <p>{"Hello world"}</p>
    <p>{"Hello {}, you have {} messages.", name, count}</p>
    <p>{count.to_string()}</p>
}
```

### `{#for}` — loops

```rust,no_run
use tidos::view;

let fruits = vec!["apple", "banana", "cherry"];

view! {
    <ul>
        {#for fruit in fruits}
            <li>{fruit}</li>
        {/for}
    </ul>
}
```

### `{#if}` — conditionals

```rust,no_run
use tidos::view;

let age = 20_u32;
let is_american = false;

view! {
    {#if age >= 18 && !is_american}
        <p>{"Allowed to drink."}</p>
    {:else if age >= 21 && is_american}
        <p>{"Allowed to drink (US rules)."}</p>
    {:else}
        <p>{"Not allowed to drink."}</p>
    {/if}
}
```

### `{#match}` — pattern matching

```rust,no_run
use tidos::view;

enum Status { Active, Banned, Guest }
let status = Status::Active;

view! {
    {#match status}
        {:case Status::Active}
            <span class="green">{"Active"}</span>
        {:case Status::Banned}
            <span class="red">{"Banned"}</span>
        {:case _}
            <span class="gray">{"Guest"}</span>
    {/match}
}
```

## Default trait support

Components that implement [`Default`] can use the `..` shorthand to fill
unspecified props with their default values — mirroring Rust's struct update
syntax:

```rust,no_run
use tidos::{view, Component, Page};

#[derive(Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Component for Coordinate {
    fn to_render(&self, _page: &mut Page) -> String {
        view! {
            <span>{self.x.to_string()}</span>
            <span>{self.y.to_string()}</span>
        }
    }
}

// Only set x; y defaults to 0
view! {
    <Coordinate x={1} .. />
}
```

The `..` flag is only valid on custom components, not on native HTML tags.

## Scoped CSS

[`scoped_css!`] reads a CSS file at compile time, generates a unique class
name, and injects a `<style>` block into the page `<head>`. The macro
returns the class name as a `&'static str`; apply it to the component's
root element. [`Page::add_elements_to_head`] deduplicates by UUID, so
calling `scoped_css!` inside a loop is safe.

```rust,no_run
use tidos::{scoped_css, view, Component, Page};

pub struct Card {
    pub title: String,
}

impl Component for Card {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class={scoped_css!("./card.css")}>
                <h2>{&self.title}</h2>
            </div>
        }
    }
}
```

Use CSS nesting so all styles live in one file:

```css
& {
    background: #16213e;
    border-radius: 8px;
}

& h2 {
    color: #a8dadc;
}
```

## Injecting `<head>` elements

Use [`head!`] inside `to_render` to add arbitrary HTML to the page
`<head>` (e.g. a `<title>` or a `<link rel="stylesheet">`):

```rust,no_run
use tidos::{head, Component, Page};

pub struct Title {
    pub title: String,
}

impl Component for Title {
    fn to_render(&self, page: &mut Page) -> String {
        head! {
            <title>{&self.title}</title>
        }
        String::new()
    }
}
```

## Internationalization

See the [`mod@i18n`] module for full details. Enable the feature flag and call
[`i18n::enable_i18n!`] once in `main.rs`:

```toml
tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
```

```rust,no_run
use tidos::i18n::{enable_i18n, i18n, Lang};
use tidos::{view, page, Component, Page};

enable_i18n!();

pub struct Greeting;

impl Component for Greeting {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <h1>{i18n!("greeting")}</h1>
        }
    }
}

#[get("/<lang>")]
pub fn index(lang: Lang) -> Page {
    page! { <Greeting /> }
}
```
