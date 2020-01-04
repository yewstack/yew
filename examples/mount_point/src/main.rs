#[cfg(feature = "std_web")]
#[macro_use]
extern crate stdweb;

use mount_point::Model;
#[cfg(feature = "std_web")]
use stdweb::web::{document, IElement, INode, IParentNode};
use yew::App;
#[cfg(feature = "web_sys")]
use ::{
    wasm_bindgen::JsValue,
    web_sys::{CanvasRenderingContext2d, HtmlCanvasElement},
};

fn main() {
    yew::initialize();
    #[cfg(feature = "std_web")]
    let document = document();
    #[cfg(feature = "web_sys")]
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.query_selector("body").unwrap().unwrap();

    // This canvas won't be overwritten by yew!
    let canvas = document.create_element("canvas").unwrap();
    #[cfg_attr(feature = "std_web", allow(unused_variables))]
    let result = body.append_child(&canvas);
    #[cfg(feature = "web_sys")]
    result.unwrap();

    #[cfg(feature = "std_web")]
    js! {
        const canvas = document.querySelector("canvas");
        canvas.width = 100;
        canvas.height = 100;
        const ctx = canvas.getContext("2d");
        ctx.fillStyle = "green";
        ctx.fillRect(10, 10, 50, 50);
    };
    #[cfg(feature = "web_sys")]
    {
        let canvas = HtmlCanvasElement::from(JsValue::from(canvas));
        canvas.set_width(100);
        canvas.set_height(100);
        let ctx = CanvasRenderingContext2d::from(JsValue::from(
            canvas.get_context("2d").unwrap().unwrap(),
        ));
        ctx.set_fill_style(&JsValue::from_str("green"));
        ctx.fill_rect(10., 10., 50., 50.);
    }

    let mount_class = "mount-point";
    let mount_point = document.create_element("div").unwrap();
    let class_list = mount_point.class_list();
    #[cfg(feature = "std_web")]
    class_list.add(mount_class).unwrap();
    #[cfg(feature = "web_sys")]
    class_list.add_1(mount_class).unwrap();
    #[cfg_attr(feature = "std_web", allow(unused_variables))]
    let result = body.append_child(&mount_point);
    #[cfg(feature = "web_sys")]
    result.unwrap();

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}
