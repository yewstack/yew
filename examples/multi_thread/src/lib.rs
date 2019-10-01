#![recursion_limit = "128"]

pub mod context;
pub mod job;
pub mod native_worker;

use log::info;
use yew::worker::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    worker: Box<Bridge<native_worker::Worker>>,
    job: Box<Bridge<job::Worker>>,
    context: Box<Bridge<context::Worker>>,
    context_2: Box<Bridge<context::Worker>>,
}

pub enum Msg {
    SendToWorker,
    SendToJob,
    SendToContext,
    DataReceived,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::DataReceived);
        let worker = native_worker::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Model {
            worker,
            job,
            context,
            context_2,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWorker => {
                self.worker.send(native_worker::Request::GetDataFromServer);
            }
            Msg::SendToJob => {
                self.job.send(job::Request::GetDataFromServer);
            }
            Msg::SendToContext => {
                self.context.send(context::Request::GetDataFromServer);
                self.context_2.send(context::Request::GetDataFromServer);
            }
            Msg::DataReceived => {
                info!("DataReceived");
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=|_| Msg::SendToWorker>{ "Send to Thread" }</button>
                    <button onclick=|_| Msg::SendToJob>{ "Send to Job" }</button>
                    <button onclick=|_| Msg::SendToContext>{ "Send to Context" }</button>
                </nav>
            </div>
        }
    }
}
