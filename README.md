# Yew

Yew is a modern Rust framework inspired by Elm and ReactJS.

## Cutting Edge technologies

### Rust to WASM compilation

This framework designed to be compiled into modern browsers' runtimes: wasm, asm.js, emscripten.

### Clean MVC approach inspired by Elm and Redux

Yew implements strict application state management based on messages passing and updates:

```rust
type Context = ();

struct Model { }

enum Msg {
    DoIt,
}

impl Component<Context> for Model {
    // Some details omitted. Explore the examples to get more.
    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                // Update your model on events
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

Predictable mutability and lifetimes (thanks Rust) make possible to reuse an instance
of the model and we shouldn't create a new one after every update. It reduces memory allocations.

### JSX-like templates with `html!` macro

Feel free to put pure Rust code into HTML tags with all compiler's and borrow checker benefits.

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
and include it directly into the `html!` template:

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
        <MyButton: title="First Button", color=Color::Red,/>
        <MyButton: title="Second Button", onclick=|_| ParentMsg::DoIt,/>
    </nav>
}
```

### Virtual DOM, independent loops, fine updates

Yew framework uses own **virtual-dom** representation. It updates DOM in a browser
with tiny patches when properties of elements had changed. Every component lives
in own independent loop, interacts with environment (`Scope`) by messages passing
and supports fine control of rendering.

Set `ShouldRender` flag to inform the loop when component should be re-rendered:

```rust
fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
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

It's more effective than compare model after every update, because not every model
change leads the changes of the view and we don't need to spend time for comparsion at all.
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

fn view(model: &Model) -> Html<Msg> {
    html! {
        <p>{ Local::now() }</p>
    }
}
```

> Some crates hasn't support true wasm target (`wasm32-unknown-unknown`) yet.

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

There is no necessary service? Want to use library from `npm`?
You can reuse `JavaScript` libraries with `stdweb` capabilities and create
own service implementation. For example let's wrap
[ccxt](https://www.npmjs.com/package/ccxt) library below:

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
