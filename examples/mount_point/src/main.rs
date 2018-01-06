#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::html::{App, Html, InputData};
use stdweb::web::{IElement, document, INode};

struct Context {}

struct Model {
    name: String,
}

enum Msg {
    UpdateName(String),
}

fn update(_: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::UpdateName(new_name) => {
            model.name = new_name;
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <input value=&model.name, oninput=|e: InputData| Msg::UpdateName(e.value), />
            <p>{ model.name.chars().rev().collect::<String>() }</p>
        </div>
    }
}

fn main() {
    yew::initialize();
    let body = document().query_selector("body").unwrap();

    // This canvas won't be overwritten by yew!
    let canvas = document().create_element("canvas");
    body.append_child(&canvas);

    js! {
        const canvas = document.querySelector("canvas");
        canvas.width = 100;
        canvas.height = 100;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 50, 50);
    };

    let mount_class = "mount-point";
    let mount_point = document().create_element("div");
    mount_point.class_list().add(mount_class);
    body.append_child(&mount_point);

    let mut app = App::new();
    let context = Context {};
    let model = Model {
        name: "Reversed".to_owned(),
    };

    let mount_point = format!(".{}", mount_class);
    app.land_to(&mount_point, context, model, update, view);
    yew::run_loop();
}
