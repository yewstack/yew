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

fn update(_: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::DoIt => {
            // Update your model on events
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
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
* `DialogService`
* `FetchService`
* `WebSocketService`

```rust
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;

struct Context {
    console: ConsoleService,
    timeout: TimeoutService<Msg>,
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Fire => {
            context.timeout.spawn(Duration::from_secs(5), || Msg::Timeout);
        }
        Msg::Timeout => {
            context.console.log("Timeout!");
        }
    }
}
```

### Easy-to-use data conversion and destructuring

You could simply choose and use a format of data to store/send and restore/receive it.

Supported: `JSON`

In development: `BSON`, `TOML`, `YAML`, `XML`

```rust
use yew::format::Json;

#[derive(Serialize, Deserialize)]
struct Client {
    first_name: String,
    last_name: String,
}

struct Model {
    clients: Vec<Client>,
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
    Msg::Store => {
        // Stores it, but in JSON format/layout
        context.local_storage.store(KEY, Json(&model.clients));
    }
    Msg::Restore => {
        // Tries to read and destructure it as JSON formatted data
        if let Json(Ok(clients)) = context.local_storage.restore(KEY) {
            model.clients = clients;
        }
    }
}
```

## Running the examples

Clone or download this repository.

There are seven examples to check how it works:
[counter], [timer], [todomvc], [game_of_life], [crm], [dashboard] and [npm_and_rest].

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
[timer]: examples/timer
[todomvc]: examples/todomvc
[game_of_life]: examples/game_of_life
[crm]: examples/crm
[dashboard]: examples/dashboard
[npm_and_rest]: examples/npm_and_rest
[cargo-web]: https://github.com/koute/cargo-web
