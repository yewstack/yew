use wasm_bindgen::JsValue;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlInputElement, InputEvent,
};
use yew::{html, Component, Context, Html, TargetCast};

pub enum Msg {
    UpdateName(String),
}

pub struct App {
    name: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "Reversed".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateName(new_name) => {
                self.name = new_name;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <input
                    value={self.name.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input = e.target_unchecked_into::<HtmlInputElement>();
                        Msg::UpdateName(input.value())
                    })}
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
    let document = gloo::utils::document();
    let body = document.query_selector("body").unwrap().unwrap();

    let canvas = create_canvas(&document);
    // This canvas won't be overwritten by yew!
    body.append_child(&canvas).unwrap();

    let mount_point = document.create_element("div").unwrap();
    let class_list = mount_point.class_list();
    class_list.add_1("mount-point").unwrap();

    body.append_child(&mount_point).unwrap();

    yew::Renderer::<App>::with_root(mount_point).render();
}
