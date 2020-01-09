# Build a Sample App

First create a new binary project:

```bash
cargo new --bin yew-app && cd yew-app
```

Add `yew` to your dependencies \(refer [here](https://docs.rs/yew) for the latest version\)

{% code title="Cargo.toml" %}
```text
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[dependencies]
yew = "0.11.0"
```
{% endcode %}

Copy the following template into your `src/main.rs` file:

{% code title="src/main.rs" %}
```rust
use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = true;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked { "Clicked!" } else { "Click me!" };

        html! {
            <button onclick=&self.onclick>{ button_text }</button>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
```
{% endcode %}

This template sets up your root `Component`, called `App` which shows a button that updates itself when you click it. Take special note of `yew::start_app::<Model>()` inside `main()` which starts your app and mounts it to the page's `<body>` tag. If you would like to start your application with any dynamic properties, you can instead use `yew::start_app_with_props(..)`.

### Run your App!

Using [`cargo-web`](https://github.com/koute/cargo-web) is the quickest way to get up and running. If you haven't already, install the tool with `cargo install cargo-web` and then build and start a development server by running:

```bash
cargo web start
```

`cargo-web` will automatically add the `wasm32-unknown-unknown` target for you, build your app, and finally make it available at [http://\[::1\]:8000](http://[::1]:8000) by default. Consult `cargo web start --help` for other options.

