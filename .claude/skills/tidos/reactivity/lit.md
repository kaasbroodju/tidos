# Reactivity: Lit + Tidos

Tidos renders on the server and has no client-side state of its own. For
interactivity, write a [LitElement](https://lit.dev/) component (a thin
layer over the native Custom Elements API) and wrap it in a Rust struct with
`#[native_element]` (see `syntax.md`). This file is self-contained: follow
it top to bottom to scaffold a working Lit + Tidos integration from
scratch.

## How it works

```
HelloWorld.js  ──►  vite build  ──►  dist/HelloWorld.js
                                             │
                                  loaded by hello_world.rs
                                             │
                                  <hello-world> in the page
```

1. Every `.js` file under `src/` that defines a custom element is bundled
   into a self-contained module in `dist/`.
2. The Tidos `Component` wraps the custom element: it injects the
   `<script>` tag into `<head>` and renders the HTML tag.
3. LitElement handles its own reactivity fully client-side via the Shadow
   DOM, the server only renders the tag.

## Project structure

```
src/
├── main.rs
├── components/
│   ├── mod.rs
│   ├── HelloWorld.js           # LitElement component (native custom element)
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

## 1. The LitElement component

A LitElement component extends `LitElement`, declares reactive properties,
and registers itself as a custom element in the same file. No separate
entry file needed, unlike Vue/React below:

```js
// src/components/HelloWorld.js
import { LitElement, html, css } from 'lit';

class HelloWorld extends LitElement {
    static properties = {
        name: { type: String },
    };

    static styles = css`
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
    `;

    constructor() {
        super();
        this.name = '';
    }

    render() {
        return html`
            <div>
                <input
                    .value=${this.name}
                    placeholder="Enter your name"
                    @input=${(e) => (this.name = e.target.value)}
                />
                ${this.name ? html`<p>Hello ${this.name}!</p>` : ''}
            </div>
        `;
    }
}

customElements.define('hello-world', HelloWorld);
```

LitElement uses the Shadow DOM for style encapsulation via `static styles`,
so component styles are isolated from the rest of the page automatically.

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
name = "lit-tidos"
version = "0.1.0"
edition = "2021"

[dependencies]
tidos = { version = "0.8.0-rc.3", features = ["rocket"] }
rocket = "0.5"
```

## 4. `package.json`

```json
{
  "name": "lit-tidos",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "type": "module",
  "devDependencies": {
    "vite": "^8.0.0"
  },
  "dependencies": {
    "lit": "^3.0.0"
  }
}
```

## 5. `vite.config.js`

Bundles every `.js` file under `src/` into `dist/`. No framework plugin is
needed, LitElement components are plain JS modules; Vite resolves the
`lit` dependency via standard module resolution. The custom `tidosLitHMR`
plugin rebuilds automatically on save:

```js
import { build, createLogger, defineConfig } from 'vite';
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

function tidosLitHMR() {
    let shouldDebounce = false;
    const hmrBuild = async () => {
        shouldDebounce = true;
        await build({ logLevel: 'silent' });
    };

    return {
        name: 'tidos-lit-hmr',
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
    plugins: [tidosLitHMR()],
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
            <h1>{"Lit + Tidos"}</h1>
            <p>{"A LitElement component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Lit + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
```

If the component takes props, add fields to the struct, LitElement maps
kebab-case attributes to camelCase properties automatically when
`attribute` is set in the property descriptor:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}
// Rendered as: <greeter initial-name="Alice"></greeter>
```

## Key gotchas

- `customElements.define('hello-world', HelloWorld)` must run at module
  load time (top level of the file), it's what actually registers the tag;
  forgetting it leaves `<hello-world>` as an unrecognized element.
- Property names declared in `static properties` map to attributes
  automatically only when Lit's default reflection applies, complex
  (object/array) props need an explicit `converter`.
- The Shadow DOM created by LitElement isolates `static styles` from the
  host page, global page CSS (`static/main.css`) never leaks in or out.

## Running it

```bash
npm install
npm run build      # produces dist/HelloWorld.js
cargo run           # serves http://localhost:8000
```

During development, run both in parallel:

```bash
# Terminal 1
npm run dev

# Terminal 2
cargo run
```
