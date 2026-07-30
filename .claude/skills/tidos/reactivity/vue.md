# Reactivity: Vue + Tidos

Tidos renders on the server and has no client-side state of its own. For
interactivity, wrap a Vue 3 SFC with `defineCustomElement` to produce a
native Custom Element and wrap it in a Rust struct with `#[native_element]`
(see `syntax.md`). This file is self-contained: follow it top to bottom to
scaffold a working Vue + Tidos integration from scratch.

## How it works

```
HelloWorld.vue  ──►  HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                     (defineCustomElement                           │
                      + customElements.define)           loaded by hello_world.rs
                                                                    │
                                                         <hello-world> in the page
```

1. Every `.js` entry file under `src/` is bundled into a self-contained
   module in `dist/`. Each entry imports a Vue SFC and registers it as a
   custom element.
2. The Tidos `Component` wraps the custom element: it injects the
   `<script>` tag into `<head>` and renders the HTML tag.
3. The Vue component handles its own reactivity fully client-side, the
   server only renders the tag.

## Project structure

```
src/
├── main.rs
├── components/
│   ├── mod.rs
│   ├── HelloWorld.vue          # Vue SFC — template and logic only, no <style> block
│   ├── HelloWorld.css          # Component styles (imported as raw string for Shadow DOM)
│   ├── HelloWorld.js           # Entry point — registers the custom element
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

## 1. The Vue component

A standard Vue SFC (`.vue`) with template and logic only, no `<style>`
block. Styles live in a separate `.css` file so they can be imported as a
raw string and passed to `defineCustomElement`:

```vue
<!-- src/components/HelloWorld.vue -->
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

Wraps the SFC with `defineCustomElement`, injects the styles, and registers
the custom element:

```js
// src/components/HelloWorld.js
import { defineCustomElement } from 'vue';
import HelloWorldVue from './HelloWorld.vue';
import styles from './HelloWorld.css?raw';

const HelloWorld = defineCustomElement({
    ...HelloWorldVue,
    styles: [styles],
});

customElements.define('hello-world', HelloWorld);
```

The `?raw` query tells Vite to import the CSS file as a plain string
instead of extracting it. Passing it via `styles` injects it into the
component's Shadow DOM, required because Shadow DOM is isolated from the
main document's stylesheets. Vite bundles `HelloWorld.js` as the entry
point, producing a single `dist/HelloWorld.js` with no separate CSS file.

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
name = "vue-tidos"
version = "0.1.0"
edition = "2021"

[dependencies]
tidos = { version = "0.8.0-rc.2", features = ["rocket"] }
rocket = "0.5"
```

## 5. `package.json`

```json
{
  "name": "vue-tidos",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "type": "module",
  "devDependencies": {
    "@vitejs/plugin-vue": "^6.0.0",
    "vite": "^8.0.0"
  },
  "dependencies": {
    "vue": "^3.5.0"
  }
}
```

## 6. `vite.config.js`

Discovers every `.js` file under `src/` and bundles each one into `dist/`.
The custom `tidosVueHMR` plugin rebuilds automatically on save so the
Rocket server always serves fresh JS during development:

```js
import { build, createLogger, defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
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

function tidosVueHMR() {
    let shouldDebounce = false;
    const hmrBuild = async () => {
        shouldDebounce = true;
        await build({ logLevel: 'silent' });
    };

    return {
        name: 'tidos-vue-hmr',
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
        vue(),
        tidosVueHMR(),
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
            <h1>{"Vue + Tidos"}</h1>
            <p>{"A Vue component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Vue + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
```

If the component takes props, add fields to the struct, they forward as
kebab-case attributes and Vue maps them automatically to camelCase props:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}
// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Key gotchas

- The `.vue` SFC must **not** contain a `<style>` block, styles live in the
  sibling `.css` file so they can be imported with `?raw` and passed to
  `defineCustomElement({ styles: [...] })`. A `<style>` block in the SFC
  itself is scoped by Vue's normal mechanism, not the Shadow DOM, and won't
  apply correctly inside a custom element.
- `defineCustomElement` needs the whole compiled SFC object spread in
  (`...HelloWorldVue`), not just the component's default export used
  directly.
- `customElements.define('hello-world', HelloWorld)` must run at module
  load time in the entry file, that's what actually registers the tag.

## Running it

```bash
npm install
npm run build      # produces dist/HelloWorld.js
cargo run           # serves http://localhost:8000
```

During development, run both in parallel:

```bash
# Terminal 1
npm run dev          # rebuilds dist/ on every .vue, .js, or .css save

# Terminal 2
cargo run             # restart manually on .rs changes
```
