---
title: Build a sample app
---
First create a new Rust library \(**important:** create a _library_, not a _binary_ by passing the `--lib` flag\):

```bash
cargo new --lib yew-app && cd yew-app
```

Add `yew` and `wasm-bindgen` to your dependencies \(refer [here](https://docs.rs/yew) for the latest version\)

```toml title="Cargo.toml"
[package]
name = "yew-app"
version = "0.1.0"
authors = ["Yew App Developer <name@example.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
yew = "0.17"
wasm-bindgen = "0.2.67"
```

Copy the following template into your `src/lib.rs` file:

```rust title="src/lib.rs"
use wasm_bindgen::prelude::*;
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

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
```

This template sets up your root `Component`, called `Model` which shows a button that updates itself when you click it. Take special note of `App::<Model>::new().mount_to_body()` inside `main()` which starts your app and mounts it to the page's `<body>` tag. If you would like to start your application with any dynamic properties, you can instead use `App::<Model>::new().mount_to_body_with_props(..)`.

Finally, add an `index.html` file into a new folder named `static` in your app.

```bash
mkdir static
```

```markup title="index.html"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Sample App</title>
        <script type="module">
            import init from "./wasm.js"
            init()
        </script>
    </head>
    <body></body>
</html>
```

## Run your app!

Using [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/) is the preferred way to get up and 
running. If you haven't already, install `wasm-pack` with `cargo install wasm-pack` and then build 
and start a development server by running:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static
```

`wasm-pack` generates a bundle in the `./static` directory with your app's compiled WebAssembly 
along with a JavaScript wrapper which will load your application's WebAssembly binary and run it.

Then, use your favorite web server to serve the files under `./static`. For example:

```bash
cargo +nightly install miniserve
miniserve ./static --index index.html
```

