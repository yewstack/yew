#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

pub mod worker;

use yew::prelude::*;

pub struct Model {
}

pub enum Msg {
    SendToThread,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsRef<Addr<worker::Worker>>,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::SendToThread => {
                env.as_ref().send(worker::Request::GetDataFromServer);
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsRef<Addr<worker::Worker>> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::SendToThread,>{ "SendToThread" }</button>
                </nav>
            </div>
        }
    }
}

