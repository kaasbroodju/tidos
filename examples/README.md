# Tidos Examples

A collection of example projects that demonstrate how to use [Tidos](https://crates.io/crates/tidos) — a Rust SSR component framework.

Each example is a self-contained Cargo project. Run any of them with:

```bash
cargo run --manifest-path examples/<name>/Cargo.toml
```

Then open [http://localhost:8000](http://localhost:8000).

---

## Examples
* [HTTP frameworks](#http-framework-examples)
* [Web frameworks](#web-framework-examples)
* [Internationalization](#internationalization)

---

## HTTP framework examples

These examples all render the same leaderboard page and are otherwise identical — the only difference is which HTTP framework serves the response. Each one shows how `Page` integrates with that framework's response type.

| Example | Framework                                       | Tidos feature flag | Concepts |
|---|-------------------------------------------------|---|---|
| [http_frameworks/rocket](./http_frameworks/rocket) | [Rocket](https://rocket.rs) 0.5                 | `rocket` | `Page` as `Responder`, `page!`, `view!`, `Component`, `scoped_css!` |
| [http_frameworks/axum](./http_frameworks/axum) | [Axum](https://github.com/tokio-rs/axum) 0.8    | `axum` | `Page` as `IntoResponse`, async handlers |
| [http_frameworks/actix](./http_frameworks/actix) | [Actix Web](https://actix.rs) 4                 | `actix-web` | `Page` as `Responder`, `App::route` |
| [http_frameworks/warp](./http_frameworks/warp) | [Warp](https://github.com/seanmonstar/warp) 0.4 | `warp` | `Page` as `Reply`, `Filter::map` |

```bash
# example: run the Axum variant
cargo run --manifest-path examples/http_frameworks/axum/Cargo.toml
```

---

## Web framework examples

These examples show how to embed a client-side JS component (React, Vue, Lit, Svelte, Angular) inside a Tidos page using the Custom Elements API.

| Example | Description | Concepts |
|---|---|---|
| [web_frameworks/react](./web_frameworks/react) | React component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/vue](./web_frameworks/vue) | Vue component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/lit](./web_frameworks/lit) | LitElement component embedded in a Tidos page — no compile step beyond bundling. | LitElement, Shadow DOM, Custom Elements, Vite |
| [web_frameworks/svelte](./web_frameworks/svelte) | Svelte component wrapped as a Custom Element and embedded in a Tidos page. | Custom Elements, `tidos::head!`, props as HTML attributes |
| [web_frameworks/angular](./web_frameworks/angular) | Angular standalone component registered as a Custom Element via `@angular/elements`. | `@angular/elements`, signals, AOT, Custom Elements, Vite |

All `web_frameworks/` examples follow the same pattern:

1. A frontend component is compiled to a self-contained JS module in `dist/` using **Vite**.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` into `<head>` and renders the HTML tag.
3. The frontend framework handles reactivity client-side — Tidos only renders the tag.

For these examples, install Node dependencies and build the JS before running Cargo:

```bash
npm install
npm run build
cargo run
```

---

## Internationalization

| Example | Description | Concepts |
|---|---|---|
| [internationalization](./internationalization) | Locale-aware page served by Rocket with Fluent `.ftl` translation files. | `i18n!`, `Lang`, `enable_i18n!`, `Tidos.toml`, Fluent pluralization |

See the [internationalization README](./internationalization/README.md) for a full walkthrough.
