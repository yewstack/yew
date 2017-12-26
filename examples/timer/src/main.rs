#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::html::*;
use yew::services::{Timeout, TimeoutHandle, Task};

struct Model {
    handle: Option<TimeoutHandle>,
    status: &'static str,
}

enum Msg {
    Fire,
    Cancel,
    Timeout,
}

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Fire => {
            let handle = context.timeout(Duration::from_secs(5), || Msg::Timeout);
            model.handle = Some(handle);
            model.status = "Counting...";
        }
        Msg::Cancel => {
            if let Some(mut handle) = model.handle.take() {
                handle.cancel();
                model.status = "Canceled!";
            }
        }
        Msg::Timeout => {
            model.handle = None;
            model.status = "Done!";
            println!("Timeout :)");
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <button onclick=|_| Msg::Fire,>{ "Fire!" }</button>
            <button disabled=model.handle.is_none(), onclick=|_| Msg::Cancel,>{ "Cancel!" }</button>
            <p>{ model.status }</p>
        </div>
    }
}

fn main() {
    let model = Model {
        handle: None,
        status: "Ready!",
    };
    program(model, update, view);
}
