# Svelte + Tidos example

This example shows how to integrate Svelte components into a Tidos project. The core idea: compile Svelte components to native [Custom Elements](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements), then wrap each one in a Rust Tidos component so props are checked at compile time.

## How it works

```
HelloWorld.svelte  ──►  vite build  ──►  dist/HelloWorld.js
                                                │
                                     loaded by hello_world.rs
                                                │
                                     <hello-world> in the page
```

1. Every `.svelte` file under `src/` is compiled to a self-contained Custom Element JS module in `dist/`.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` tag into `<head>` and renders the HTML tag.
3. The Svelte component handles its own reactivity fully client-side — the server only renders the tag.

## Project structure

```
src/
├── components/
│   ├── HelloWorld.svelte       # Svelte component (compiled to a custom element)
│   ├── hello_world.rs          # Tidos wrapper component
│   └── hello_world.css         # Scoped styles for the wrapper
└── pages/
    └── index.rs                # Route that uses the Tidos wrapper
```

## Svelte component

Each Svelte component must declare itself as a custom element via `<svelte:options>`:

```svelte
<svelte:options customElement="hello-world"></svelte:options>
```

This tells the Svelte compiler to output a Custom Element class instead of a regular Svelte component. The tag name (`hello-world`) is what you use in your HTML.

## Tidos wrapper

The Rust wrapper in `hello_world.rs` does two things:

1. Injects the compiled JS into the page `<head>` using `tidos::head!`.
2. Renders the custom element tag.

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

If the component accepts props, add fields to the struct and pass them as attributes on the custom element tag. Because this is Rust, any mismatch between the struct fields and the rendered attributes is caught at **compile time**:

```rust
pub struct Greeter {
    pub initial_name: String,
}

// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Build tooling

### `vite.config.js`

Vite is the build system used by `npm run build`. The config automatically discovers every `.svelte` file under `src/` and compiles each one as a custom element into `dist/`:

```
npm run build  →  dist/HelloWorld.js
```

During development, `npm run dev` starts Vite with HMR. The custom `tidosSvelteHMR` plugin watches for `.svelte` changes and triggers a new production build automatically, so the Rocket server always serves the latest JS without a manual rebuild.

## Running the example

### 1. Install Node dependencies

```bash
npm install
```

### 2. Build the Svelte components

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

Run both in parallel. Vite rebuilds the JS automatically on every `.svelte` save. For the Rust side, restart `cargo run` manually when you change `.rs` files.

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```