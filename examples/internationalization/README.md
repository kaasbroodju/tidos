# Internationalization Example

This example demonstrates i18n support in Tidos using [Fluent](https://projectfluent.org/) (`.ftl`) translation files.

## How it works

### 1. Configure `Tidos.toml`

```toml
[default]
resource_location = "translations"
default_locale = "en-US"
resources = ["common.ftl"]
```

Place your `.ftl` files under the `resource_location` directory, organized by locale:

```
translations/
├── en-US/
│   └── common.ftl
└── nl-NL/
    └── common.ftl
```

### 2. Enable i18n in `main.rs`

Call `enable_i18n!()` at the top level of your `main.rs` to initialize the translation system:

```rust
enable_i18n!();
```

### 3. Add `lang: Lang` to your Rocket route

The URL is expected to contain the locale as a path segment (e.g. `/en-US` or `/nl-NL`). Tidos extracts it automatically when you add `lang: Lang` as a route parameter. The `page!` macro picks this up and passes the correct locale to all components.

```rust
#[get("/<lang>")]
pub fn index(lang: Lang) -> Page {
    page! {
        <main>
            <Greeting />
        </main>
    }
}
```

### 4. Translate strings with `i18n!`

Use the `i18n!` macro in your components with a Fluent message key:

```rust
i18n!("greeting")
```

You can pass optional key-value parameters for Fluent variables:

```rust
i18n!("shared-photos", "userName", "Anne", "userGender", "female", "photoCount", 3)
```

Parameters are passed as alternating key-value pairs after the message key.

### Fluent file example

```ftl
greeting = hello

shared-photos =
    {$userName} {$photoCount ->
        [one] added a new photo
       *[other] added {$photoCount} new photos
    } to {$userGender ->
        [male] his stream
        [female] her stream
       *[other] their stream
    }.
```

Fluent supports pluralization, gender variants, and other selectors out of the box.