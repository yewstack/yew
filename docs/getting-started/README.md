---
description: How to set up and build your app
---

# Getting Started

## Quick Sample App

First create a new binary project:

```bash
cargo new --bin yew-app && cd yew-app
```

Add yew to your dependencies \(refer [here](https://docs.rs/yew) for the latest version\)

{% code title="Cargo.toml" %}
```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[dependencies]
yew = "0.10.0"
```
{% endcode %}

Copy this template into your `src/main.rs` file:

{% code title="src/main.rs" %}
```rust
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct App {
    clicked: bool,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { clicked: false }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };
        
        html! {
            <button onclick=|_| Msg::Click>{ button_text }</button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
```
{% endcode %}

This template sets up your root `Component`, called `App` which shows a button which updates itself when you click it. `yew::start_app::<Model>()` starts your app and mounts it to the page's `<body>` tag.

#### Run your App!

Using [`cargo-web`](https://github.com/koute/cargo-web) is the quickest way to get up and running. First install the tool with `cargo install cargo-web` and then to build and start a development server, run:

```bash
cargo web start
```

This compiles using the `wasm32-unknown-unknown` target and will make your application available at [http://\[::1\]:8000](http://[::1]:8000) by default. Consult `cargo web start --help` for other options.

