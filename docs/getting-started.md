---
description: How to set up and build your app
---

# Getting Started

## Project Setup for Cargo Web

Using cargo-web is pretty straight-forward:

You create a new binary project:

```bash
cargo new --bin my-app && cd my-app
```

Add yew to your dependencies:

{% code title="Cargo.toml" %}
```text
[package]
name = "my-app"
version = "0.1.0"
authors = ["Your Name Here <name@example.com>"]
edition = "2018"

[dependencies]
yew = "0.10.0"
```
{% endcode %}

And copy the template into your `src/main.rs` file:

{% code title="src/main.rs" %}
```rust
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model { }

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details are omitted. Explore the examples to see more.
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

    fn view(&self) -> Html<Self> {
        html! {
            // Render your model here
            <button onclick=|_| Msg::DoIt>{ "Click me!" }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```
{% endcode %}

This template sets up your root component, called `Model` which shows a button that sends an message that does nothing when you click it. `yew::start_app::<Model>()` starts your app with the `Model` component as the root component.

#### Building using Cargo Web

To build and start a development server, run:

```bash
cargo web start
```

This compile using the `wasm32-unknown-unknown` target and will make your application available at [http://\[::1\]:8000](http://[::1]:8000) by default. Consult `cargo web start --help` for other options.

## Project Setup for wasm-pack

If you want to use a JS Bundler instead of cargo web, it is recommended that you copy from an existing template.

* [Minimal](https://github.com/yewstack/yew-wasm-pack-minimal) - Uses wasm-pack, and rollup to build your application, and your own server to serve it. No bells or whistles here.
  * `python -m SimpleHTTPServer 8080` Is a good quick and dirty server for development purposes.
* [WebPack](https://github.com/yewstack/yew-wasm-pack-template) - WebPack is used to manage your development and deployments via a wasm-pack plugin.
* Parcel - Parcel is used to manage your development and deployments

The important distinction between this approach and using cargo-web is that this approach uses a lib, not a bin target, and the entry-point to your program is annotated with `#[wasm_bindgen]` annotation and is called from a JS file you must supply yourself.

Your `Cargo.toml` also should specify that you have a cdylib crate-type.

{% code title="Cargo.toml" %}
```text
[package]
name = "my-app"
version = "0.1.0"
authors = ["Your Name Here <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
yew = "0.10.0"
wasm-bindgen = "0.2"
```
{% endcode %}



