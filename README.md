<div align="center">

  <img src="https://github.com/yewstack/yew/blob/master/.static/yew.svg" width="150" />

  <h1>
    Yew &nbsp;
    <a href="https://crates.io/crates/yew"><img alt="Build Status" src="https://img.shields.io/crates/v/yew.svg"/></a>
  </h1>

  <p>
    <strong>Rust / Wasm client web app framework</strong>
  </p>

  <p>
    <a href="https://travis-ci.com/yewstack/yew"><img alt="Build Status" src="https://travis-ci.com/yewstack/yew.svg?branch=master"/></a>
    <a href="https://gitter.im/yewframework/Lobby"><img alt="Gitter Chat" src="https://badges.gitter.im/yewframework.svg"/></a>
    <a href="https://blog.rust-lang.org/2019/11/07/Rust-1.39.0.html"><img alt="Rustc Version 1.39+" src="https://img.shields.io/badge/rustc-1.39+-lightgray.svg"/></a>
  </p>

  <h4>
    <a href="https://yew.rs">Website</a>
    <span> | </span>
    <a href="https://docs.rs/yew">API Docs</a>
    <span> | </span>
    <a href="#running-the-examples">Examples</a>
    <span> | </span>
    <a href="https://github.com/yewstack/yew/blob/master/CHANGELOG.md">Changelog</a>
    <span> | </span>
    <a href="https://yew.rs/docs/roadmap">Roadmap</a>
    <span> | </span>
    <a href="https://github.com/yewstack/yew/blob/master/CODE_OF_CONDUCT.md">Code of Conduct</a>
  </h4>
</div>

## Overview

**Yew** is a modern Rust framework inspired by Elm and React for
creating multi-threaded frontend apps with WebAssembly.

The framework supports ***multi-threading & concurrency*** out of the box.
It uses [Web Workers API] to spawn actors (agents) in separate threads
and uses a local scheduler attached to a thread for concurrent tasks.

[Web Workers API]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API

[Become a sponsor on Patreon](https://www.patreon.com/deniskolodin)

[Check out a live demo](https://yew-todomvc.netlify.com/) powered by [`yew-wasm-pack-template`](https://github.com/yewstack/yew-wasm-pack-template)

## Cutting Edge technologies

### Rust to WASM compilation

This framework is designed to be compiled into modern browsers' runtimes: wasm, asm.js, emscripten.

### Architecture inspired by Elm and Redux

Yew implements strict application state management based on message passing and updates:

`src/main.rs`

```rust
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    link: ComponentLink<Self>,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
                true
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::DoIt);
        html! {
            // Render your model here
            <button onclick=onclick>{ "Click me!" }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```

Predictable mutability and lifetimes (thanks Rust!) make it possible to reuse a single instance of the model
without a need to create a fresh one on every update. It also helps to reduce memory allocations.

### JSX-like templates with `html!` macro

Feel free to put pure Rust code into HTML tags with all the compiler and borrow checker's benefits.

```rust
html! {
    <section class="todoapp">
        <header class="header">
            <h1>{ "todos" }</h1>
            { view_input(&model) }
        </header>
        <section class="main">
            <input class="toggle-all"
                   type="checkbox"
                   checked=model.is_all_completed()
                   onclick=|_| Msg::ToggleAll />
            { view_entries(&model) }
        </section>
    </section>
}
```

### Agents - actor model inspired by Erlang and Actix

Every `Component` can spawn an agent and attach to it.
Agents can coordinate global state, spawn long-running tasks, and offload tasks to a web worker.
They run independently of components, but hook nicely into their update mechanism.

```rust
use yew::worker::*;

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
    // - `Job` (one per bridge on the main thread)
    // - `Context` (shared in the main thread)
    // - `Private` (one per bridge in a separate thread)
    // - `Public` (shared in a separate thread)
    type Reach = Context; // Spawn only one instance on the main thread (all components can share this agent)
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    // Create an instance with a link to the agent.
    fn create(link: AgentLink<Self>) -> Self {
        Worker { link }
    }

    // Handle inner messages (from callbacks)
    fn update(&mut self, msg: Self::Message) { /* ... */ }

    // Handle incoming messages from components of other agents.
    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Question(_) => {
                self.link.respond(who, Response::Answer("That's cool!".into()));
            },
        }
    }
}
```

Build the bridge to an instance of this agent.
It spawns a worker automatically or reuses an existing one, depending on the type of the agent:

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
        let callback = link.callback(|_| Msg::ContextMsg);
        // `Worker::bridge` spawns an instance if no one is available
        let context = context::Worker::bridge(callback); // Connected! :tada:
        Model { context }
    }
}
```

You can use as many agents as you want. For example you could separate all interactions
with a server to a separate thread (a real OS thread because Web Workers map to the native threads).

> **REMEMBER!** Not every API is available for every environment. For example you can't use
`StorageService` from a separate thread. It won't work with `Public` or `Private` agents,
only with `Job` and `Context` ones.

### Components

Yew supports components! You could create a new one by implementing a `Component` trait
and including it directly into the `html!` template:

```rust
html! {
    <nav class="menu">
        <MyButton title="First Button" />
        <MyButton title="Second Button "/>
        <MyList name="Grocery List">
          <MyListItem text="Apples" />
        </MyList>
    </nav>
}
```

### Scopes

Components live in an Angular-like scopes with **parent-to-child** *(properties)* and
**child-to-parent** *(events)* interaction.

Properties are also pure Rust types with strict type-checking during the compilation.

```rust
// my_button.rs

