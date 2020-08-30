mod agents;

use yew::prelude::*;
//use yew::agent::{Bridgeable, Dispatcher, Dispatched, ReadOnly, StoreWrapper};
use web_sys::console;
use yewtil::store::{Bridgeable, ReadOnly, StoreWrapper};

use crate::agents::media_manager::{MediaManager, Request};

pub struct App {
    link: ComponentLink<Self>,
    media_manager: Box<dyn Bridge<StoreWrapper<MediaManager>>>,
}

pub enum Msg {
    GetStream,
    GetDevices,
    MediaManagerMsg(ReadOnly<MediaManager>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::MediaManagerMsg);
        let media_manager = MediaManager::bridge(callback);
        Self {
            link,
            media_manager,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetStream => {
                self.media_manager.send(Request::GetStream);
                console::log_1(&"after send".into());
            }
            Msg::GetDevices => self.media_manager.send(Request::GetDevices),
            Msg::MediaManagerMsg(state) => {
                if let Some(stream) = &state.borrow().media_stream {
                    console::log_2(&"We have a stream".into(), &stream);
                }

                // We can see this is logged once before we click any button.
                // The state of the store is sent when we open a bridge.
                console::log_1(&"Received update".into());
            }
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick=self.link.callback(|_| Msg::GetStream)>
                    { "get stream" }
                </button>

                <button onclick=self.link.callback(|_| Msg::GetDevices)>
                    { "get devices" }
                </button>
            </div>
        }
    }
}
