# Reactivity: React + Tidos

Tidos renders on the server and has no client-side state of its own. For
interactivity, mount a React component inside a manually defined Custom
Element (using a Shadow DOM) and wrap it in a Rust struct with
`#[native_element]` (see `syntax.md`). This file is self-contained: follow
it top to bottom to scaffold a working React + Tidos integration from
scratch.

## How it works

```
HelloWorld.jsx  ──►  HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                     (HTMLElement class                             │
                      + ReactDOM.createRoot                loaded by hello_world.rs
                      + customElements.define)                      │
                                                          <hello-world> in the page
```

1. Every `.js` entry file under `src/` is bundled into a self-contained
   module in `dist/`. Each entry defines an `HTMLElement` subclass that
   mounts a React component into its Shadow DOM.
2. The Tidos `Component` wraps the custom element: it injects the
   `<script>` tag into `<head>` and renders the HTML tag.
3. React handles its own reactivity fully client-side, the server only
   renders the tag.

## Project structure

```
src/
├── main.rs
├── components/
│   ├── mod.rs
│   ├── HelloWorld.jsx          # React component
│   ├── HelloWorld.css          # Component styles (injected into Shadow DOM)
│   ├── HelloWorld.js           # Entry point — defines the custom element and mounts React
│   └── hello_world.rs          # Tidos wrapper component
├── pages/
│   ├── mod.rs
│   └── index.rs                # Route that uses the Tidos wrapper
└── static/
    └── main.css
package.json
vite.config.js
Cargo.toml
```

## 1. The React component

A standard React function component using hooks:

```jsx
// src/components/HelloWorld.jsx
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

```css
/* src/components/HelloWorld.css */
:host {
    display: block;
}