#[derive(Clone, Properties, PartialEq)]
pub struct Properties {
    pub hidden: bool,
    #[props(required)]
    pub color: Color,
    #[props(required)]
    pub onclick: Callback<()>,
}

```

```rust
// confirm_dialog.rs

html! {
    <div class="confirm-dialog">
        <MyButton onclick=|_| DialogMsg::Cancel color=Color::Red hidden=true />
        <MyButton onclick=|_| DialogMsg::Submit color=Color::Blue />
    </div>
}
```

### Fragments

Yew supports fragments: elements without a parent which can be attached to one somewhere else.

```rust
html! {
    <>
        <tr><td>{ "Row" }</td></tr>
        <tr><td>{ "Row" }</td></tr>
        <tr><td>{ "Row" }</td></tr>
    </>
}
```

### Virtual DOM

Yew uses its own **virtual-dom** implementation. It updates the browser's DOM with tiny patches when properties of elements have changed. Every component can be interacted with using its (`Scope`) to pass messages and trigger updates.

The `ShouldRender` returns the value which informs the loop when the component should be re-rendered:

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

Using `ShouldRender` is more effective than comparing the model after every update because not every change to the model
causes an update to the view. It allows the framework to only compare parts of the model essential to rendering the view.

### Rust/JS/C-style comments in templates

Use single-line or multi-line Rust comments inside html-templates.

```rust
html! {
    <section>
   /* Write some ideas
    * in multiline comments
    */
    <p>{ "and tags can be placed between comments!" }</p>
    // <li>{ "or single-line comments" }</li>
    </section>
}
```

### Third-party crates and pure Rust expressions inside

Use external crates and put values from them into the template:

```rust
extern crate chrono;
use chrono::prelude::*;

impl Renderable for Model {
    fn render(&self) -> Html {
        html! {
            <p>{ Local::now() }</p>
        }
    }
}
```

> Some crates don't support the `wasm32-unknown-unknown` target yet.

### Services

Yew has implemented pluggable services that allow you to call external APIs, such as:
JavaScript alerts, timeout, storage, fetches and websockets.
It's a handy alternative to subscriptions.

Implemented:
* `IntervalService`
* `RenderService`
* `ResizeService`
* `TimeoutService`
* `StorageService`
* `DialogService`
* `ConsoleService`
* `FetchService`
* `WebSocketService`
* `KeyboardService`

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
                let timeout = self.link.callback(|_| Msg::Timeout);
                self.timeout.spawn(Duration::from_secs(5), timeout);
            }
            Msg::Timeout => {
                self.console.log("Timeout!");
            }
        }
    }
}
```

