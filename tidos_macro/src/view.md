Renders an HTML template to a `String`.

Write HTML directly in Rust. All text between tags must be wrapped in `{ }`.
Use the control-flow blocks below for loops, conditionals, and pattern matching.

# Text content

| Form | Meaning | Sanitized? |
|---|---|---|
| `{"literal"}` | Static text | No (emitted as-is) |
| `{"Hello {}", name}` | Format string with comma-separated params | Yes |
| `{expr}` | Any Rust expression | Yes |
| `@html{expr}` | Raw unsanitized HTML | No |

# Syntax summary

| Syntax | Meaning |
|---|---|
| `{"literal"}` | Static text |
| `{"fmt {}", param}` | Formatted text |
| `{expr}` | Interpolate a Rust expression |
| `@html{expr}` | Raw HTML output |
| `{#for x in iter} … {/for}` | Loop |
| `{#if cond} … {:else if cond} … {:else} … {/if}` | Conditional |
| `{#match val} {:case Pat} … {/match}` | Pattern match |
| `<Component prop={expr} />` | Render a component |
| `<Component prop={expr} .. />` | Render a component, filling unset fields with `Default::default()` |

# Examples

## Text content
```rust,no_run
use tidos_macro::view;

let name = "Alice";
let count = 42_usize;

view! {
    <p>{"Hello world"}</p>
    <p>{"Hello {}, you have {} messages.", name, count}</p>
    <p>{count.to_string()}</p>
}
```

## For loop
```rust,no_run
use tidos_macro::view;

let names = vec!["Bob", "Alice"];

view! {
    {#for name in names}
        <p>{format!("Hello {}!", name)}</p>
    {/for}
}
```

## If / else
```rust,no_run
use tidos_macro::view;

let age = 18;
let is_american = false;

view! {
    {#if age >= 18 && !is_american}
        <p>{"User is allowed to drink."}</p>
    {:else if age >= 21 && is_american}
        <p>{"User is allowed to drink."}</p>
    {:else if age >= 18 && age < 21 && is_american}
        <p>{"User is probably the designated driver."}</p>
    {:else}
        <p>{"User is not allowed to drink."}</p>
    {/if}
}
```

## Match statement
```rust,no_run
use tidos_macro::view;

enum Pet { Fish, Dog, Cat, Other { name: String } }
use Pet::*;

let my_pet = Dog;

view! {
    {#match my_pet}
        {:case Fish}
            <p>{"Blub!"}</p>
        {:case Dog}
            <p>{"Who's a good boy!"}</p>
        {:case Cat}
            <p>{"Give all mortal possessions to cat!"}</p>
        {:case _}
            <p>{"Is it a snake or a spider?"}</p>
    {/match}
}
```

## Default trait (`..`)

Components whose struct implements [`Default`] can use `..` to fill any
unspecified props with their default values, mirroring Rust's struct update
syntax. The `..` must appear after all explicit props and is only valid on
custom components — native HTML tags do not support it.

```rust,no_run
use tidos_macro::view;
use tidos::{Component, Page};

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

// Sets x=1, y defaults to 0
view! { <Coordinate x={1} .. /> }

// All fields default
view! { <Coordinate .. /> }
```
