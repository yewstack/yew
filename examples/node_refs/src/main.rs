mod input;

use input::InputComponent;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    HoverIndex(usize),
}

pub struct App {
    refs: Vec<NodeRef>,
    focus_index: usize,
}
impl App {
    fn apply_focus(&self) {
        if let Some(input) = self.refs[self.focus_index].cast::<HtmlInputElement>() {
            input.focus().unwrap();
        }
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.apply_focus();
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HoverIndex(index) => {
                self.focus_index = index;
                self.apply_focus();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="main">
                <h1>{ "Node Refs Example" }</h1>
                <p>{ "Refs can be used to access and manipulate DOM elements directly" }</p>
                <ul>
                    <li>{ "First input will focus on mount" }</li>
                    <li>{ "Each input will focus on hover" }</li>
                </ul>
                <div>
                    <label>{ "Using tag ref: " }</label>
                    <input
                        type="text"
                        ref={self.refs[0].clone()}
                        class="input-element"
                        onmouseover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                    />
                </div>
                <div>
                    <label>{ "Using component ref: " }</label>
                    <InputComponent
                        ref={self.refs[1].clone()}
                        on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                    />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
