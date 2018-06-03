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

impl<CTX: 'static> Component<CTX> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<CTX, Self>, _: &mut CTX) -> Self {
        let callback = link.send_back(|_| Msg::DataReceived);
        let mut addr = worker::Worker::spawn();
        let bridge = addr.bridge(callback);
        Model { bridge }
    }

    fn update(&mut self, msg: Self::Message, _: &mut CTX) -> ShouldRender {
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

impl<CTX: 'static> Renderable<CTX, Model> for Model {
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

