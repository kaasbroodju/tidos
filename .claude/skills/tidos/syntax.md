# Tidos Syntax Reference

Full syntax for `view!`, `page!`, the `Component` trait, HTML tag attributes,
`scoped_css!`, and `#[native_element]`. Read this whenever you're writing,
reading, or debugging Tidos template code. For opinionated dos/don'ts and
common compile errors, see `best_practices.md`. For i18n, see `i18n.md`. For
wiring up a JS framework via `#[native_element]`, see `reactivity/`.

---

## Text content — `{"..."}`, `{expr}`, `@html{...}`

All text between HTML tags **must** be wrapped in `{ }`. Raw text without
braces causes a compile-time panic. There are six forms:

| Form | When to use | Sanitized? |
|---|---|---|
| `{"literal text"}` | Static string that never changes | No (emitted as-is) |
| `{"Hello {}", name}` | Format-string with comma-separated Rust params | Yes |
| `{expr}` | Any Rust expression | Yes |
| `@html{"literal html"}` | Static raw HTML string | No |
| `@html{"<b>{}</b>", expr}` | Formatted raw HTML | No |
| `@html{expr}` | Any Rust expression producing raw HTML | No |

```rust
use tidos::view;

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
> view! { <p>Hello world</p> }  // panics
> ```

## HTML tag attributes

An attribute on a native HTML tag (`<input ...>`) comes in five forms:

| Form | Meaning |
|---|---|
| `disabled` | Bare name, rendered as-is, no value |
| `class="wrapper"` | String literal value |
| `value={expr}` | Braced value, sanitized |
| `:disabled` | Toggle attribute, present only when `true` |
| `:disabled={expr}` | Toggle attribute driven by a boolean expression |

A braced (non-toggle) attribute value (`name={...}`) is parsed as the same
grammar as text content, so it accepts all three of its sub-forms, not just
a plain expression:

| Sub-form | Example |
|---|---|
| Literal | `class={"wrapper"}` |
| Format-string, comma-separated params | `class={"{} data-tidos", scoped_css!("mod.css")}` |
| Expression | `value={expr}` |

A **toggle** attribute's braced value (`:name={...}`) is more restricted: it
must be a plain boolean expression, none of the literal/format-string sugar
applies there. The leading `:` marks the attribute as a toggle: it's
rendered conditionally (present with no value when true, omitted entirely
when false) rather than as `name="value"`. This is the form
`#[native_element]` generates for `bool`-typed struct fields (see below).

```rust
view! {
    <input :disabled={self.is_locked} placeholder="Name" />
    <button :disabled>{"Always disabled"}</button>
}
```

## `{#for … }` / `{/for}` — loops

