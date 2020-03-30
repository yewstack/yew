use futures::future::ready;
use futures_signals::signal::SignalExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::store::{ArcState, Store, StoreOutput};

pub struct Subscriber {
    ip: Option<String>,
    link: ComponentLink<Subscriber>,
    store: Box<dyn Bridge<Store>>,
    state_ref: Option<ArcState>,
    id: i32,
}

pub enum Msg {
    FromStore(StoreOutput),
    SetIp(Option<String>),
}

impl Subscriber {
    fn register_state_handlers(&self) {
        let callback = self.link.callback(|ip| Msg::SetIp(ip));
        let state = self.state_ref.as_ref().unwrap();
        let handler = state.ip.signal_cloned().for_each(move |u| {
            callback.emit(u);
            ready(())
        });
        spawn_local(handler);
    }
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32,
}

impl Component for Subscriber {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let store = Store::bridge(link.callback(|d| Msg::FromStore(d)));
        Self {
            ip: None,
            link,
            store,
            state_ref: None,
            id: props.id,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FromStore(s) => match s {
                StoreOutput::StateInstance(state) => {
                    self.state_ref = Some(state);
                    self.register_state_handlers();
                }
            },
            Msg::SetIp(ip) => self.ip = ip,
        }
        true
    }

    fn view(&self) -> Html {
        let ip = if self.ip.is_some() {
            self.ip.as_ref().unwrap()
        } else {
            "No IP yet. It will show up here"
        };
        html! {
          <div class="subscriber-container">
            <h4 class="subscriber">{{"I'm subscriber "}}{{ self.id }}</h4>
            {{"IP: "}}{{ ip }}
          </div>
        }
    }
}
