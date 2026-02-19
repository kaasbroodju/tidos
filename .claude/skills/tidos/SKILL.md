---
name: tidos
description: >
  Use this skill whenever working with the Tidos Rust component framework.
  Triggers: any mention of `tidos`, `view!`, `page!`, `Component`, `Page`,
  Tidos macros, or SSR templating in a Rust context. This skill explains the
  full Tidos syntax so Claude can write, read, and debug Tidos code correctly.
---

# Tidos Framework Skill

Tidos is a **Rust SSR component framework** (`crates.io: tidos`). It lets you
write HTML components directly in Rust using two procedural macros: `view!` and
`page!`. Components implement the `Component` trait; pages are produced by route
handlers and implement `Page`.

## Cargo.toml

```toml
[dependencies]
tidos = "0.7.2"            # or latest
# Optional: if using Rocket as the web framework
tidos = { version = "0.7.2", features = ["rocket"] }
# Optional: enable internationalization support (Fluent-based)
tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
```

---

## Core concepts

| Concept | What it is |
|---|---|
| `view!` | Macro that renders a fragment of HTML. Returns a `String`. |
| `page!` | Macro that wraps a full page. Returns a `Page` struct. |
| `Component` trait | Implement `to_render(&self, page: &mut Page) -> String` to make a struct a component. |
| `Page` | A top-level page object that collects rendered HTML and can be returned from route handlers. |
| `scoped_css!` | Injects a scoped `<style>` tag into the page `<head>` and returns the generated class name. |
| `i18n!` | *(feature: `i18n`)* Macro that looks up a Fluent translation key and returns a `String`. |
| `enable_i18n!` | *(feature: `i18n`)* Initialises the translation system; call once at the top level of `main.rs`. |
| `Lang` | *(feature: `i18n`)* Rocket request guard that extracts the locale from the URL path segment. |

---

## `view!` macro — syntax reference

`view!` is the primary templating macro. Inside it you write **HTML** with
**Rust expressions** and **control-flow blocks** using a special `{# ... }` /
`{/ ... }` syntax.

### Inline Rust expressions

Wrap any Rust expression in `{ }` to interpolate it into the HTML:

```rust
use tidos_macro::view;

let name = "Alice";
let count = 42;

view! {
    <p>Hello {name}, you have {count} messages.</p>
}
// → "<p>Hello Alice, you have 42 messages.</p>"
```

For method calls / format strings, just write normal Rust inside the braces:

```rust
view! {
    <p>{format!("Score: {:.1}", 9.5_f64)}</p>
    <p>{some_value.to_uppercase()}</p>
}
```

### `{#for … }` / `{/for}` — loops

```rust
use tidos_macro::view;

let fruits = vec!["apple", "banana", "cherry"];

view! {
    <ul>
        {#for fruit in fruits}
            <li>{fruit}</li>
        {/for}
    </ul>
}
```

- Opening tag: `{#for <pattern> in <iterator>}`
- Closing tag: `{/for}`
- The loop body is ordinary `view!` content.
- Pattern destructuring works: `{#for (k, v) in map}`

### `{#if … }` / `{:else if … }` / `{:else}` / `{/if}` — conditionals

```rust
use tidos_macro::view;

let age = 20;
let is_american = false;

view! {
    {#if age >= 18 && !is_american}
        <p>Allowed to drink.</p>
    {:else if age >= 21 && is_american}
        <p>Allowed to drink (US rules).</p>
    {:else if age >= 18 && age < 21 && is_american}
        <p>Probably the designated driver.</p>
    {:else}
        <p>Not allowed to drink.</p>
    {/if}
}
```

Rules:
- Opening: `{#if <bool_expr>}`
- Optional branches: `{:else if <bool_expr>}` (zero or more)
- Optional fallback: `{:else}`
- Always close with `{/if}`

### `{#match … }` / `{:case … }` / `{/match}` — pattern matching

```rust
use tidos_macro::view;

enum Status { Active, Banned, Guest }
let status = Status::Active;

view! {
    {#match status}
        {:case Status::Active}
            <span class="green">Active</span>
        {:case Status::Banned}
            <span class="red">Banned</span>
        {:case _}
            <span class="gray">Guest</span>
    {/match}
}
```

Rules:
- Opening: `{#match <expr>}`
- Each arm: `{:case <pattern>}` followed by HTML body
- Wildcards (`_`) and enum variants both work as patterns
- Close with `{/match}`

---

## `Component` trait

Define a struct, implement `Component`, use `view!` inside `to_render`:

```rust
use tidos::{Component, Page};
use tidos_macro::view;

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

Key points:
- `&self` gives access to the struct's fields.
- `page: &mut Page` is passed in automatically — use it to nest child
  components (call `child.to_render(page)`).
- The return value is a plain `String` of rendered HTML.

### Using a component inside `view!` / `page!`

```rust
// JSX-like syntax with named props:
view! {
    <Card title={ String::from("News") } body={ String::from("Hello world") } />
}
```

- Self-closing tag `<ComponentName prop={expr} />`
- Prop values are Rust expressions wrapped in `{ }`.

---

## `page!` macro and `Page`

`page!` wraps the whole page and returns a `Page` value that web frameworks
(e.g. Rocket) can return directly from route handlers:

```rust
use tidos::{Component, Page};
use tidos_macro::{page, view};

