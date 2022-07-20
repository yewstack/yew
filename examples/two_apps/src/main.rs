use yew::html::Scope;
use yew::{html, AppHandle, Component, Context, Html};

pub enum Msg {
    SetOpposite(Scope<App>),
    SendToOpposite(String),
    SetTitle(String),
}

pub struct App {
    opposite: Option<Scope<App>>,
    selector: &'static str,
    title: String,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            opposite: None,
            selector: "",
            title: "Nothing".to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick={ctx.link().callback(|_| Msg::SendToOpposite("One".into()))}>{ "One" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SendToOpposite("Two".into()))}>{ "Two" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SendToOpposite("Three".into()))}>{ "Three" }</button>
                <button onclick={ctx.link().callback(|_| Msg::SendToOpposite("Ping".into()))}>{ "Ping" }</button>
            </div>
        }
    }
}

fn mount_app(selector: &'static str) -> AppHandle<App> {
    let document = gloo::utils::document();
    let element = document.query_selector(selector).unwrap().unwrap();
    yew::Renderer::<App>::with_root(element).render()
}

fn main() {
    let first_app = mount_app(".first-app");
    let second_app = mount_app(".second-app");

    first_app.send_message(Msg::SetOpposite(second_app.clone()));
    second_app.send_message(Msg::SetOpposite(first_app.clone()));
}
