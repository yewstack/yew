[![Build Status](https://api.travis-ci.org/DenisKolodin/yew.svg)](https://travis-ci.org/DenisKolodin/yew)

# Yew

Yew is a modern Rust framework inspired by Elm and ReactJS for
creating multi-threaded frontent apps with WebAssembly.

**NEW!** The framework supports ***multi-threading & concurrency*** out of the box.
It uses [Web Workers API] for spawning actors (agents) in separate threads
and uses a local scheduler attached to a thread for spawning concurrent tasks.

[Web Workers API]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API

[Become a sponsor on Patreon](https://www.patreon.com/deniskolodin)

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

struct Model { }

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to get more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
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

### Agents - actors model inspired by Erlang and Actix

Every `Component` could spawn an agent and attach to it.
Agetns are separate tasks which works concurrently.

Create your worker/agent (in `context.rs` for example):

```rust
use yew::prelude::worker::*;

struct Worker {
    link: AgentLink<Worker>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    Question(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Answer(String),
}

impl Agent for Worker {
    // Available:
    // - `Job` (one per bridge)
    // - `Context` (shared in the same thread)
    // - `Public` (separate thread).
    type Reach = Context; // Spawn only one instance per thread (all componentis could reach this)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Creates an instance with a link to agent's environment.
    fn create(link: AgentLink<Self>) -> Self {
        Worker { link }
    }

    // Implement it for handling inner messages (of services of `send_back` callbacks)
    fn update(&mut self, msg: Self::Message) { /* ... */ }

    // Implement it for handling incoming messages form components of other agents.
    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Question(_) => {
                self.link.response(who, Response::Answer("That's cool!".into()));
            },
        }
    }
}
```

Build the bridge to an instance of this agent.
It spawns a worker automatically or reuse an existent (it depends of type of the agent):

```rust
struct Model {
    context: Box<Bridge<context::Worker>>,
}

enum Msg {
    ContextMsg(context::Response),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::ContextMsg);
        // `Worker::bridge` method spawns an instance if no one available
        let context = context::Worker::bridge(callback); // Connected! :tada:
        Model { context }
    }
}
```

You could use as many agents as you want. For example you could separate all interactions
with a server to a separate thread (real OS thread, because Web Workers maps to native threads).

> **REMEMBER!** Not every APIs available for every environment. For example you couldn't use
`StorageService` from a separate thread that means it won't work with `Public` kind of agent,
but local storage available for `Job` and `Context` kind of agents.

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
fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
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
use yew::services::{ConsoleService, TimeoutService};

struct Model {
    link: ComponentLink<Model>,
    console: ConsoleService,
    timeout: TimeoutService,
}

impl Component for Model {
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Fire => {
                let send_msg = self.link.send_back(|_| Msg::Timeout);
                self.timeout.spawn(Duration::from_secs(5), send_msg);
            }
            Msg::Timeout => {
                self.console.log("Timeout!");
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
    local_storage: StorageService,
    clients: Vec<Client>,
}

impl Component for Model {
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        Msg::Store => {
            // Stores it, but in JSON format/layout
            self.local_storage.store(KEY, Json(&model.clients));
        }
        Msg::Restore => {
            // Tries to read and destructure it as JSON formatted data
            if let Json(Ok(clients)) = self.local_storage.restore(KEY) {
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
