#[macro_use]
extern crate stdweb;

use mount_point_std_web::Model;
use stdweb::web::{document, IElement, INode, IParentNode};
use yew::App;

fn main() {
    yew::initialize();
    let body = document().query_selector("body").unwrap().unwrap();

    // This canvas won't be overwritten by yew!
    let canvas = document().create_element("canvas").unwrap();
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
    let mount_point = document().create_element("div").unwrap();
    mount_point.class_list().add(mount_class).unwrap();
    body.append_child(&mount_point);

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}