Can't find an essential service? Want to use a library from `npm`?
You can wrap `JavaScript` libraries using `stdweb` and create
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

Yew allows for serialization (store/send and restore/receive) formats.

Implemented: `JSON`, `TOML`, `YAML`, `MSGPACK`, `CBOR`.

In development: `BSON`, `XML`.

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

Only `JSON` is available by default but you can activate the rest through features in
your project's `Cargo.toml`:

```toml
[dependencies]
yew = { git = "https://github.com/yewstack/yew", features = ["toml", "yaml", "msgpack", "cbor"] }
```

## Development setup

Clone or download this repository.

### Install [cargo-web]

This is an optional tool that simplifies deploying web applications:

```bash
cargo install cargo-web
```

> Add `--force` option to ensure you install the latest version.

### Build

```bash
cargo web build

# without cargo-web, only the wasm32-unknown-unknown target is supported
cargo build --target wasm32-unknown-unknown
```

### Running Tests
For the tests to work one have to ensure that `wasm-bindgen-cli` is installed. 
[Instructions](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#install-the-test-runner)

Additionally a webdriver must be installed locally and configured to be on the 
`PATH`. Currently supports `geckodriver`, `chromedriver`, and `safaridriver`,
although more driver support may be added! You can download these at:

* geckodriver - https://github.com/mozilla/geckodriver/releases
* chromedriver - http://chromedriver.chromium.org/downloads
* safaridriver - should be preinstalled on OSX

```bash
./ci/run_tests.sh
```

### Running the examples

There are many examples that show how the framework works:
[counter], [crm], [custom_components], [dashboard], [fragments],
[game_of_life], [mount_point], [npm_and_rest], [timer], [todomvc], [two_apps].

To start an example enter its directory and start it with [cargo-web]:

```bash
cargo web start
```

To run an optimised build instead of a debug build use:

```bash
cargo web start --release
```

This will use the `wasm32-unknown-unknown` target by default, which is Rust's native WebAssembly target.
The Emscripten-based `wasm32-unknown-emscripten` and `asmjs-unknown-emscripten` targets are also supported
if you tell the `cargo-web` to build for them using the `--target` parameter.

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


## Project templates

* [`yew-wasm-pack-template`](https://github.com/yewstack/yew-wasm-pack-template)
* [`yew-wasm-pack-minimal`](https://github.com/yewstack/yew-wasm-pack-minimal)

## Contributors

### Code Contributors

This project exists thanks to all the people who contribute. [[Contribute](CONTRIBUTING.md)].
<a href="https://github.com/yewstack/yew/graphs/contributors"><img src="https://opencollective.com/yew/contributors.svg?width=890&button=false" /></a>

### Financial Contributors

Become a financial contributor and help us sustain our community. [[Contribute](https://opencollective.com/yew/contribute)]

#### Individuals

<a href="https://opencollective.com/yew"><img src="https://opencollective.com/yew/individuals.svg?width=890"></a>

#### Organizations

Support this project with your organization. Your logo will show up here with a link to your website. [[Contribute](https://opencollective.com/yew/contribute)]

<a href="https://opencollective.com/yew/organization/0/website"><img src="https://opencollective.com/yew/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/1/website"><img src="https://opencollective.com/yew/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/2/website"><img src="https://opencollective.com/yew/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/3/website"><img src="https://opencollective.com/yew/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/4/website"><img src="https://opencollective.com/yew/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/5/website"><img src="https://opencollective.com/yew/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/6/website"><img src="https://opencollective.com/yew/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/7/website"><img src="https://opencollective.com/yew/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/8/website"><img src="https://opencollective.com/yew/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/yew/organization/9/website"><img src="https://opencollective.com/yew/organization/9/avatar.svg"></a>
