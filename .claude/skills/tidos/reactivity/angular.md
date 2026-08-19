# Reactivity: Angular + Tidos

Tidos renders on the server and has no client-side state of its own. For
interactivity, use `@angular/elements` to convert an Angular standalone
component into a native Custom Element and wrap it in a Rust struct with
`#[native_element]` (see `syntax.md`). This file is self-contained: follow
it top to bottom to scaffold a working Angular + Tidos integration from
scratch.

## How it works

```
HelloWorld.ts  ──►  vite build  ──►  dist/HelloWorld.js
(@angular/elements                            │
 createCustomElement                 loaded by hello_world.rs
 customElements.define)                       │
                                    <hello-world> in the page
```

1. Every `.ts` entry file under `src/` is compiled and bundled into a
   self-contained module in `dist/`. Each entry defines an Angular
   standalone component and registers it as a custom element via
   `@angular/elements`.
2. The Tidos `Component` wraps the custom element: it injects the
   `<script>` tag into `<head>` and renders the HTML tag.
3. Angular handles its own reactivity fully client-side via signals, the
   server only renders the tag.

## Project structure

```
src/
├── main.rs
├── components/
│   ├── mod.rs
│   ├── HelloWorld.ts           # Angular component + custom element registration
│   └── hello_world.rs          # Tidos wrapper component
├── pages/
│   ├── mod.rs
│   └── index.rs                # Route that uses the Tidos wrapper
└── static/
    └── main.css
package.json
tsconfig.app.json
vite.config.js
Cargo.toml
```

## 1. The Angular component

Angular 21 standalone component using signals for reactivity. The component
definition, styles, and custom element registration all live in one `.ts`
file:

```typescript
// src/components/HelloWorld.ts
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
- `zone.js` must be imported first, Angular requires it for change
  detection.
- `createApplication()` bootstraps a lightweight Angular app without a root
  component.
- `createCustomElement` wraps the Angular component as an `HTMLElement`
  subclass.
- Angular uses the Shadow DOM for style encapsulation via `styles: [...]`.
- `selector` inside `@Component` is only used by Angular's DI system, the
  actual HTML tag name is set by `customElements.define`.

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
name = "angular-tidos"
version = "0.1.0"
edition = "2021"

[dependencies]
tidos = { version = "0.8.0-rc.3", features = ["rocket"] }
rocket = "0.5"
```

## 4. `package.json`

```json
{
  "name": "angular-tidos",
  "version": "1.0.0",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "type": "module",
  "devDependencies": {
    "@analogjs/vite-plugin-angular": "^2.3.1",
    "@angular/build": "^21.0.0",
    "typescript": "~6.0.2",
    "vite": "^8.0.0"
  },
  "dependencies": {
    "@angular/common": "^21.0.0",
    "@angular/core": "^21.0.0",
    "@angular/elements": "^21.0.0",
    "@angular/platform-browser": "^21.0.0",
    "rxjs": "~7.8.0",
    "zone.js": "~0.16.0"
  }
}
```

## 5. `tsconfig.app.json`

Required by `@analogjs/vite-plugin-angular` for the Angular compiler:

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "experimentalDecorators": true,
    "useDefineForClassFields": false,
    "strict": true,
    "lib": ["ES2022", "dom"],
    "skipLibCheck": true
  }
}
```

## 6. `vite.config.js`

Compiles every `.ts` file under `src/` using `@analogjs/vite-plugin-angular`
(runs the Angular compiler, including template compilation and AOT). The
custom `tidosAngularHMR` plugin rebuilds automatically on save:

```js
import { build, createLogger, defineConfig } from 'vite';
import angular from '@analogjs/vite-plugin-angular';
import { extname, join, basename, resolve } from 'path';
import { readdirSync, statSync } from 'fs';

let logger = createLogger('info', { prefix: '[tidos]' });

function getEntries(dir, extension = '.ts') {
    const entries = {};

    function walk(currentDir) {
        const files = readdirSync(currentDir);

        files.forEach((file) => {
            const fullPath = join(currentDir, file);
            const stat = statSync(fullPath);

            if (stat.isDirectory()) {
                walk(fullPath);
            } else if (extname(file) === extension && !file.includes('.config.')) {
                const name = basename(file, extension);
                entries[name] = resolve(__dirname, fullPath);
            }
        });
    }

    walk(dir);
    return entries;
}

const entries = getEntries('src');

function tidosAngularHMR() {
    let shouldDebounce = false;
    const hmrBuild = async () => {
        shouldDebounce = true;
        await build({ logLevel: 'silent' });
    };

    return {
        name: 'tidos-angular-hmr',
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
        angular(),
        tidosAngularHMR(),
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
            <h1>{"Angular + Tidos"}</h1>
            <p>{"An Angular component compiled to a native custom element, embedded in a Tidos page."}</p>
            <HelloWorld />
        </main>
    );

    tidos::head! {
        <title>{"Angular + Tidos example"}</title>
        <link rel="stylesheet" href="/public/main.css" />
    }

    page
}
```

If the component takes props, add fields to the struct, `@angular/elements`
maps kebab-case attributes to camelCase `@Input()` properties
automatically:

```rust
#[native_element]
pub struct Greeter {
    pub initial_name: String,
}
// Rendered as: <greeter initial-name="Alice"></greeter>
// Forgetting a required field → compile error
```

## Key gotchas

- `import 'zone.js'` must be the very first import in the entry file,
  Angular's change detection depends on it being loaded before
  `@angular/core`.
- `createApplication()` (not `bootstrapApplication`) is what lets you run an
  Angular component as a custom element without a root component or router.
- `@Component({ selector: ... })` is not the tag name, only
  `customElements.define(...)` determines the actual HTML tag. Mixing these
  up is the most common source of confusion.
- Requires Node's decorator support enabled via `experimentalDecorators` in
  `tsconfig.app.json`, missing it fails the Angular compiler step in Vite.

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
