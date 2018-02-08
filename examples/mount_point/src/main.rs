#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::prelude::*;
use stdweb::web::{IElement, document, INode};

type Context = ();

struct Model {
    name: String,
}

enum Msg {
    UpdateName(String),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Msg, _: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <input value=&self.name, oninput=|e: InputData| Msg::UpdateName(e.value), />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
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

    let app: App<_, Model> = App::new(());
    app.mount(mount_point);
    yew::run_loop();
}