div {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

input {
    box-sizing: border-box;
    padding: 0.5rem 0.75rem;
    border: 1px solid #ccc;
    border-radius: 6px;
    font-size: 1rem;
    font-family: inherit;
    width: 100%;
    outline: none;
    transition: border-color 0.15s;
}

input:focus {
    border-color: #555;
}

p {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: #1a1a1a;
}
```

## 2. Custom element entry file

React has no built-in custom element compilation step. Instead, the entry
file manually defines an `HTMLElement` subclass, attaches a Shadow DOM,
injects styles, and mounts the React component into it:

```js
// src/components/HelloWorld.js
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
- CSS is imported with `?raw` so Vite keeps it as a plain string rather than
  extracting it to a separate file. It's then injected via a `<style>`
  element inside the shadow root.

## 3. The Tidos wrapper

```rust
// src/components/hello_world.rs
use tidos::native_element;

#[native_element]
pub struct HelloWorld;
```

## 4. `Cargo.toml`

```toml
[package]
name = "react-tidos"
version = "0.1.0"
edition = "2021"

[dependencies]
tidos = { version = "0.8.0-rc.2", features = ["rocket"] }
rocket = "0.5"
```

## 5. `package.json`

```json
{
  "name": "react-tidos",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "type": "module",
  "devDependencies": {
    "@vitejs/plugin-react": "^6.0.0",
    "vite": "^8.0.0"
  },
  "dependencies": {
    "react": "^19.0.0",
    "react-dom": "^19.0.0"
  }
}
```

## 6. `vite.config.js`

Discovers every `.js` file under `src/` and bundles each into `dist/`. The
`@vitejs/plugin-react` plugin handles JSX transformation. The custom
`tidosReactHMR` plugin rebuilds automatically on save:

```js
import { build, createLogger, defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { extname, join, basename, resolve } from 'path';
import { readdirSync, statSync } from 'fs';

let logger = createLogger('info', { prefix: '[tidos]' });

function getEntries(dir, extension = '.js') {
    const entries = {};

    function walk(currentDir) {
        const files = readdirSync(currentDir);

        files.forEach((file) => {
            const fullPath = join(currentDir, file);
            const stat = statSync(fullPath);

            if (stat.isDirectory()) {
                walk(fullPath);
            } else if (extname(file) === extension && !file.endsWith('.config.js')) {
                const name = basename(file, extension);
                entries[name] = resolve(__dirname, fullPath);
            }
        });
    }

    walk(dir);
    return entries;
}

const entries = getEntries('src');

function tidosReactHMR() {
    let shouldDebounce = false;
    const hmrBuild = async () => {
        shouldDebounce = true;
        await build({ logLevel: 'silent' });
    };

    return {
        name: 'tidos-react-hmr',
        enforce: 'pre',
        handleHotUpdate: ({ file, server }) => {
            if (!shouldDebounce) {
                logger.info(`Changes detected, building new version...`, { timestamp: true });
                hmrBuild().then(() => {
                    shouldDebounce = false;
                    logger.info(`Build completed.`, { timestamp: true });
                });
            }
            return [];
        },
    };
}

export default defineConfig({
    plugins: [
        react(),
        tidosReactHMR(),
    ],
    build: {
        rollupOptions: {
            input: entries,
            output: {
                entryFileNames: '[name].js',
                chunkFileNames: '[name].js',
                dir: 'dist',
                assetFileNames: '[name][extname]',
            },
        },
    },
});
```

## 7. `main.rs` — serving the compiled JS

Identical across every framework in this directory: serves `dist/` (long
cache lifetime) and `static/` alongside the app's routes.

```rust
mod components;
mod pages;

#[macro_use] extern crate rocket;

use std::path::PathBuf;
use rocket::fs::{FileServer, NamedFile, relative};
use rocket::http::Header;
use rocket::Request;
use rocket::response::Responder;
use crate::pages::index::index;

#[get("/dist/<file..>")]
async fn files(file: PathBuf) -> Option<CachedFile> {
    NamedFile::open(std::path::Path::new("dist/").join(file)).await.ok().map(CachedFile)
}

struct CachedFile(NamedFile);

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for CachedFile {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let mut response = self.0.respond_to(req)?;
        response.set_header(Header::new("Cache-Control", "public, max-age=180"));
        Ok(response)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, files])
        .mount("/dist", FileServer::from(relative!("dist")).rank(-2))
        .mount("/public", FileServer::from(relative!("static")).rank(-3))
}
```

## 8. Using it in a route

```rust
// src/pages/index.rs
use tidos::Component;
use tidos::{Page, page};
use crate::components::hello_world::HelloWorld;

#[get("/")]
pub fn index() -> Page {
    let mut page = page!(
        <main>
            <h1>{"React + Tidos"}</h1>
            <p>{"A React component mounted in a Shadow DOM custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"React + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
```

If the component takes props, add fields to the struct, they forward as
kebab-case attributes. Read them inside `connectedCallback` via
`this.getAttribute(...)`:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}
// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Key gotchas

- React has no native Custom Element output, unlike Svelte/Vue/Lit, you
  must write the `HTMLElement` subclass and Shadow DOM wiring by hand (the
  entry file above). Forgetting `attachShadow` means styles leak into/out
  of the component.
- Props aren't automatically forwarded into the React tree, `hello_world.rs`
  only sets HTML attributes on the tag; the entry file must read them via
  `this.getAttribute('prop-name')` inside `connectedCallback` and pass them
  into the React component manually if the example needs props.
- `createRoot(mount).render(...)` must target a plain child element inside
  the shadow root, not the custom element itself, mounting React directly
  onto `this` conflicts with the element's own lifecycle.

## Running it

```bash
npm install
npm run build      # produces dist/HelloWorld.js
cargo run           # serves http://localhost:8000
```

During development, run both in parallel:

```bash
# Terminal 1
npm run dev          # rebuilds dist/ on every .jsx, .js, or .css save

# Terminal 2
cargo run             # restart manually on .rs changes
```
