/// This example demonstrates low-level usage of scopes.

#[macro_use]
extern crate yew;

use yew::html::*;

pub struct Context {
    pub senders: Vec<ScopeSender<Context, Model>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            senders: Vec::new(),
        }
    }
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}

pub struct Model {
    sender: ScopeSender<Context, Model>,
    selector: &'static str,
    title: String,
}

pub enum Msg {
    SendToOpposite(String),
    SetTitle(String),
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<Context>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<CTX, Self>) -> Self {
        let sender = context.as_mut().senders.pop().unwrap();
        Model {
            // TODO Use properties to set sender...
            sender,
            selector: "",
            title: "Nothing".into(),
        }
    }

    fn update(&mut self, msg: Msg, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::SendToOpposite(title) => {
                self.sender.send(ComponentUpdate::Message(Msg::SetTitle(title)));
            }
            Msg::SetTitle(title) => {
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
            </div>
        }
    }
}
