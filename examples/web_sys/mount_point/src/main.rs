use mount_point_web_sys::Model;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::App;

fn main() {
    yew::initialize();
    let document = yew::utils::document();
    let body = document.query_selector("body").unwrap().unwrap();

    // This canvas won't be overwritten by yew!
    let canvas = document.create_element("canvas").unwrap();
    body.append_child(&canvas).unwrap();

    let canvas = HtmlCanvasElement::from(JsValue::from(canvas));
    canvas.set_width(100);
    canvas.set_height(100);
    let ctx =
        CanvasRenderingContext2d::from(JsValue::from(canvas.get_context("2d").unwrap().unwrap()));
    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill_rect(10., 10., 50., 50.);

    let mount_class = "mount-point";
    let mount_point = document.create_element("div").unwrap();
    let class_list = mount_point.class_list();
    class_list.add_1(mount_class).unwrap();
    body.append_child(&mount_point).unwrap();

    App::<Model>::new().mount(mount_point);
    yew::run_loop();
}
