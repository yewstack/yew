#![recursion_limit = "256"]

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
use yew::components::Select;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Clone, Debug, Display, EnumString, EnumIter, PartialEq)]
pub enum Fruit {
    Apple,
    Banana,
    Grape,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: Option<Fruit>,
}

pub enum Msg {
    Set(Fruit),
    Clear,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model { link, value: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Set(fruit) => {
                self.value = Some(fruit);
            }
            Msg::Clear => {
                self.value = None;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Select<Fruit>
                    selected=self.value.clone()
                    options=Fruit::iter().collect::<Vec<_>>()
                    onchange=self.link.callback(Msg::Set) />
                <p>{ format!("Selected = {:?}", self.value) }</p>
                <button onclick=self.link.callback(|_| Msg::Clear)>{ "Reset" }</button>
            </div>
        }
    }
}
