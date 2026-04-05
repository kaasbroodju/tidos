# LitElement + Tidos example

This example shows how to integrate [LitElement](https://lit.dev/) components into a Tidos project. The core idea: LitElement extends the browser's native Custom Elements API, so no compilation step is needed beyond bundling — Vite bundles the JS, and Tidos wraps the custom element tag on the server side.

## How it works

```
HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                                             │
                                  loaded by hello_world.rs
                                             │
                                  <hello-world> in the page
```

1. Every `.js` file under `src/` that defines a custom element is bundled into a self-contained module in `dist/`.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` tag into `<head>` and renders the HTML tag.
3. LitElement handles its own reactivity fully client-side via the Shadow DOM — the server only renders the tag.

## Project structure

```
src/
├── components/
│   ├── HelloWorld.js           # LitElement component (native custom element)
│   └── hello_world.rs          # Tidos wrapper component
└── pages/
    └── index.rs                # Route that uses the Tidos wrapper
```

## LitElement component

A LitElement component extends `LitElement`, declares reactive properties, and registers itself as a custom element:

```js
import { LitElement, html, css } from 'lit';

class HelloWorld extends LitElement {
    static properties = {
        name: { type: String },
    };

    static styles = css`/* Shadow DOM styles */`;

    render() {
        return html`...`;
    }
}

customElements.define('hello-world', HelloWorld);
```

LitElement uses the Shadow DOM for style encapsulation, so component styles are isolated from the rest of the page.

## Tidos wrapper

The Rust wrapper in `hello_world.rs` uses `#[native_element]` to automatically generate the `Component` implementation:

```rust
use tidos::native_element;

#[native_element]
pub struct HelloWorld;
```

The macro injects `<script type="module" src="/dist/HelloWorld.js">` into `<head>` and renders the `<hello-world>` tag. If the component accepts props, add fields to the struct — they are forwarded as kebab-case HTML attributes. LitElement maps these automatically to camelCase properties when `attribute` is set in the property descriptor:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}

// Rendered as: <greeter initial-name="Alice"></greeter>
```

## Build tooling

### `vite.config.js`

Vite bundles every `.js` file under `src/` into `dist/`. No framework plugin is needed because LitElement components are plain JavaScript modules — Vite handles the `lit` dependency via its standard module resolution.

```
npm run build  →  dist/HelloWorld.js
```

During development, `npm run dev` starts Vite with HMR. The custom `tidosLitHMR` plugin watches for JS changes and triggers a new production build automatically.

## Running the example

### 1. Install Node dependencies

```bash
npm install
```

### 2. Build the LitElement components

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

Run both in parallel:

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```
