[![Build Status](https://api.travis-ci.org/DenisKolodin/yew.svg)](https://travis-ci.org/DenisKolodin/yew)

# Yew

Yew is a modern Rust framework inspired by Elm and ReactJS.

## Cutting Edge technologies

### Rust to WASM compilation

This framework is designed to be compiled into modern browsers' runtimes: wasm, asm.js, emscripten.

To prepare the developments environment use installation instruction here: [wasm-and-rust](https://github.com/raphamorim/wasm-and-rust)

### Clean MVC approach inspired by Elm and Redux

Yew implements strict application state management based on message passing and updates:

`src/main.rs`

```rust
#[macro_use]
extern crate yew;
use yew::prelude::*;

type Context = ();

struct Model { }

enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}

fn main() {
    yew::initialize();
    let app: App<_, Model> = App::new(());
    app.mount_to_body();
    yew::run_loop();
}
```

Predictable mutability and lifetimes (thanks Rust) make it possible to reuse a single instance of the model
without needing to create a fresh one every update. It helps reduce memory allocations.

### JSX-like templates with `html!` macro

Feel free to put pure Rust code into HTML tags with all the compiler's and borrow checker benefits.

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

### Components

Yew supports components! You can create a new one by implementing a `Component` trait
and including it directly into the `html!` template:

```rust
html! {
    <nav class="menu",>
        <MyButton: title="First Button",/>
        <MyButton: title="Second Button",/>
    </nav>
}
```

### Scopes

Components lives in Angular-like scopes with **parent-to-child** *(properties)* and
**child-to-parent** *(events)* interaction.

Properties also are pure Rust types with strict checking during compilation.

```rust
html! {
    <nav class="menu",>
        <MyButton: color=Color::Red,/>
        <MyButton: onclick=|_| ParentMsg::DoIt,/>
    </nav>
}
```

### Fragments

Yew supports fragments: elements without a parent which could be attached somewhere later.

```rust
html! {
    <>
        <tr><td>{ "Row" }</td></tr>
        <tr><td>{ "Row" }</td></tr>
        <tr><td>{ "Row" }</td></tr>
    </>
}
```

### Virtual DOM, independent loops, fine updates

Yew framework uses its own **virtual-dom** representation. It updates the browser's DOM
with tiny patches when properties of elements had changed. Every component lives
in its own independent loop, interacts with the environment (`Scope`) by messages passing
and supports fine control of rendering.

The `ShouldRender` return value informs the loop when the component should be re-rendered:

```rust
fn update(&mut self, msg: Self::Message, _: &mut Env<Context, Self>) -> ShouldRender {
    match msg {
        Msg::UpdateValue(value) => {
            self.value = value;
            true
        }
        Msg::Ignore => {
            false
        }
    }
}
```

It's more effective than comparing the model after every update, because not every model
change leads to a view update. It lets us skip model comparison checks entirely.
You can control updates very accurately.

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

### Third-party crates and pure Rust expressions inside

You can use external crates and put values from them into the template:

```rust
extern crate chrono;
use chrono::prelude::*;

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <p>{ Local::now() }</p>
        }
    }
}
```

> Some crates don't support the true wasm target (`wasm32-unknown-unknown`) yet.

### Services

Yew has implemented pluggable services that allow you to call external APIs, such as:
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

impl Component<Context> for Model {
    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Fire => {
                let send_msg = context.send_back(|_| Msg::Timeout);
                context.timeout.spawn(Duration::from_secs(5), send_msg);
            }
            Msg::Timeout => {
                context.console.log("Timeout!");
            }
        }
    }
}
```

Can't find an essential service? Want to use library from `npm`?
You can reuse `JavaScript` libraries with `stdweb` capabilities and create
your own service implementation. Here's an example below of how to wrap the
[ccxt](https://www.npmjs.com/package/ccxt) library:

```rust
pub struct CcxtService(Option<Value>);

impl CcxtService {
    pub fn new() -> Self {
        let lib = js! {
            return ccxt;
        };
        CcxtService(Some(lib))
    }

    pub fn exchanges(&mut self) -> Vec<String> {
        let lib = self.0.as_ref().expect("ccxt library object lost");
        let v: Value = js! {
            var ccxt = @{lib};
            console.log(ccxt.exchanges);
            return ccxt.exchanges;
        };
        let v: Vec<String> = v.try_into().expect("can't extract exchanges");
        v
    }

    // Wrap more methods here!
}
```

### Easy-to-use data conversion and destructuring

Yew allows for serialization (store/send and restore/recieve) formats.

Implemented: `JSON`, `TOML`, `YAML`, `MSGPACK`, `CBOR`

In development: `BSON`, `XML`

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

impl Component<Context> for Model {
    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
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
}
```

By default only `Json` format available, but you can activate more with features in
`Cargo.toml` of your project:

```toml
[dependencies]
yew = { git = "https://github.com/DenisKolodin/yew", features = ["toml", "yaml", "msgpack", "cbor"] }
```

## Development setup

Clone or download this repository.

Add necessary targets to your compiler:

    $ rustup target add wasm32-unknown-emscripten

> We used `wasm32-unknown-emscripten` target here, because not every crate could be compiled to
the pure `wasm32-unknown-unknown` target. But the crates still improving and you can do it soon.

To build this project you need to have [cargo-web] installed:

    $ cargo install cargo-web

> Add `--force` option to ensure the latest version.

### Build

    $ cargo web build

### Running Tests

    $ ./ci/run_tests.sh

### Running the examples

There are many examples that show how the framework works:
[counter], [crm], [custom_components], [dashboard], [fragments],
[game_of_life], [mount_point], [npm_and_rest], [timer], [todomvc], [two_apps].

To start an example enter its directory and start it with [cargo-web]:

    $ cargo web start

To run an optimised build instead of a debug build use:

    $ cargo web start --release

**Note**: By default `cargo-web` will use Emscripten to generate asm.js. You can also
compile to WebAssembly if you add either `--target=wasm32-unknown-emscripten` or
`--target=wasm32-unknown-unknown`, where the first one will use Emscripten and
the second one will use Rust's native WebAssembly backend (Rust nightly only!).

[counter]: examples/counter
[crm]: examples/crm
[custom_components]: examples/custom_components
[dashboard]: examples/dashboard
[fragments]: examples/fragments
[game_of_life]: examples/game_of_life
[mount_point]: examples/mount_point
[npm_and_rest]: examples/npm_and_rest
[timer]: examples/timer
[todomvc]: examples/todomvc
[two_apps]: examples/two_apps
[cargo-web]: https://github.com/koute/cargo-web
