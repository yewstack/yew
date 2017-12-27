# Yew

Yew is a modern Rust framework inspired by Elm and ReactJS.

## Cutting Edge technologies

### Rust to WASM compilation

This framework designed to be compiled into modern browsers' runtimes: wasm, asm.js, emscripten.

### Clean MVC approach inspired by Elm

```rust
struct Model { }

enum Msg {
    DoIt,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::DoIt => {
            // Update your model on events
        }
    }
}

fn view(model: &Model) -> html::Html<Msg> {
    html! {
        // Render your model here
        <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
    }
}
```

### VirtualDOM

Yew framework uses own virtual-dom representation.

### JSX-like templates with `html!` macro

Put pure Rust code into HTML tags.

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

### Services

Pluggable services that allow you to call external APIs like:
JavaScript alerts, timeout, storage, fetches and websockets.
It's a handy alternative to subscriptions.

Implemented:
* `IntervalService`
* `TimeoutService`
* `StorageService`
* `AlertSerivce`

In development:
* `FetchService`
* `WebSocketService`

```rust
use yew::services::timeout::TimeoutService;

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Fire => {
            context.timeout(Duration::from_secs(5), || Msg::Timeout);
        }
        Msg::Timeout => {
            println!("Timeout!");
        }
    }
}
```

## Running the examples

There are four examples to check how it works:
[counter], [timer], [todomvc] and [crm].

To run them you need to have [cargo-web] installed as well as a suitable target
for the Rust compiler to generate web output. By default cargo-web uses
`asmjs-unknown-emscripten`. Install cargo-web and the asmjs emscripten target
as follows:

    $ cargo install cargo-web
    $ rustup target add asmjs-unknown-emscripten

To start an example enter its directory start it with [cargo-web]:

    $ cargo web start

To run an optimised build instead of a debug build use:

    $ cargo web start --release


[counter]: examples/counter
[todomvc]: examples/todomvc
[timer]: examples/timer
[crm]: examples/crm
[cargo-web]: https://github.com/koute/cargo-web
