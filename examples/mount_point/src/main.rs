use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub enum Msg {
    UpdateName(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    name: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    value=&self.name
                    oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
                />
                <p>{ self.name.chars().rev().collect::<String>() }</p>
            </div>
        }
    }
}

fn create_canvas(document: &Document) -> HtmlCanvasElement {
    let canvas = HtmlCanvasElement::from(JsValue::from(document.create_element("canvas").unwrap()));
    canvas.set_width(100);
    canvas.set_height(100);
    let ctx =
        CanvasRenderingContext2d::from(JsValue::from(canvas.get_context("2d").unwrap().unwrap()));
    ctx.set_fill_style(&JsValue::from_str("green"));
    ctx.fill_rect(10., 10., 50., 50.);

    canvas
}

fn main() {
    let document = yew::utils::document();
    let body = document.query_selector("body").unwrap().unwrap();

    let canvas = create_canvas(&document);
    // This canvas won't be overwritten by yew!
    body.append_child(&canvas).unwrap();

    let mount_point = document.create_element("div").unwrap();
    let class_list = mount_point.class_list();
    class_list.add_1("mount-point").unwrap();

    body.append_child(&mount_point).unwrap();

    yew::App::<Model>::new().mount(mount_point);
}
