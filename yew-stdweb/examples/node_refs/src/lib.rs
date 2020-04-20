#![recursion_limit = "256"]

mod input;

use input::InputComponent;
use stdweb::web::html_element::InputElement;
use stdweb::web::IHtmlElement;
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

    fn mounted(&mut self) -> ShouldRender {
        if let Some(input) = self.refs[self.focus_index].cast::<InputElement>() {
            input.focus();
        }
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HoverIndex(index) => self.focus_index = index,
        }
        if let Some(input) = self.refs[self.focus_index].cast::<InputElement>() {
            input.focus();
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
