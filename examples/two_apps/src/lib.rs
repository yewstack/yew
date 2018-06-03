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
    scope: Option<Scope<Context, Model>>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SetScope(Scope<Context, Model>),
    SendToOpposite(String),
    SetTitle(String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<Context>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<CTX, Self>) -> Self {
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
                self.scope.as_mut().unwrap().send_message(Msg::SetTitle(title));
            }
            Msg::SetTitle(title) => {
                match title.as_ref() {
                    "Ping" => {
                        self.scope.as_mut().unwrap().send_message(Msg::SetTitle("Pong".into()));
                    }
                    "Pong" => {
                        self.scope.as_mut().unwrap().send_message(Msg::SetTitle("Pong Done".into()));
                    }
                    "Pong Done" => {
                        self.scope.as_mut().unwrap().send_message(Msg::SetTitle("Ping Done".into()));
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
