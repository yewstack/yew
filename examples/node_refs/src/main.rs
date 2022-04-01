mod input;

use input::InputComponent;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    HoverIndex(usize),
}

pub struct App {
    input_ref: NodeRef,
    input_comp_ref: HtmlRef<HtmlInputElement>,
    focus_index: usize,
}
impl App {
    fn apply_focus(&self) {
        if let Some(m) = self.input_ref.cast::<HtmlInputElement>() {
            m.focus().unwrap();
        }
        if let Some(m) = self.input_comp_ref.get() {
            m.focus().unwrap();
        }
    }
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            focus_index: 0,
            input_ref: NodeRef::default(),
            input_comp_ref: HtmlRef::default(),
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
                        ref={self.input_ref.clone()}
                        class="input-element"
                        onmouseover={ctx.link().callback(|_| Msg::HoverIndex(0))}
                    />
                </div>
                <div>
                    <label>{ "Using component ref: " }</label>
                    <InputComponent
                        ref={self.input_comp_ref.clone()}
                        on_hover={ctx.link().callback(|_| Msg::HoverIndex(1))}
                    />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
