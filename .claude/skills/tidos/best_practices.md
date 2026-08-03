# Tidos Best Practices

Opinionated dos and don'ts on top of the core syntax (see `syntax.md`). Read
this after you have working code, to sanity-check it, or when a compile error
matches one of the pitfalls below.

## Common Pitfalls

1. **Raw text between tags panics**: All text must be in `{}`. Use `{"Hello"}`,
   `{"Hello {}", name}`, or `{expr}`. Bare text like `<p>Hello</p>` causes a
   compile-time panic.

2. **Borrow vs. owned**: `view!` often needs `&self.field` (a reference) to
   avoid moving out of `self`. If you see a "value moved" error, add `&`.

3. **Closing tags**: every `{#for}`, `{#if}`, `{#match}` block **must** have a
   matching close tag (`{/for}`, `{/if}`, `{/match}`). Missing them causes
   compile errors.

4. **`{:else}` placement**: `{:else}` must come *after* all `{:else if}`
   branches and *before* `{/if}`.

5. **Prop types**: props passed to components with `<Component prop={expr} />`
   must match the struct field types exactly. Use `.into()`, `.clone()`, or
   `String::from(…)` as needed.

6. **Slots render with `@slot`, not `@html`**: a component receiving slot
   content must invoke it with `@slot{self.field}` (named) or `@slot{self.0}`
   (unnamed), this calls the boxed closure. `{self.field}` or
   `@html{self.field}` won't work; `Slot<'a>` is a closure, not a `String`.

7. **Slot field type**: the field that receives slot content must be
   `Slot<'a>` (`Box<dyn Fn(&mut Page) + 'a>`). Use a named field when the
   component has multiple slots or a slot alongside other props; use a tuple
   struct field `0` only for a single slot with no other props.

8. **Mixing slotted and un-slotted children**: if a component tag's first
   child is a `{#slot:name}` block, the whole tag is treated as named-slot
   mode, any other top-level children that aren't `{#slot:...}` blocks are
   silently dropped, not rendered. Don't mix the two forms in one tag.

9. **One `Content` item per `{#if}`/`{:else}`/`{:case}` branch**: the macro
   concatenates a branch's children without an explicit `+`, so a branch with
   more than one top-level child fails to compile. Collapse multiple children
   into a single tag or a single `{format!(...)}` expression.

10. **`Component` must be in scope** wherever `page!`/`view!` render a
    component tag (`<NavBar />`), since the macro calls `.to_render(page)` on
    it directly.

11. **Numeric types**: `{self.score}` where `score` is `u32`/`usize` fails
    because sanitize requires `AsRef<str>`. Use `{self.score.to_string()}`.

12. **`@class` is `class`-only, native tags only**: `@` is only valid on the
    `class` attribute (`@data-foo="..."` is a compile error), and only on
    native HTML tags, not component tags (there's no HTML string to merge a
    class into). The path must be a bare string literal, not an expression.

## Attribute Formatting

When a tag (native HTML or custom component) has **3 or more attributes**,
put one attribute per line with consistent indentation, Prettier-style,
instead of a long single-line tag. With 1-2 attributes, keep them inline.

Order attributes from most static to most dynamic, ending with `..` if
present:

1. Bare attribute, no value (`disabled`)
2. String literal value (`class="wrapper"`)
3. `@class` scoped-CSS shorthand (`@class="mod.css"` / `@class={"mod.css", "data-tidos"}`)
4. Braced expression value (`value={expr}`)
5. Bare toggle (`:toggle`)
6. Toggle with an expression (`toggle={expr}`)
7. `..` default-fill, always last, only on custom components

```rust
// 1-2 attributes: inline
<input placeholder="Name" />

// 3+ attributes: one per line, ordered static → dynamic → `..`
<UserBadge
    disabled
    class="wrapper"
    @class={"mod.css", "data-tidos"}
    value={expr}
    :toggle
    toggle={expr}
    ..
/>
```

## Styling Conventions

### One `scoped_css!` per component

Apply the scoped class **once, on the root element** of the component. Style
all children via CSS descendant selectors inside that one CSS file, the same
approach as Svelte's `<style>` block. Calling `scoped_css!` once per element
instead of once per component works, but scatters styling across multiple
tiny CSS files for no benefit and makes the component harder to restyle as a
whole.

```rust
// correct: one scoped_css! on the root, children styled in CSS
view! {
    <nav @class="./nav_bar.css">
        <span>{&self.title}</span>
        <ul>…</ul>
    </nav>
}

// avoid: multiple scoped_css! calls, one per element
view! {
    <nav @class="./nav.css">
        <span @class="./title.css">{&self.title}</span>
    </nav>
}
```

Prefer the `@class` shorthand (`@class="./nav_bar.css"`) over
`class={scoped_css!("./nav_bar.css")}` on native HTML tags, see "Scoped CSS"
in `syntax.md`. Fall back to calling `scoped_css!` directly only when you
need the class name itself as a value (passed as a prop, combined with other
logic) or on a component tag, where `@class` isn't available.

### Styling variants with `data-*` attributes

When child elements of the same tag differ only in style (tier badges, status
indicators), use `data-*` attributes instead of extra CSS files or extra
`scoped_css!` calls:

```rust
{#if self.score >= 9000}
    <span data-tier="legendary">{"LEGENDARY"}</span>
{:else if self.score >= 5000}
    <span data-tier="expert">{"EXPERT"}</span>
{:else}
    <span data-tier="newbie">{"NEWBIE"}</span>
{/if}
```

```css
& span[data-tier="legendary"] { color: #ffd700; }
& span[data-tier="expert"]    { color: #c0c0c0; }
& span[data-tier="newbie"]    { color: #555; }
```

Hyphenated attribute names like `data-tier` and `data-status` are fully
supported by the `view!` macro parser.
