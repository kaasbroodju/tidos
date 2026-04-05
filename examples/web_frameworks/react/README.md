# React + Tidos example

This example shows how to integrate React components into a Tidos project. The core idea: mount a React component inside a manually defined [Custom Element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements) using a Shadow DOM, then wrap it in a Rust Tidos component so props are checked at compile time.

## How it works

```
HelloWorld.jsx  ──►  HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                     (HTMLElement class                             │
                      + ReactDOM.createRoot                loaded by hello_world.rs
                      + customElements.define)                      │
                                                          <hello-world> in the page
```

1. Every `.js` entry file under `src/` is bundled into a self-contained module in `dist/`. Each entry defines an `HTMLElement` subclass that mounts a React component into its Shadow DOM.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` tag into `<head>` and renders the HTML tag.
3. React handles its own reactivity fully client-side — the server only renders the tag.

## Project structure

```
src/
├── components/
│   ├── HelloWorld.jsx          # React component
│   ├── HelloWorld.css          # Component styles (injected into Shadow DOM)
│   ├── HelloWorld.js           # Entry point — defines the custom element and mounts React
│   └── hello_world.rs          # Tidos wrapper component
└── pages/
    └── index.rs                # Route that uses the Tidos wrapper
```

## React component

The component is a standard React function component using hooks:

```jsx
import { useState } from 'react';

export default function HelloWorld() {
    const [name, setName] = useState('');

    return (
        <div>
            <input
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name"
            />
            {name && <p>Hello {name}!</p>}
        </div>
    );
}
```

## Custom element entry file

React has no built-in custom element compilation step. Instead, `HelloWorld.js` manually defines an `HTMLElement` subclass, attaches a Shadow DOM, injects styles, and mounts the React component into it:

```js
import React from 'react';
import { createRoot } from 'react-dom/client';
import HelloWorldReact from './HelloWorld.jsx';
import styles from './HelloWorld.css?raw';

class HelloWorld extends HTMLElement {
    connectedCallback() {
        const shadow = this.attachShadow({ mode: 'open' });

        const style = document.createElement('style');
        style.textContent = styles;
        shadow.appendChild(style);

        const mount = document.createElement('div');
        shadow.appendChild(mount);

        createRoot(mount).render(React.createElement(HelloWorldReact));
    }
}

customElements.define('hello-world', HelloWorld);
```

- `connectedCallback` runs when the element is inserted into the DOM.
- The Shadow DOM isolates the component's styles from the rest of the page.
- CSS is imported with `?raw` so Vite keeps it as a plain string rather than extracting it to a separate file. It is then injected via a `<style>` element inside the shadow root.

## Tidos wrapper

The Rust wrapper in `hello_world.rs` uses `#[native_element]` to automatically generate the `Component` implementation:

```rust
use tidos::native_element;

#[native_element]
pub struct HelloWorld;
```

The macro injects `<script type="module" src="/dist/HelloWorld.js">` into `<head>` and renders the `<hello-world>` tag. If the component accepts props, add fields to the struct — they are forwarded as kebab-case HTML attributes. Read them inside `connectedCallback` via `this.getAttribute(...)`:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}

// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Build tooling

### `vite.config.js`

Vite discovers every `.js` file under `src/` and bundles each one into `dist/`. The `@vitejs/plugin-react` plugin handles JSX transformation.

```
npm run build  →  dist/HelloWorld.js
```

During development, `npm run dev` starts Vite with HMR. The custom `tidosReactHMR` plugin watches for changes and triggers a new production build automatically, so the Rocket server always serves the latest JS without a manual rebuild.

## Running the example

### 1. Install Node dependencies

```bash
npm install
```

### 2. Build the React components

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

Run both in parallel. Vite rebuilds the JS automatically on every `.jsx`, `.js`, or `.css` save. For the Rust side, restart `cargo run` manually when you change `.rs` files.

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```
