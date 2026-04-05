# Angular + Tidos example

This example shows how to integrate Angular components into a Tidos project. The core idea: use `@angular/elements` to convert an Angular standalone component to a native [Custom Element](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements), then wrap it in a Rust Tidos component so props are checked at compile time.

## How it works

```
HelloWorld.ts  ──►  vite build  ──►  dist/HelloWorld.js
(@angular/elements                            │
 createCustomElement                 loaded by hello_world.rs
 customElements.define)                       │
                                    <hello-world> in the page
```

1. Every `.ts` entry file under `src/` is compiled and bundled into a self-contained module in `dist/`. Each entry defines an Angular standalone component and registers it as a custom element via `@angular/elements`.
2. A Tidos `Component` wraps the custom element: it injects the `<script>` tag into `<head>` and renders the HTML tag.
3. Angular handles its own reactivity fully client-side via signals — the server only renders the tag.

## Project structure

```
src/
├── components/
│   ├── HelloWorld.ts           # Angular component + custom element registration
│   └── hello_world.rs          # Tidos wrapper component
└── pages/
    └── index.rs                # Route that uses the Tidos wrapper
```

## Angular component

The component is an Angular 21 standalone component using signals for reactivity. The component definition, styles, and custom element registration all live in one `.ts` file:

```typescript
import 'zone.js';
import { Component, signal } from '@angular/core';
import { createApplication } from '@angular/platform-browser';
import { createCustomElement } from '@angular/elements';

@Component({
    standalone: true,
    selector: 'hello-world-ng',
    template: `
        <div>
            <input [value]="name()" (input)="name.set($any($event).target.value)" placeholder="Enter your name" />
            @if (name()) {
                <p>Hello {{ name() }}!</p>
            }
        </div>
    `,
    styles: [`
        :host { display: block; }
        /* ... */
    `],
})
export class HelloWorldComponent {
    name = signal('');
}

createApplication().then((appRef) => {
    const HelloWorld = createCustomElement(HelloWorldComponent, {
        injector: appRef.injector,
    });
    customElements.define('hello-world', HelloWorld);
});
```

Key points:
- `zone.js` must be imported first — Angular requires it for change detection.
- `createApplication()` bootstraps a lightweight Angular app without a root component.
- `createCustomElement` wraps the Angular component as an `HTMLElement` subclass.
- Angular uses the Shadow DOM for style encapsulation via `styles: [...]` in the decorator.
- `selector` inside `@Component` is only used by Angular's DI system — the actual HTML tag name is defined by `customElements.define`.

## Tidos wrapper

The Rust wrapper in `hello_world.rs` uses `#[native_element]` to automatically generate the `Component` implementation:

```rust
use tidos::native_element;

#[native_element]
pub struct HelloWorld;
```

The macro injects `<script type="module" src="/dist/HelloWorld.js">` into `<head>` and renders the `<hello-world>` tag. If the component accepts props, add fields to the struct — they are forwarded as kebab-case HTML attributes. Angular maps these automatically to camelCase `@Input()` properties via `@angular/elements`:

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

Vite compiles every `.ts` file under `src/` using `@analogjs/vite-plugin-angular`, which runs the Angular compiler (including template compilation and AOT). The plugin expects a `tsconfig.app.json` in the project root.

```
npm run build  →  dist/HelloWorld.js
```

During development, `npm run dev` starts Vite with HMR. The custom `tidosAngularHMR` plugin watches for changes and triggers a new production build automatically.

## Running the example

### 1. Install Node dependencies

```bash
npm install
```

### 2. Build the Angular components

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

Run both in parallel. Vite rebuilds the JS automatically on every `.ts` save. For the Rust side, restart `cargo run` manually when you change `.rs` files.

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```
