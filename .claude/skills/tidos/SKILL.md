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
tidos = "0.8.0-rc.2"            # or latest
# Optional: if using Rocket as the web framework
tidos = { version = "0.8.0-rc.2", features = ["rocket"] }
# Optional: enable internationalization support (Fluent-based)
tidos = { version = "0.8.0-rc.2", features = ["rocket", "i18n"] }
```

---

## Core concepts

| Concept | What it is |
|---|---|
| `view!` | Macro that renders a fragment of HTML directly into a `page: &mut Page` binding already in scope. |
| `page!` | Macro that wraps a full page. Evaluates to a `Page` value. |
| `Component` trait | Implement `to_render(&self, page: &mut Page)` to make a struct a component — it renders directly into `page`, it does not return a `String`. |
| `Page` | A top-level page object that collects rendered HTML and can be returned from route handlers. |
| `@html{…}` | Inserts a pre-rendered HTML string or expression without escaping. |
| `Slot<'render>` | `Box<dyn Fn(&mut Page) + 'render>` — a boxed closure holding slot content, invoked with `@slot{...}`. |
| `{#slot:name}` | Named slot — fills a `Slot<'render>` field of the same name on a child component. |
| (children, no `{#slot:name}`) | Unnamed slot — fills a child component's tuple field `0`. Only for a single slot with no other props. |
| `scoped_css!` | Injects a scoped `<style>` tag into the page `<head>` and returns the generated class name. |
| `#[native_element]` | Attribute macro — auto-generates a `Component` impl that injects the JS `<script>` and renders the kebab-case custom element tag with all struct fields as attributes. |
| `i18n!` | *(feature: `i18n`)* Macro that looks up a Fluent translation key and returns a `String`. |
| `enable_i18n!` | *(feature: `i18n`)* Initialises the translation system; call once at the top level of `main.rs`. |
| `Lang` | *(feature: `i18n`)* Rocket request guard that extracts the locale from the URL path segment. |

---

## `view!` macro — syntax reference

`view!` is the primary templating macro. Inside it you write **HTML** with
**Rust expressions** and **control-flow blocks** using a special `{# ... }` /
`{/ ... }` syntax.

### Text content — `{"..."}`, `{expr}`, `@html{...}`

All text between HTML tags **must** be wrapped in `{ }`. Raw text without
braces causes a compile-time panic. There are four variants:

| Form | When to use | Sanitized? |
|---|---|---|
| `{"literal text"}` | Static string that never changes | No (emitted as-is) |
| `{"Hello {}", name}` | Format-string with comma-separated Rust params | Yes |
| `{expr}` | Any Rust expression | Yes |
| `@html{"literal html"}` | Static raw HTML string | No |
| `@html{"<b>{}</b>", expr}` | Formatted raw HTML | No |
| `@html{expr}` | Any Rust expression producing raw HTML | No |

```rust
use tidos_macro::view;

let name = "Alice";
let count = 42_usize;
let raw = "<em>emphasized</em>";

view! {
    // static text — emitted as-is
    <p>{"Hello world"}</p>

    // formatted text — comma-separated params, sanitized
    <p>{"Hello {}, you have {} messages.", name, count}</p>

    // Rust expression — sanitized
    <p>{count.to_string()}</p>

    // raw HTML — literal, not sanitized
    <div>@html{"<b>bold</b>"}</div>

    // raw HTML — formatted, not sanitized
    <div>@html{"<b>{}</b>", name}</div>

    // raw HTML — expression, not sanitized
    <div>@html{raw}</div>
}
```

> **WRONG** — raw text between tags panics at compile time:
> ```rust
> view! { <p>Hello world</p> }  // ✗ panics
> ```

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
        <p>{"Allowed to drink."}</p>
    {:else if age >= 21 && is_american}
        <p>{"Allowed to drink (US rules)."}</p>
    {:else if age >= 18 && age < 21 && is_american}
        <p>{"Probably the designated driver."}</p>
    {:else}
        <p>{"Not allowed to drink."}</p>
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
            <span class="green">{"Active"}</span>
        {:case Status::Banned}
            <span class="red">{"Banned"}</span>
        {:case _}
            <span class="gray">{"Guest"}</span>
    {/match}
}
```

Rules:
- Opening: `{#match <expr>}`
- Each arm: `{:case <pattern>}` followed by HTML body
- Wildcards (`_`) and enum variants both work as patterns
- Close with `{/match}`

### Slots — named and unnamed

Slots let you pass **rendered content** into a child component, similar to
slots in Vue or Svelte. This is useful when a parent component wants to
provide complex, dynamic content (including loops and nested components) that
the child component renders inside its own template.

Slot content is held as a `Slot<'render>` field — a boxed closure
(`Box<dyn Fn(&mut Page) + 'render>`), not a `String`. The child renders it with
`@slot{...}`, which calls the closure directly into `page`. There are two
shapes, and the choice isn't stylistic — it's dictated by the component:

| Use | When |
|---|---|
| **Unnamed** slot — tuple struct, `Slot<'render>` as field `0` | The component takes **exactly one** slot and **no other props** |
| **Named** slot(s) — regular `Slot<'render>` field(s) | The component has **multiple** slots, **or** one slot **alongside other props/attributes** |

An unnamed (tuple) field can't sit next to named fields in the same struct
literal, which is why a single slot only stays unnamed when there's nothing
else to pass in.

#### Unnamed slot

**Child side** — a tuple struct with `Slot<'render>` as field `0`:

```rust
pub struct Card<'a>(pub Slot<'a>);

impl Component for Card<'_> {
    fn to_render(&self, page: &mut Page) {
        view! {
            <div class="card">
                @slot{self.0}
            </div>
        }
    }
}
```

**Parent side** — pass content directly as children, with **no**
`{#slot:name}` wrapper:

```rust
view! {
    <Card>
        <p>{"This content goes straight into the card."}</p>
    </Card>
}
```

#### Named slot(s)

**Child side** — a named `Slot<'render>` field per slot (here combined with a
`title` prop, which is why this one can't be unnamed):

```rust
pub struct Card<'a> {
    pub title: String,
    pub body: Slot<'a>,
}

impl Component for Card<'_> {
    fn to_render(&self, page: &mut Page) {
        view! {
            <div class="card">
                <h2>{&self.title}</h2>
                <div class="card-body">
                    @slot{self.body}
                </div>
            </div>
        }
    }
}
```

**Parent side** — wrap content in `{#slot:name} … {/slot}` inside the
component's opening and closing tags:

```rust
view! {
    <Card title={String::from("My Card")}>
        {#slot:body}
            <p>{"This content is passed to the Card's body prop."}</p>
            <p>{"You can use loops, conditionals, and nested components here."}</p>
        {/slot}
    </Card>
}
```

Rules:
- Opening: `{#slot:<name>}` where `<name>` matches a `Slot<'render>` field on
  the child component's struct. Closing: `{/slot}`.
- The slot body is ordinary `view!` content — loops, conditionals, nested
  components, and expressions all work inside slots.
- The child renders slot content with `@slot{self.<name>}` (named) or
  `@slot{self.0}` (unnamed) — **never** `{self.<name>}` or `@html{self.<name>}`,
  since the field is a closure, not a string.
- A component can accept **multiple named slots** — just add multiple
  `Slot<'render>` fields and corresponding `{#slot:name}` blocks.
- **Don't mix the two at the top level of one tag.** If the first child of a
  component tag is a `{#slot:name}` block, the macro treats the whole tag as
  named-slot mode and silently drops any other top-level children that aren't
  `{#slot:...}` blocks. Keep it consistent: either wrap everything in
  `{#slot:name}` blocks, or use none.

Full example (table with a named slotted body, alongside a `headers` prop):

```rust
// --- Parent component ---
view! {
    <LeaderboardTable headers={headers}>
        {#slot:body}
            {#for (i, player) in players.iter().enumerate()}
                <PlayerRow
                    rank={i + 1}
                    name={player.name.clone()}
                    score={player.score}
                />
            {/for}
        {/slot}
    </LeaderboardTable>
}

// --- Child component ---
pub struct LeaderboardTable<'a> {
    pub headers: Vec<String>,
    pub body: Slot<'a>,
}

impl Component for LeaderboardTable<'_> {
    fn to_render(&self, page: &mut Page) {
        view! {
            <table>
                <thead>
                    <tr>
                        {#for title in &self.headers}
                            <th>{title}</th>
                        {/for}
                    </tr>
                </thead>
                <tbody>
                    @slot{self.body}
                </tbody>
            </table>
        }
    }
}
```

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
    fn to_render(&self, page: &mut Page) {
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
- `to_render` renders directly into `page` — it returns `()`, not a `String`.

### Using a component inside `view!` / `page!`

```rust
// JSX-like syntax with named props:
view! {
    <Card title={ String::from("News") } body={ String::from("Hello world") } />
}
```

- Self-closing tag `<ComponentName prop={expr} />`
- Prop values are Rust expressions wrapped in `{ }`.

### Default trait (`..`)

Components whose struct implements `Default` can use `..` after explicit props
to fill the remaining fields with their default values — identical to Rust's
struct update syntax. The component struct must `#[derive(Default)]` or
manually implement `Default`.

```rust
#[derive(Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Component for Coordinate {
    fn to_render(&self, _page: &mut Page) {
        view! {
            <span>{self.x.to_string()}</span>
            <span>{self.y.to_string()}</span>
        }
    }
}

// x=1, y defaults to 0
view! { <Coordinate x={1} .. /> }

// all fields default
view! { <Coordinate .. /> }
```

Rules:
- `..` must appear after all explicit props, before `/>`.
- Only valid on custom components — native HTML tags (`<img .. />`) cause a compile error.
- The struct must implement `Default`.

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
    fn to_render(&self, page: &mut Page) {
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
    fn to_render(&self, page: &mut Page) {
        view! {
            <div class="badge">
                <span>{&self.username}</span>
                {#if self.is_admin}
                    <span class="admin-tag">{"Admin"}</span>
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
        <h1>{"Welcome to the dashboard"}</h1>
    }
}
```

---

## Syntax cheat sheet

| Syntax | Meaning |
|---|---|
| `{"literal text"}` | Static text — emitted as-is |
| `{"Hello {}", name}` | Formatted text — sanitized |
| `{expr}` | Rust expression — sanitized |
| `@html{"<b>literal</b>"}` | Raw HTML literal — not sanitized |
| `@html{"<b>{}</b>", expr}` | Raw HTML formatted — not sanitized |
| `@html{expr}` | Raw HTML expression — not sanitized |
| `{#for x in iter} … {/for}` | Loop |
| `{#if cond} … {:else if cond} … {:else} … {/if}` | Conditional |
| `{#match val} {:case Pat} … {/match}` | Pattern match |
| `{#slot:name} … {/slot}` | Named slot — fills a `Slot<'a>` field of the same name on a child component |
| `<Component>…</Component>` (no `{#slot:name}`) | Unnamed slot — fills a child component's tuple field `0`; only when the component has one slot and no other props |
| `@slot{self.field}` / `@slot{self.0}` | Render a `Slot<'a>` field's content (child side, named / unnamed) |
| `<Component prop={expr} />` | Render a component with props |
| `<Component prop={expr} .. />` | Render a component, filling unset fields with `Default::default()` |
| `page! { … }` | Produce a full `Page` for a route |
| `view! { … }` | Render an HTML fragment into `page` (in scope) |
| `scoped_css!("./component.css")` | Inject scoped CSS and return the class name |
| `#[native_element]` | Auto-generate `Component` for a Custom Element wrapper struct |
| `i18n!("key")` | *(feature: `i18n`)* Look up a Fluent translation key |
| `i18n!("key", ("var", val), …)` | *(feature: `i18n`)* Look up key with Fluent variables |
| `enable_i18n!()` | *(feature: `i18n`)* Initialise the translation system in `main.rs` |

---

## Common pitfalls

1. **Raw text between tags panics**: All text must be in `{}`. Use `{"Hello"}`,
   `{"Hello {}", name}`, or `{expr}`. Bare text like `<p>Hello</p>` causes a
   compile-time panic.

2. **Borrow vs. owned**: `view!` often needs `&self.field` (a reference) to
   avoid moving out of `self`. If you see a "value moved" error, add `&`.

3. **Closing tags**: every `{#for}`, `{#if}`, `{#match}` block **must** have a
   matching close tag (`{/for}`, `{/if}`, `{/match}`). Missing them causes
   compile errors.

4. **`{:else}` placement**: `{:else}` must come *after* all `{:else if}`
   branches and *before* `{/if}`.

5. **Prop types**: props passed to components with `<Component prop={expr} />`
   must match the struct field types exactly. Use `.into()`, `.clone()`, or
   `String::from(…)` as needed.

6. **Slots render with `@slot`, not `@html`**: a component receiving slot
   content must invoke it with `@slot{self.field}` (named) or `@slot{self.0}`
   (unnamed) — this calls the boxed closure. `{self.field}` or
   `@html{self.field}` won't work; `Slot<'a>` is a closure, not a `String`.

7. **Slot field type**: the field that receives slot content must be
   `Slot<'a>` (`Box<dyn Fn(&mut Page) + 'a>`). Use a named field when the
   component has multiple slots or a slot alongside other props; use a tuple
   struct field `0` only for a single slot with no other props.

8. **Mixing slotted and un-slotted children**: if a component tag's first
   child is a `{#slot:name}` block, the whole tag is treated as named-slot
   mode — any other top-level children that aren't `{#slot:...}` blocks are
   silently dropped, not rendered. Don't mix the two forms in one tag.

9. **Numeric types**: `{self.score}` where `score` is `u32`/`usize` fails
   because sanitize requires `AsRef<str>`. Use `{self.score.to_string()}`.

---

## Scoped CSS (`scoped_css!`)

`scoped_css!` is the idiomatic way to style components in Tidos. It reads a CSS
file at compile time, generates a unique UUID-based class name, and injects a
`<style>` tag into the page `<head>`. The macro returns the class name as a
`&'static str` so you can bind it to an element's `class` attribute.

```rust
use tidos::{scoped_css, view, Component, Page};

impl Component for MyCard {
    fn to_render(&self, page: &mut Page) {
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
    <span data-tier="legendary">{"LEGENDARY"}</span>
{:else if self.score >= 5000}
    <span data-tier="expert">{"EXPERT"}</span>
{:else}
    <span data-tier="newbie">{"NEWBIE"}</span>
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
i18n!("shared-photos", ("userName", "Anne"), ("userGender", "female"), ("photoCount", 3))
```

Full component example:

```rust
use tidos::{view, Component, Page};
use tidos::i18n::i18n;

pub struct Greeting;

impl Component for Greeting {
    fn to_render(&self, page: &mut Page) {
        view! {
            <section>
                <h1>{i18n!("greeting")}</h1>
                <p>{i18n!("shared-photos", ("userName", "Anne"), ("userGender", "female"), ("photoCount", 3))}</p>
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