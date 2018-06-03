#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

pub mod worker;

use yew::prelude::*;

pub struct Model {
    bridge: Bridge<worker::Worker>,
}

pub enum Msg {
    SendToThread,
    DataReceived,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<Addr<worker::Worker>> + 'static,
{
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<CTX, Self>) -> Self {
        let callback = env.send_back(|_| Msg::DataReceived);
        let bridge = env.as_mut().bridge(callback);
        Model { bridge }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::SendToThread => {
                self.bridge.send(worker::Request::GetDataFromServer);
            }
            Msg::DataReceived => {
                info!("DataReceived");
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<Addr<worker::Worker>> + 'static,
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

