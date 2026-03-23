# Vue + Tidos example

This example shows how to integrate Vue 3 components into a Tidos project. The core idea: wrap a Vue SFC with `defineCustomElement` to produce a native [Custom Element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements), then wrap it in a Rust Tidos component so props are checked at compile time.

## How it works

```
HelloWorld.vue  ──►  HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                     (defineCustomElement                           │
                      + customElements.define)           loaded by hello_world.rs
                                                                    │
                                                         <hello-world> in the page
```

1. Every `.js` entry file under `src/` is bundled into a self-contained module in `dist/`. Each entry imports a Vue SFC and registers it as a custom element.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` tag into `<head>` and renders the HTML tag.
3. The Vue component handles its own reactivity fully client-side — the server only renders the tag.

## Project structure

```
src/
├── components/
│   ├── HelloWorld.vue          # Vue SFC — template and logic only, no <style> block
│   ├── HelloWorld.css          # Component styles (imported as raw string for Shadow DOM)
│   ├── HelloWorld.js           # Entry point — registers the custom element
│   ├── hello_world.rs          # Tidos wrapper component
│   └── hello_world.css         # Scoped styles for the wrapper div
└── pages/
    └── index.rs                # Route that uses the Tidos wrapper
```

## Vue component

The component is a standard Vue SFC (`.vue`) with template and logic only — no `<style>` block. Styles live in a separate `.css` file so they can be imported as a raw string and passed to `defineCustomElement`:

```vue
<template>
  <div>
    <input v-model="name" placeholder="Enter your name" />
    <p v-if="name">Hello {{ name }}!</p>
  </div>
</template>

<script setup>
import { ref } from 'vue';
const name = ref('');
</script>
```

The `HelloWorld.js` entry file wraps the SFC with `defineCustomElement`, injects the styles, and registers the custom element:

```js
import { defineCustomElement } from 'vue';
import HelloWorldVue from './HelloWorld.vue';
import styles from './HelloWorld.css?raw';

const HelloWorld = defineCustomElement({
    ...HelloWorldVue,
    styles: [styles],
});

customElements.define('hello-world', HelloWorld);
```

The `?raw` query tells Vite to import the CSS file as a plain string instead of extracting it. Passing it via `styles` injects it into the component's Shadow DOM — this is required because Shadow DOM is isolated from the main document's stylesheets.

Vite bundles `HelloWorld.js` as the entry point, producing a single `dist/HelloWorld.js` with no separate CSS file.

## Tidos wrapper

The Rust wrapper in `hello_world.rs` injects the compiled JS into the page `<head>` and renders the custom element tag:

```rust
pub struct HelloWorld;

impl Component for HelloWorld {
    fn to_render(&self, page: &mut Page) -> String {
        tidos::head!(
            <script r#type="module" src="/dist/HelloWorld.js"></script>
        );

        view!(
            <div class={scoped_css!("./hello_world.css")}>
                <hello-world></hello-world>
            </div>
        )
    }
}
```

If the component accepts props, add fields to the struct and pass them as attributes on the custom element tag. Vue maps kebab-case HTML attributes to camelCase props automatically:

```rust
pub struct Greeter {
    pub initial_name: String,
}

// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Build tooling

### `vite.config.js`

Vite is the build system used by `npm run build`. The config automatically discovers every `.js` file under `src/` and bundles each one into `dist/`:

```
npm run build  →  dist/HelloWorld.js
```

During development, `npm run dev` starts Vite with HMR. The custom `tidosVueHMR` plugin watches for changes and triggers a new production build automatically, so the Rocket server always serves the latest JS without a manual rebuild.

## Running the example

### 1. Install Node dependencies

```bash
npm install
```

### 2. Build the Vue components

```bash
npm run build
```

This produces `dist/HelloWorld.js`.

### 3. Run the Rocket server

```bash
cargo run
```

Open [http://localhost:8000](http://localhost:8000).

### Development workflow

Run both in parallel. Vite rebuilds the JS automatically on every `.vue`, `.js`, or `.css` save. For the Rust side, restart `cargo run` manually when you change `.rs` files.

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```
