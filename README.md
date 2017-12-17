# Yew

Elm like framework, but for Rust and WASM revolution.

## Minimal application

```rust
extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::html;

struct Model {
    value: u8,
}

enum Msg {
    Increment,
    Decrement,
}

fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::Increment => {
            model.value = model.value + 1;
        }
        Msg::Decrement => {
            model.value = model.value - 1;
        }
    }
}

fn view(model: &Model) -> html::Html<Msg> {
    html! {
        <div>
            <nav class="menu",>
                <button>{ "Increment" }</button>
                <button>{ "Decrement" }</button>
            </nav>
            <p>{ Local::now() }</p>
        </div>
    }
}

fn main() {
    let model = Model {
        value: 0,
    };
    html::program(model, update, view);
}
```
