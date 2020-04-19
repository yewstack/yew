#![recursion_limit = "256"]

use yew::{html, App, Component, ComponentLink, Html, ShouldRender};
use yew::html::{Scope};

pub struct Model {
    link: ComponentLink<Self>,
    opposite: Option<ComponentLink<Model>>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SetOpposite(ComponentLink<Model>),
    SendToOpposite(String),
    SetTitle(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            opposite: None,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetOpposite(opposite) => {
                self.opposite = Some(opposite);
            }
            Msg::SendToOpposite(title) => {
                self.opposite
                    .as_mut()
                    .unwrap()
                    .send_message(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                let send_msg = match title.as_ref() {
                    "Ping" => Some(Msg::SetTitle("Pong".into())),
                    "Pong" => Some(Msg::SetTitle("Pong Done".into())),
                    "Pong Done" => Some(Msg::SetTitle("Ping Done".into())),
                    _ => None,
                };

                if let Some(send_msg) = send_msg {
                    self.opposite.as_mut().unwrap().send_message(send_msg);
                }

                self.title = title;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
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

fn mount_app(selector: &'static str, app: App<Model>) -> Scope<Model> {
    let document = yew::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    app.mount(element)
}

pub struct TwoModels {}

impl Component for TwoModels {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        TwoModels { }
    }

    fn mounted(&mut self) -> ShouldRender {
        let first_app = App::new();
        let second_app = App::new();
        let to_first = mount_app(".first-app", first_app);
        let to_second = mount_app(".second-app", second_app);
        to_first.send_message(Msg::SetOpposite(to_second.clone()));
        to_second.send_message(Msg::SetOpposite(to_first.clone()));
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="first-app"></div>
                <div class="second-app"></div>
            </div>
        }
    }
}