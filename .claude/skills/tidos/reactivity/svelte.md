# Reactivity: Svelte + Tidos

Tidos renders on the server and has no client-side state of its own. For
interactivity, compile a Svelte component to a native Custom Element and
wrap it in a Rust struct with `#[native_element]` (see `syntax.md`). This
file is self-contained: follow it top to bottom to scaffold a working
Svelte + Tidos integration from scratch.

## How it works

```
HelloWorld.svelte  ──►  vite build  ──►  dist/HelloWorld.js
                                                │
                                     loaded by hello_world.rs
                                                │
                                     <hello-world> in the page
```

1. Every `.svelte` file under `src/` is compiled to a self-contained Custom
   Element JS module in `dist/`.
2. The Tidos `Component` wraps the custom element: it injects the
   `<script>` tag into `<head>` and renders the HTML tag.
3. The Svelte component handles its own reactivity fully client-side, the
   server only renders the tag.

## Project structure

```
src/
├── main.rs
├── components/
│   ├── mod.rs
│   ├── HelloWorld.svelte       # Svelte component, compiled to a custom element
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

## 1. The Svelte component

Declare the component as a custom element via `<svelte:options>`. The tag
name given here (`hello-world`) is what you use in HTML:

```svelte
<!-- src/components/HelloWorld.svelte -->
<svelte:options customElement="hello-world"></svelte:options>

<script>
	let name = $state('');
</script>

<div>
	<input bind:value={name} placeholder="Enter your name" />
	{#if name}
		<p>Hello {name}!</p>
	{/if}
</div>

<style>
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
</style>
```

No separate custom-element entry file is needed, `compilerOptions.customElement: true` (below) plus `<svelte:options customElement>` is enough.

## 2. The Tidos wrapper

```rust
// src/components/hello_world.rs
use tidos::native_element;

#[native_element]
pub struct HelloWorld;
```

## 3. `Cargo.toml`

```toml
[package]
name = "svelte-tidos"
version = "0.1.0"
edition = "2021"

[dependencies]
tidos = { version = "0.8.0-rc.2", features = ["rocket"] }
rocket = "0.5"
```

## 4. `package.json`

```json
{
  "name": "svelte-tidos",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "type": "module",
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^7.0.0",
    "svelte": "^5.54.1",
    "vite": "^8.0.0"
  }
}
```

## 5. `vite.config.js`

Discovers every `.svelte` file under `src/` and compiles it as a custom
element into `dist/`. The custom `tidosSvelteHMR` plugin rebuilds
automatically on save so the Rocket server always serves fresh JS during
development:

```js
import { build, createLogger, defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { extname, join, basename, resolve } from 'path';
import { readdirSync, statSync } from 'fs';

let logger = createLogger('info', { prefix: '[tidos]' });

function getEntries(dir, extension = '.svelte') {
    const entries = {};

    function walk(currentDir) {
        const files = readdirSync(currentDir);

        files.forEach((file) => {
            const fullPath = join(currentDir, file);
            const stat = statSync(fullPath);

            if (stat.isDirectory()) {
                walk(fullPath);
            } else if (extname(file) === extension) {
                const name = basename(file, extension);
                entries[name] = resolve(__dirname, fullPath);
            }
        });
    }

    walk(dir);
    return entries;
}

const entries = getEntries('src');

function tidosSvelteHMR() {
    let shouldDebounce = false;
    const hmrBuild = async () => {
        shouldDebounce = true;
        await build({ logLevel: 'silent' });
    };

    return {
        name: 'tidos-svelte-hmr',
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
        svelte({
            compilerOptions: {
                customElement: true,
            },
        }),
        tidosSvelteHMR(),
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

## 6. `main.rs` — serving the compiled JS

This Rocket boilerplate is identical across every framework in this
directory: it serves `dist/` (the compiled custom elements, long cache
lifetime) and `static/` (plain assets) alongside the app's routes.

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

## 7. Using it in a route

```rust
// src/pages/index.rs
use tidos::Component;
use tidos::{Page, page};
use crate::components::hello_world::HelloWorld;

#[get("/")]
pub fn index() -> Page {
    let mut page = page!(
        <main>
            <h1>{"Svelte + Tidos"}</h1>
            <p>{"A Svelte component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Svelte + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
```

If the component takes props, add fields to the struct, they forward as
kebab-case attributes and are checked at compile time:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}
// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Key gotchas

- `compilerOptions.customElement: true` in `vite.config.js` **and**
  `<svelte:options customElement="...">` in the component are both
  required, missing either one leaves you with a regular Svelte component,
  not a Custom Element.
- The tag name comes from `<svelte:options customElement="...">`, not from
  the file name or the `#[native_element]` struct name, keep them in sync
  by convention (`HelloWorld.svelte` → `hello-world` → `pub struct
  HelloWorld`).
- `$state`/`$props` (Svelte 5 runes) work fine inside a custom element,
  reactivity is entirely client-side; the server never re-renders it.

## Running it

```bash
npm install
npm run build      # produces dist/HelloWorld.js
cargo run           # serves http://localhost:8000
```

During development, run both in parallel:

```bash
# Terminal 1
npm run dev          # rebuilds dist/ on every .svelte save

# Terminal 2
cargo run             # restart manually on .rs changes
```