// With Rocket:
#[get("/")]
pub fn index() -> Page {
    page! {
        <main>
            <Greet name={ String::from("world") } />
        </main>
    }
}
```

`page!` accepts the same syntax as `view!` (HTML + Rust expressions +
control-flow blocks).

---

## Complete example

```rust
use tidos::{Component, Page};
use tidos_macro::{page, view};

// --- Components ---

pub struct NavBar {
    pub links: Vec<(String, String)>, // (label, href)
}

impl Component for NavBar {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <nav>
                <ul>
                    {#for (label, href) in &self.links}
                        <li><a href={href}>{label}</a></li>
                    {/for}
                </ul>
            </nav>
        }
    }
}

pub struct UserBadge {
    pub username: String,
    pub is_admin: bool,
}

impl Component for UserBadge {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class="badge">
                <span>{&self.username}</span>
                {#if self.is_admin}
                    <span class="admin-tag">Admin</span>
                {/if}
            </div>
        }
    }
}

// --- Route ---

#[get("/dashboard")]
pub fn dashboard() -> Page {
    let nav = NavBar {
        links: vec![
            ("Home".into(), "/".into()),
            ("Profile".into(), "/profile".into()),
        ],
    };
    let user = UserBadge {
        username: "kaasbroodju".into(),
        is_admin: true,
    };

    page! {
        <NavBar links={nav.links} />
        <UserBadge username={user.username} is_admin={user.is_admin} />
        <h1>Welcome to the dashboard</h1>
    }
}
```

---

## Syntax cheat sheet

| Syntax | Meaning |
|---|---|
| `{expr}` | Interpolate a Rust expression |
| `{#for x in iter} … {/for}` | Loop |
| `{#if cond} … {:else if cond} … {:else} … {/if}` | Conditional |
| `{#match val} {:case Pat} … {/match}` | Pattern match |
| `<Component prop={expr} />` | Render a component with props |
| `page! { … }` | Produce a full `Page` for a route |
| `view! { … }` | Produce an HTML `String` fragment |
| `scoped_css!("./component.css")` | Inject scoped CSS and return the class name |
| `i18n!("key")` | *(feature: `i18n`)* Look up a Fluent translation key |
| `i18n!("key", "var", val, …)` | *(feature: `i18n`)* Look up key with Fluent variables |
| `enable_i18n!()` | *(feature: `i18n`)* Initialise the translation system in `main.rs` |

---

## Common pitfalls

1. **Borrow vs. owned**: `view!` often needs `&self.field` (a reference) to
   avoid moving out of `self`. If you see a "value moved" error, add `&`.

2. **Closing tags**: every `{#for}`, `{#if}`, `{#match}` block **must** have a
   matching close tag (`{/for}`, `{/if}`, `{/match}`). Missing them causes
   compile errors.

3. **`{:else}` placement**: `{:else}` must come *after* all `{:else if}`
   branches and *before* `{/if}`.

4. **Prop types**: props passed to components with `<Component prop={expr} />`
   must match the struct field types exactly. Use `.into()`, `.clone()`, or
   `String::from(…)` as needed.

---

## Scoped CSS (`scoped_css!`)

`scoped_css!` is the idiomatic way to style components in Tidos. It reads a CSS
file at compile time, generates a unique UUID-based class name, and injects a
`<style>` tag into the page `<head>`. The macro returns the class name as a
`&'static str` so you can bind it to an element's `class` attribute.

```rust
use tidos::{scoped_css, view, Component, Page};

impl Component for MyCard {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <div class={scoped_css!("./my_card.css")}>
                <h2>{&self.title}</h2>
                <p>{&self.body}</p>
            </div>
        }
    }
}
```

### How it works

- The path is **relative to the source file** (like `include_str!`).
- The CSS file content is wrapped as `.tidos-<uuid> { … }` in the `<head>`.
- `Page::add_elements_to_head` **deduplicates by UUID**, so calling
  `scoped_css!` inside a `{#for}` loop is safe — the style is only injected
  once.
- `page` must be in scope. It is available in both `to_render` and inside
  `page! { … }` (the macro binds `page` internally).

### File & directory structure

Organise each component as a **directory** with a `mod.rs` and a CSS file
alongside it:

```
src/
├── main.rs
├── main.css
└── components/
    ├── mod.rs          ← pub mod nav_bar; pub mod card; …
    ├── nav_bar/
    │   ├── mod.rs      ← the NavBar component
    │   └── nav_bar.css
    └── card/
        ├── mod.rs      ← the Card component
        └── card.css
```

Rust resolves `pub mod nav_bar;` to either `nav_bar.rs` or `nav_bar/mod.rs`,
so the existing module declarations in `components/mod.rs` require no changes.

### CSS file format — one file per component

Use **CSS nesting** so the file has proper syntax highlighting. Wrap the root
element's declarations in `& { }` and target child elements with `& child { }`:

```css
/* card.css */
& {
    background: #16213e;
    border-radius: 8px;
    padding: 1.5rem;
}

& h2 {
    color: #a8dadc;
    margin-top: 0;
}

& p {
    color: #888;
    font-size: 0.9rem;
}
```

The generated `<style>` tag becomes:

```css
.tidos-<uuid> {
    & { background: #16213e; … }
    & h2 { color: #a8dadc; … }
    & p { color: #888; … }
}
```

Modern browsers (Chrome 112+, Firefox 117+, Safari 16.5+) support CSS nesting
natively.

### One `scoped_css!` per component

Apply the scoped class **once on the root element** of the component. Style all
children via CSS descendant selectors inside the single CSS file — the same
approach as Svelte's `<style>` block:

```rust
// ✓ correct — one scoped_css! on the root, children styled in CSS
view! {
    <nav class={scoped_css!("./nav_bar.css")}>
        <span>{&self.title}</span>
        <ul>…</ul>
    </nav>
}

// ✗ avoid — multiple scoped_css! calls, one per element
view! {
    <nav class={scoped_css!("./nav.css")}>
        <span class={scoped_css!("./title.css")}>{&self.title}</span>
    </nav>
}
```

### Styling variants with `data-*` attributes

When child elements of the same tag differ only in style (e.g. tier badges,
status indicators), use `data-*` attributes instead of extra CSS files or extra
`scoped_css!` calls:

```rust
// In the component template:
{#if self.score >= 9000}
    <span data-tier="legendary">LEGENDARY</span>
{:else if self.score >= 5000}
    <span data-tier="expert">EXPERT</span>
{:else}
    <span data-tier="newbie">NEWBIE</span>
{/if}
```

```css
/* In the CSS file — attribute selectors target each variant */
& span[data-tier="legendary"] { color: #ffd700; }
& span[data-tier="expert"]    { color: #c0c0c0; }
& span[data-tier="newbie"]    { color: #555; }
```

Hyphenated attribute names like `data-tier` and `data-status` are fully
supported by the `view!` macro parser.

---

## Internationalization (`i18n` feature flag)

> **Feature flag required.** Add `"i18n"` to the `features` list in `Cargo.toml`.
> The `i18n` feature uses [Fluent](https://projectfluent.org/) (`.ftl`) files for translations.

### 1. `Cargo.toml`

```toml
tidos = { version = "0.7.2", features = ["rocket", "i18n"] }
```

### 2. `Tidos.toml` — translation configuration

Create a `Tidos.toml` in the project root:

```toml
[default]
resource_location = "translations"
default_locale = "en-US"
resources = ["common.ftl"]
```

Place `.ftl` files under `resource_location`, organised by locale:

```
translations/
├── en-US/
│   └── common.ftl
└── nl-NL/
    └── common.ftl
```

### 3. Initialise in `main.rs`

Call `enable_i18n!()` once at the top level of `main.rs`:

```rust
use tidos::i18n::{Lang, enable_i18n};

enable_i18n!();

#[rocket::main]
async fn main() { /* … */ }
```

### 4. Add `lang: Lang` to route handlers

The locale is expected as the first path segment (e.g. `/en-US` or `/nl-NL`).
`page!` picks it up automatically when `lang: Lang` is a route parameter:

```rust
use tidos::i18n::Lang;
use tidos::{Page, page};

#[get("/<lang>")]
pub fn index(lang: Lang) -> Page {
    page! {
        <main>
            <Greeting />
        </main>
    }
}
```

### 5. Translate strings with `i18n!`

Import and call `i18n!` inside any component with a Fluent message key:

```rust
use tidos::i18n::i18n;

// Simple key lookup:
i18n!("greeting")

// With Fluent variables (alternating key-value pairs after the message key):
i18n!("shared-photos", "userName", "Anne", "userGender", "female", "photoCount", 3)
```

Full component example:

```rust
use tidos::{view, Component, Page};
use tidos::i18n::i18n;

pub struct Greeting;

impl Component for Greeting {
    fn to_render(&self, page: &mut Page) -> String {
        view! {
            <section>
                <h1>{i18n!("greeting")}</h1>
                <p>{i18n!("shared-photos", "userName", "Anne", "userGender", "female", "photoCount", 3)}</p>
            </section>
        }
    }
}
```

### Fluent file example

```ftl
greeting = Hello

shared-photos =
    {$userName} {$photoCount ->
        [one] added a new photo
       *[other] added {$photoCount} new photos
    } to {$userGender ->
        [male] his stream
        [female] her stream
       *[other] their stream
    }.
```

Fluent supports pluralization, gender variants, and other selectors out of the box.