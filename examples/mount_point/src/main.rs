#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

use yew::html::*;
use stdweb::web::{IElement, document, INode};

struct Model {
    name: String,
}

enum Msg {
    UpdateName(String),
}

impl Component<()> for Model {
    type Msg = Msg;

    fn create(_: &mut ScopeRef<(), Msg>) -> Self {
        Model {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Msg, _: &mut ScopeRef<(), Msg>) {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
            }
        }
    }

    fn view(&self) -> Html<(), Msg> {
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

    let app = Scope::new(());
    app.mount::<Model>(mount_point);
    yew::run_loop();
}
