#![recursion_limit = "256"]

mod input;

use input::InputComponent;
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    refs: Vec<NodeRef>,
    focus_index: usize,
}

pub enum Msg {
    HoverIndex(usize),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            focus_index: 0,
            refs: vec![NodeRef::default(), NodeRef::default()],
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.refs[self.focus_index].cast::<InputElement>() {
                input.focus().unwrap();
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HoverIndex(index) => self.focus_index = index,
        }
        if let Some(input) = self.refs[self.focus_index].cast::<InputElement>() {
            input.focus().unwrap();
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="main">
                <h1>{ "Node Refs Demo" }</h1>
                <p>{ "Refs can be used to access and manipulate DOM elements directly" }</p>
                <ul>
                    <li>{ "First input will focus on mount" }</li>
                    <li>{ "Each input will focus on hover" }</li>
                </ul>
                <div>
                    <label>{ "Using tag ref: " }</label>
                    <input
                        type="text"
                        ref=self.refs[0].clone()
                        class="input-element"
                        onmouseover=self.link.callback(|_| Msg::HoverIndex(0)) />
                </div>
                <div>
                    <label>{ "Using component ref: " }</label>
                    <InputComponent
                        ref=self.refs[1].clone()
                        on_hover=self.link.callback(|_| Msg::HoverIndex(1)) />
                </div>
            </div>
        }
    }
}
