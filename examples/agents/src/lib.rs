pub mod native_worker;

use std::rc::Rc;
use yew::{html, Component, Context, Html};
use yew_agent::{Bridge, Bridged};

pub enum Msg {
    SendToWorker,
    DataReceived,
}

pub struct App {
    worker: Box<dyn Bridge<native_worker::Worker>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let callback = move |_| link.send_message(Msg::DataReceived);
        let worker = native_worker::Worker::bridge(Rc::new(callback));

        Self { worker }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendToWorker => {
                self.worker.send(native_worker::Request::GetDataFromServer);
                false
            }
            Msg::DataReceived => {
                log::info!("DataReceived");
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick={ctx.link().callback(|_| Msg::SendToWorker)}>{ "Send to Thread" }</button>
                </nav>
            </div>
        }
    }
}
