#![recursion_limit = "256"]

use yew::html::Scope;
/// This example demonstrates low-level usage of scopes.
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
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

    fn view(&self) -> Html {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("One".into()))>{ "One" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Two".into()))>{ "Two" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Three".into()))>{ "Three" }</button>
                <button onclick=self.link.callback(|_| Msg::SendToOpposite("Ping".into()))>{ "Ping" }</button>
            </div>
        }
    }
}
