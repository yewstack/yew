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
yew = { version = "0.15", package = "yew-stdweb" }
```
{% endcode %}

Copy the following template into your `src/main.rs` file:

{% code title="src/main.rs" %}
```rust
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}
```
{% endcode %}

This template sets up your root `Component`, called `Model` which shows a button that updates itself when you click it. Take special note of `App::<Model>::new().mount_to_body()` inside `main()` which starts your app and mounts it to the page's `<body>` tag. If you would like to start your application with any dynamic properties, you can instead use `App::<Model>::new().mount_to_body_with_props(..)`.

## Run your App!

Using [`cargo-web`](https://github.com/koute/cargo-web) is the quickest way to get up and running. If you haven't already, install the tool with `cargo install cargo-web` and then build and start a development server by running:

```bash
cargo web start
```

`cargo-web` will automatically add the `wasm32-unknown-unknown` target for you, build your app, and finally make it available at [http://\[::1\]:8000](http://[::1]:8000) by default. Consult `cargo web start --help` for other options.

