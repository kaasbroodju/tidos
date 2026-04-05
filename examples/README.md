# Tidos Examples

A collection of example projects that demonstrate how to use [Tidos](https://crates.io/crates/tidos) — a Rust SSR component framework.

Each example is a self-contained Cargo project. Run any of them with:

```bash
cargo run --manifest-path examples/<name>/Cargo.toml
```

Then open [http://localhost:8000](http://localhost:8000).

---

## Examples

| Example                                            | Description | Concepts |
|----------------------------------------------------|-------------|----------|
| [simple_rocket](./simple_rocket)                   | A Rocket server with a NavBar, a Leaderboard, and scoped CSS. Good starting point. | `page!`, `view!`, `Component`, `scoped_css!`, for-loops, conditionals |
| [internationalization](./internationalization)     | Multi-locale pages using Fluent `.ftl` translation files. | `i18n!`, `enable_i18n!`, `Lang`, `Tidos.toml`, Fluent pluralization |
| [web_frameworks/react](./web_frameworks/react)     | React component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/vue](./web_frameworks/vue)         | Vue component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/lit](./web_frameworks/lit)         | LitElement component embedded in a Tidos page — no compile step beyond bundling. | LitElement, Shadow DOM, Custom Elements, Vite |
| [web_frameworks/svelte](./web_frameworks/svelte)   | Svelte component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/angular](./web_frameworks/angular) | Angular standalone component registered as a Custom Element via `@angular/elements`. | `@angular/elements`, signals, AOT, Custom Elements, Vite |

---

## Feature flags

Some examples require optional feature flags in `Cargo.toml`:

| Feature | Used by | What it enables |
|---------|---------|-----------------|
| `rocket` | all examples | Rocket web framework integration (`Page` as a Rocket responder) |
| `i18n` | `internationalization` | Fluent-based translation support (`i18n!`, `Lang`, `enable_i18n!`) |

---

## Web framework examples

The examples under `web_frameworks/` all follow the same pattern:

1. A frontend component is compiled to a self-contained JS module in `dist/` using **Vite**.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` into `<head>` and renders the HTML tag.
3. The frontend framework handles reactivity client-side — Tidos only renders the tag.

For these examples, install Node dependencies and build the JS before running Cargo:

```bash
npm install
npm run build
cargo run
```