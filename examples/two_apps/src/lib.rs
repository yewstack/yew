/// This example demonstrates low-level usage of scopes.

#[macro_use]
extern crate yew;

use yew::html::*;

#[derive(Default)]
pub struct Context {
}

impl Context {
    pub fn new() -> Self {
        Context {
        }
    }
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}

pub struct Model {
    activator: Option<Activator<Context, Model>>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SetActivator(Activator<Context, Model>),
    SendToOpposite(String),
    SetTitle(String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<Context>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            activator: None,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::SetActivator(activator) => {
                self.activator = Some(activator);
            }
            Msg::SendToOpposite(title) => {
                self.activator.as_mut().unwrap().send_message(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                match title.as_ref() {
                    "Ping" => {
                        self.activator.as_mut().unwrap().send_message(Msg::SetTitle("Pong".into()));
                    }
                    "Pong" => {
                        self.activator.as_mut().unwrap().send_message(Msg::SetTitle("Pong Done".into()));
                    }
                    "Pong Done" => {
                        self.activator.as_mut().unwrap().send_message(Msg::SetTitle("Ping Done".into()));
                    }
                    _ => {
                    }
                }
                self.title = title;
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<Context> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <h3>{ format!("{} received <{}>", self.selector, self.title) }</h3>
                <button onclick=|_| Msg::SendToOpposite("One".into()),>{ "One" }</button>
                <button onclick=|_| Msg::SendToOpposite("Two".into()),>{ "Two" }</button>
                <button onclick=|_| Msg::SendToOpposite("Three".into()),>{ "Three" }</button>
                <button onclick=|_| Msg::SendToOpposite("Ping".into()),>{ "Ping" }</button>
            </div>
        }
    }
}
