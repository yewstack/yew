#![recursion_limit = "128"]

pub mod context;
pub mod job;
pub mod native_worker;

use log::info;
use yew::worker::{Bridge, Bridged};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    worker: Box<dyn Bridge<native_worker::Worker>>,
    job: Box<dyn Bridge<job::Worker>>,
    context: Box<dyn Bridge<context::Worker>>,
    context_2: Box<dyn Bridge<context::Worker>>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::DataReceived);
        let worker = native_worker::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Model {
            link,
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

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::SendToWorker)>{ "Send to Thread" }</button>
                    <button onclick=self.link.callback(|_| Msg::SendToJob)>{ "Send to Job" }</button>
                    <button onclick=self.link.callback(|_| Msg::SendToContext)>{ "Send to Context" }</button>
                </nav>
            </div>
        }
    }
}
