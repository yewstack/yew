#![recursion_limit = "128"]

use yew::html::Scope;
/// This example demonstrates low-level usage of scopes.
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    scope: Option<Scope<Model>>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SetScope(Scope<Model>),
    SendToOpposite(String),
    SetTitle(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            scope: None,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetScope(scope) => {
                self.scope = Some(scope);
            }
            Msg::SendToOpposite(title) => {
                self.scope
                    .as_mut()
                    .unwrap()
                    .send_message(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                match title.as_ref() {
                    "Ping" => {
                        self.scope
                            .as_mut()
                            .unwrap()
                            .send_message(Msg::SetTitle("Pong".into()));
                    }
                    "Pong" => {
                        self.scope
                            .as_mut()
                            .unwrap()
                            .send_message(Msg::SetTitle("Pong Done".into()));
                    }
                    "Pong Done" => {
                        self.scope
                            .as_mut()
                            .unwrap()
                            .send_message(Msg::SetTitle("Ping Done".into()));
                    }
                    _ => {}
                }
                self.title = title;
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=|_| Msg::SendToOpposite("One".into())>{ "One" }</button>
                <button onclick=|_| Msg::SendToOpposite("Two".into())>{ "Two" }</button>
                <button onclick=|_| Msg::SendToOpposite("Three".into())>{ "Three" }</button>
                <button onclick=|_| Msg::SendToOpposite("Ping".into())>{ "Ping" }</button>
            </div>
        }
    }
}
