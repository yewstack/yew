extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::html::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

struct Model {
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Increment => {
            model.value = model.value + 1;
            context.console.log("plus one");
        }
        Msg::Decrement => {
            model.value = model.value - 1;
            context.console.log("minus one");
        }
        Msg::Bulk(list) => {
            for msg in list {
                update(context, model, msg);
            }
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <nav class="menu",>
                <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                <button onclick=|_| Msg::Bulk(vec!(Msg::Increment, Msg::Increment)),>{ "Increment Twice" }</button>
            </nav>
            <p>{ model.value }</p>
            <p>{ Local::now() }</p>
        </div>
    }
}

fn main() {
    yew::initialize();
    let mut app = App::new();
    let context = Context {
        console: ConsoleService,
    };
    let model = Model {
        value: 0,
    };
    app.mount(context, model, update, view);
    yew::run_loop();
}
