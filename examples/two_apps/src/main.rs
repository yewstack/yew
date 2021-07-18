use yew::{html, AppHandle, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    SetOpposite(ComponentLink<Model>),
    SendToOpposite(String),
    SetTitle(String),
}

pub struct Model {
    link: ComponentLink<Self>,
    opposite: Option<ComponentLink<Model>>,
    selector: &'static str,
    title: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            opposite: None,
            selector: "",
            title: "Nothing".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetOpposite(opposite) => {
                self.opposite = Some(opposite);
                false
            }
            Msg::SendToOpposite(title) => {
                self.opposite
                    .as_mut()
                    .unwrap()
                    .send_message(Msg::SetTitle(title));
                false
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
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick={self.link.callback(|_| Msg::SendToOpposite("One".into()))}>{ "One" }</button>
                <button onclick={self.link.callback(|_| Msg::SendToOpposite("Two".into()))}>{ "Two" }</button>
                <button onclick={self.link.callback(|_| Msg::SendToOpposite("Three".into()))}>{ "Three" }</button>
                <button onclick={self.link.callback(|_| Msg::SendToOpposite("Ping".into()))}>{ "Ping" }</button>
            </div>
        }
    }
}

fn mount_app(selector: &'static str) -> AppHandle<Model> {
    let document = yew::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    yew::start_app_in_element(element)
}

fn main() {
    let first_app = mount_app(".first-app");
    let second_app = mount_app(".second-app");

    first_app.send_message(Msg::SetOpposite(second_app.clone()));
    second_app.send_message(Msg::SetOpposite(first_app.clone()));
}
