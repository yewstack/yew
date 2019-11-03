use crate::Msg::FutureFinished;
use std::fmt::{Error, Formatter};
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct Model {
    future_data: Option<String>,
}

enum Msg {
    FutureFinished(String),
}

/// An error that can never happen (because an instance of this can not be created).
#[derive(Debug, Clone, PartialEq)]
pub enum InfallibleFutureError {}
impl std::fmt::Display for InfallibleFutureError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), Error> {
        unimplemented!()
    }
}
impl std::error::Error for InfallibleFutureError {}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let future = async {
            let x: Result<Msg, InfallibleFutureError> =
                Ok(Msg::FutureFinished("Hello Future World!".to_string()));
            x
        };
        link.send_future(future);
        Model { future_data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FutureFinished(resolved_future) => {
                self.future_data = Some(resolved_future);
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            if let Some(future_data) = &self.future_data {
                html! {
                    &future_data
                }
            } else {
                html! {
                    "no future yet"
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<Model>();
}
