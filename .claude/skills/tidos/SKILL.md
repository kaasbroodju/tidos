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
write HTML components directly in Rust using two procedural macros: `view!`
and `page!`. Components implement the `Component` trait; pages are produced
by route handlers and are a `Page` value.

This skill is split across files. Read the one relevant to the task at
hand, don't load all of them speculatively.

## Cargo.toml

```toml
[dependencies]
tidos = "0.8.0-rc.3"            # or latest

# Pick the web framework integration you need (or none, for a bare binary):
# tidos = { version = "0.8.0-rc.3", features = ["rocket"] }
# tidos = { version = "0.8.0-rc.3", features = ["actix-web"] }
# tidos = { version = "0.8.0-rc.3", features = ["axum"] }
# tidos = { version = "0.8.0-rc.3", features = ["warp"] }

# Add i18n support on top of any of the above:
# tidos = { version = "0.8.0-rc.3", features = ["rocket", "i18n"] }
```

## Core concepts

| Concept | What it is |
|---|---|
| `view!` | Renders a fragment of HTML directly into a `page: &mut Page` binding already in scope. |
| `page!` | Wraps a full page. Evaluates to a `Page` value. |
| `Component` trait | Implement `to_render(&self, page: &mut Page)` to make a struct a component, it renders directly into `page`, it does not return a `String`. |
| `Page` | Collects rendered HTML and `<head>` elements; returned from route handlers. |
| `@html{…}` | Inserts a pre-rendered HTML string or expression without escaping. |
| `Slot<'render>` | `Box<dyn Fn(&mut Page) + 'render>`, holds slot content, invoked with `@slot{...}`. |
| `{#slot:name}` | Named slot, fills a `Slot<'render>` field of the same name on a child component. |
| `scoped_css!` | Injects a scoped `<style>` tag into the page `<head>` and returns the generated class name. |
| `@class="file.css"` | `class`-only shorthand for `scoped_css!` on native HTML tags; generates the class name as a plain literal instead of an opaque expression. |
| `head!` | Injects arbitrary HTML into the page `<head>`, deduplicated by UUID. |
| `#[native_element]` | Auto-generates a `Component` impl that injects a JS `<script>` and renders a kebab-case custom element tag with struct fields as attributes. |
| `i18n!` | *(feature: `i18n`)* Looks up a Fluent translation key. |
| `enable_i18n!` | *(feature: `i18n`)* Initialises the translation system in `main.rs`. |

## File map

- **`syntax.md`** — the full `view!`/`page!` syntax reference: text content
  forms, HTML tag attributes (including `:toggle` attributes), `{#for}`,
  `{#if}`, `{#match}`, slots, the `Component` trait, `Default`'s `..`
  shorthand, `scoped_css!`, `head!`, and `#[native_element]` mechanics, plus
  a complete example and a syntax cheat sheet. Read this whenever writing or
  debugging Tidos template code.

- **`best_practices.md`** — common compile-error pitfalls and opinionated
  styling/structure conventions (one `scoped_css!` per component, `data-*`
  attribute variants). Read this to sanity-check finished code, or when a
  compile error looks like one of the known pitfalls.

- **`i18n.md`** — the `i18n` feature flag end to end: `Tidos.toml`,
  `enable_i18n!`, the `Lang` request guard, `i18n!` usage, and a Fluent
  (`.ftl`) file example. Read this whenever a task involves translations or
  locales.

- **`reactivity/svelte.md`**, **`reactivity/lit.md`**,
  **`reactivity/angular.md`**, **`reactivity/vue.md`**,
  **`reactivity/react.md`** — a complete, reproducible setup per JS
  framework (project structure, `package.json`, `vite.config.js`, the
  component itself, and the `#[native_element]` Rust wrapper) for adding
  client-side interactivity via a Custom Element. Read the one matching the
  framework asked for when scaffolding a new integration or a new project
  from scratch; each file is self-contained enough to reproduce the whole
  setup on its own.

If unsure which file covers something, `syntax.md` is the right default,
it covers everything except i18n and JS-framework reactivity.
