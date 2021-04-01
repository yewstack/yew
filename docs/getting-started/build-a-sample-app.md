---
title: Build a sample app
---

## Create Project

To get started, create a new cargo project.

```bash
cargo new yew-app
```

Open the newly created directory.

```bash
cd yew-app
```

## Run a hello world example

To verify the Rust environment is setup, run the initial project using the cargo build tool.  After output about the build process, you should see the expected "Hello World" message.


```bash
cargo run
```

## Convert project to Yew web application

To convert this simple command line application to a basic Yew web application, a few changes are needed.

### Update Cargo.toml

Add `yew` to the list of dependencies in the `Cargo.toml` file.

```toml
[package]
name = "yew-app"
version = "0.1.0"
edition = "2018"

[dependencies]
# you can check the latest version here: https://crates.io/crates/yew
yew = "0.17"
```

### Update main.rs

Replace the contents of `src/main.rs` with the following code.

This template sets up your root `Component`, called `Model` which shows a button that updates itself when you click it.
Take special note of `yew::start_app::<Model>()` inside `main()` which starts your app and mounts it to the page's `<body>` tag.
If you would like to start your application with any dynamic properties, you can instead use `yew::start_app_with_props::<Model>(..)`.


```rust
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
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
    yew::start_app::<Model>();
}
```

### Create index.html

Finally, add an `index.html` file in the root directory of your app.

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

## View your web applcation

Run the following command to build your application, launch a local web hosting environment, and launch a web browser to view your application.

```bash
trunk serve
```

This will start a development server which continually updates the app every time you change something in your web application.


## Congratulations

You have now successfully setup your Yew development environment, and built your first web application.

Experiment with this application and review the [examples](./examples.md) to further your learning.
