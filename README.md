# Yew

Yew is a modern Rust framework inspired by Elm and ReactJS.

## Cutting Edge technologies

### Rust to WASM compilation

This framework designed to be compiled into modern browsers' runtimes: WASM, Asm.js, emscripten.

### Clean MVC approach inspired by Elm

```rust
struct Model { }

enum Msg {
  DoIt,
}

fn update(model: &mut Model, msg: Msg) {
  match *model {
      Msg::DoIt => {
         // Update your model on events
      }
  }
}

fn view(model: &Model) -> html::Html<Msg> {
    html! {
        // Render your model here
        <button onclick=|_| Msg::DoIt,></div>
    }
}
```

### VirtualDOM

Yew framework uses own virtual-dom representation.

### JSX-like templates with `html!` macro

Put pure Rust code into html tags.

```rust
html! {
    <section class="todoapp",>
        <header class="header",>
            <h1>{ "todos" }</h1>
            { view_input(&model) }
        </header>
        <section class="main",>
            <input class="toggle-all",
                   type="checkbox",
                   checked=model.is_all_completed(),
                   onclick=|_| Msg::ToggleAll, />
            { view_entries(&model) }
        </section>
    </section>
}
```

### Rust/JS/C-style comments in templates

Use single-line or multi-line Rust comments inside html-templates.

```rust
html! {
    <section>
   /* Write some ideas
    * in multiline comments
    */
    <p>{ "and tags could be placed between comments!" }</p>
    // <li>{ "or single-line comments" }</li>
    </section>
}
```

### Pure Rust expressions inside

```rust
extern crate chrono;
use chrono::prelude::*;

fn view(model: &Model) -> Html<Msg> {
    html! {
        <p>{ Local::now() }</p>
    }
}
```
