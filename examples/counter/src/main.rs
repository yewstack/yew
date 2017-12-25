extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::html::*;

struct Model {
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Increment => {
            model.value = model.value + 1;
        }
        Msg::Decrement => {
            model.value = model.value - 1;
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <nav class="menu",>
                <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
            </nav>
            <p>{ model.value }</p>
            <p>{ Local::now() }</p>
        </div>
    }
}

fn main() {
    let model = Model {
        value: 0,
    };
    program(model, update, view);
}