```rust
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

- Opening tag: `{#for <pattern> in <iterator>}`
- Closing tag: `{/for}`
- The loop body is ordinary `view!` content, a single child per iteration.
- Pattern destructuring works: `{#for (k, v) in map}`

## `{#if … }` / `{:else if … }` / `{:else}` / `{/if}` — conditionals

```rust
use tidos::view;

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
- Optional fallback: `{:else}`, must come after all `{:else if}` branches
- Always close with `{/if}`
- Each branch renders exactly **one** `Content` item, see `best_practices.md`
  if you need more than one element per branch.

## `{#match … }` / `{:case … }` / `{/match}` — pattern matching

```rust
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

Rules:
- Opening: `{#match <expr>}`
- Each arm: `{:case <pattern>}` followed by HTML body
- Wildcards (`_`) and enum variants both work as patterns
- Close with `{/match}`
- Each arm renders exactly **one** `Content` item, same rule as `{#if}`.

## Slots — named and unnamed

Slots let you pass **rendered content** into a child component, similar to
slots in Vue or Svelte. This is useful when a parent component wants to
provide complex, dynamic content (including loops and nested components)
that the child component renders inside its own template.

Slot content is held as a `Slot<'render>` field, a boxed closure
(`Box<dyn Fn(&mut Page) + 'render>`), not a `String`. The child renders it
with `@slot{...}`, which calls the closure directly into `page`. There are
two shapes, and the choice isn't stylistic, it's dictated by the component:

| Use | When |
|---|---|
| **Unnamed** slot, tuple struct, `Slot<'render>` as field `0` | The component takes **exactly one** slot and **no other props** |
| **Named** slot(s), regular `Slot<'render>` field(s) | The component has **multiple** slots, **or** one slot **alongside other props/attributes** |

An unnamed (tuple) field can't sit next to named fields in the same struct
literal, which is why a single slot only stays unnamed when there's nothing
else to pass in.

### Unnamed slot

**Child side** — a tuple struct with `Slot<'render>` as field `0`:

```rust
use tidos::{view, Component, Page, Slot};

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

### Named slot(s)

**Child side** — a named `Slot<'render>` field per slot (here combined with
a `title` prop, which is why this one can't be unnamed):

```rust
use tidos::{view, Component, Page, Slot};

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
- The slot body is ordinary `view!` content, loops, conditionals, nested
  components, and expressions all work inside slots.
- The child renders slot content with `@slot{self.<name>}` (named) or
  `@slot{self.0}` (unnamed), **never** `{self.<name>}` or
  `@html{self.<name>}`, since the field is a closure, not a string.
- A component can accept **multiple named slots**, just add multiple
  `Slot<'render>` fields and corresponding `{#slot:name}` blocks.
- **Don't mix the two at the top level of one tag.** If the first child of a
  component tag is a `{#slot:name}` block, the macro treats the whole tag as
  named-slot mode and silently drops any other top-level children that
  aren't `{#slot:...}` blocks. Keep it consistent: either wrap everything in
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

## `Component` trait

Define a struct, implement `Component`, use `view!` inside `to_render`:

```rust
use tidos::{view, Component, Page};

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
- `&self` gives access to the struct's fields; use `&self.field` rather than
  moving out of `self`.
- `page: &mut Page` is passed in automatically, use it to nest child
  components (`child.to_render(page)`), inject `<head>` elements, or inject
  scoped CSS.
- `to_render` renders directly into `page`; it returns `()`, not a `String`.

### Using a component inside `view!` / `page!`

```rust
view! {
    <Card title={ String::from("News") } body={ String::from("Hello world") } />
}
```

- Self-closing tag: `<ComponentName prop={expr} />`
- Prop values are Rust expressions wrapped in `{ }`.
- `Component` must be in scope wherever a component tag is used, the macro
  calls `.to_render(page)` on it directly.

### Default trait (`..`)

Components whose struct implements `Default` can use `..` after explicit
props to fill the remaining fields with their default values, identical to
Rust's struct update syntax:

```rust
use tidos::{view, Component, Page};

#[derive(Default)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Component for Coordinate {
    fn to_render(&self, page: &mut Page) {
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
- Only valid on custom components, native HTML tags (`<img .. />`) cause a
  compile error.
- The struct must implement `Default`.

---

## `page!` macro and `Page`

`page!` wraps the whole page and returns a `Page` value that web frameworks
(Rocket, Actix, Axum, Warp, or a bare binary) can turn into a response:

```rust
use tidos::{page, Component, Page};

#[get("/")]
pub fn index() -> Page {
    page! {
        <main>
            <Greet name={ String::from("world") } />
        </main>
    }
}
```

`page!` accepts the same syntax as `view!` (HTML, Rust expressions, control
flow, slots) and binds `page` internally, so `scoped_css!`/`head!` calls at
the top level of a `page! { … }` block work without extra setup.

---

## Complete example

```rust
use tidos::{page, view, Component, Page};

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
| `{"literal text"}` | Static text, emitted as-is |
| `{"Hello {}", name}` | Formatted text, sanitized |
| `{expr}` | Rust expression, sanitized |
| `@html{"<b>literal</b>"}` | Raw HTML literal, not sanitized |
| `@html{"<b>{}</b>", expr}` | Raw HTML formatted, not sanitized |
| `@html{expr}` | Raw HTML expression, not sanitized |
| `attr={expr}` | HTML attribute with a Rust expression value |
| `:attr` / `:attr={expr}` | Toggle attribute, present only when true |
| `{#for x in iter} … {/for}` | Loop |
| `{#if cond} … {:else if cond} … {:else} … {/if}` | Conditional |
| `{#match val} {:case Pat} … {/match}` | Pattern match |
| `{#slot:name} … {/slot}` | Named slot, fills a `Slot<'a>` field of the same name on a child component |
| `<Component>…</Component>` (no `{#slot:name}`) | Unnamed slot, fills a child component's tuple field `0`; only when the component has one slot and no other props |
| `@slot{self.field}` / `@slot{self.0}` | Render a `Slot<'a>` field's content (child side, named / unnamed) |
| `<Component prop={expr} />` | Render a component with props |
| `<Component prop={expr} .. />` | Render a component, filling unset fields with `Default::default()` |
| `page! { … }` | Produce a full `Page` for a route |
| `view! { … }` | Render an HTML fragment into `page` (in scope) |
| `head! { … }` | Inject HTML into the page `<head>`, deduplicated by UUID |
| `scoped_css!("./component.css")` | Inject scoped CSS and return the class name |
| `#[native_element]` | Auto-generate `Component` for a Custom Element wrapper struct |

---

## Scoped CSS (`scoped_css!`)

`scoped_css!` is the idiomatic way to style components in Tidos. It reads a
CSS file at compile time, generates a unique UUID-based class name, and
injects a `<style>` tag into the page `<head>`. The macro returns the class
name as a `&'static str` so you can bind it to an element's `class`
attribute.

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
  `scoped_css!` inside a `{#for}` loop is safe, the style is only injected
  once.
- `page` must be in scope. It is available in both `to_render` and inside
  `page! { … }` (the macro binds `page` internally).

### File & directory structure

Organise each component as a **directory** with a `mod.rs` and a CSS file
alongside it. Keep route handlers that produce a `page!` in their own
`src/pages/` tree, one file per page, mirrored by a `pages/mod.rs` that
re-exports them:

```
src/
├── main.rs
├── main.css
├── components/
│   ├── mod.rs          # pub mod nav_bar; pub mod card; …
│   ├── nav_bar/
│   │   ├── mod.rs      # the NavBar component
│   │   └── nav_bar.css
│   └── card/
│       ├── mod.rs      # the Card component
│       └── card.css
└── pages/
    ├── mod.rs          # pub mod index; pub mod docs; pub mod news; …
    ├── index.rs        # #[get("/")] fn index() -> Page { page! { … } }
    ├── docs/
    │   └── …            # one file (or submodule) per doc page
    └── news/
        └── …            # one file (or submodule) per news page
```

Rust resolves `pub mod nav_bar;` to either `nav_bar.rs` or `nav_bar/mod.rs`,
so the existing module declarations in `components/mod.rs` (or
`pages/mod.rs`) require no changes when a component or page moves from a
flat file into a directory.

Each page file owns one route handler returning a `Page`: the `page! { … }`
body plus any `head! { … }` calls for that page's metadata (title,
`<meta>` tags, structured data). Pages compose components from
`src/components/` rather than defining their own markup inline.

### CSS file format, one file per component

Use **CSS nesting** so the file has proper syntax highlighting. Wrap the
root element's declarations in `& { }` and target child elements with
`& child { }`:

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

Modern browsers (Chrome 112+, Firefox 117+, Safari 16.5+) support CSS
nesting natively. See `best_practices.md` for when to reach for a single
`scoped_css!` vs. `data-*` attribute variants.

---

## `head!` — injecting `<head>` elements

Call `head!` inside `Component::to_render` to add elements such as
`<title>`, `<link>`, or `<meta>` tags to the page `<head>`. Each call is
deduplicated by a compile-time UUID, so it is safe to call from components
rendered inside a loop.

```rust
use tidos::{head, Component, Page};

pub struct Title {
    pub title: String,
}

impl Component for Title {
    fn to_render(&self, page: &mut Page) {
        head! {
            <title>{&self.title}</title>
        }
    }
}
```

---

## `#[native_element]` — wrapping a JS Custom Element

`#[native_element]` derives `Component` for a struct that wraps a native
[Custom Element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements).
This is how Tidos brings in client-side reactivity: Tidos itself has no
concept of client state, so interactive components are written in whatever
JS framework you like, compiled to a Custom Element, and wrapped in a
type-checked Rust struct.

```rust
use tidos::native_element;

#[native_element]
pub struct GreetUser {
    pub name: String,
    pub is_admin: bool,
}
```

Placing `#[native_element]` on a struct generates a `Component` impl that:

1. Injects `<script type="module" src="/dist/StructName.js"></script>` into
   the page `<head>`.
2. Renders the kebab-case HTML tag derived from the struct name, forwarding
   every field as an HTML attribute.

Attribute mapping:
- Field names convert from `snake_case` to `kebab-case`
  (`initial_name` → `initial-name`).
- The struct name converts from `PascalCase` to `kebab-case` for the tag,
  and is used as-is (`PascalCase`) for the JS filename
  (`GreetUser` → `<greet-user>` served from `/dist/GreetUser.js`).
- `bool` fields render as a **toggle** attribute (`:is-admin={self.is_admin}`,
  see "HTML tag attributes" above), present only when `true`.
- The struct name must contain at least one word boundary (produce a
  hyphen once kebab-cased); single-word names like `Greeter` fail to
  compile, since custom element tag names are required to contain a `-`.

Roughly expands to:

```rust
impl tidos::Component for GreetUser {
    fn to_render(&self, page: &mut tidos::Page) {
        tidos::head! { <script type="module" src="/dist/GreetUser.js"></script> }
        tidos::view! { <greet-user name={self.name} :is-admin={self.is_admin}></greet-user> }
    }
}
```

For a complete, reproducible setup per JS framework (project structure,
package.json, vite.config.js, the component itself, and the Rust wrapper),
see `reactivity/svelte.md`, `reactivity/lit.md`, `reactivity/angular.md`,
`reactivity/vue.md`, and `reactivity/react.md`.
