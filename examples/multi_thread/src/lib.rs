#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

pub mod thread_worker;
pub mod shared_worker;
pub mod job;
pub mod context;

use yew::prelude::*;

pub struct Model {
    thread_worker: Box<Bridge<thread_worker::Worker>>,
    shared_worker: Box<Bridge<shared_worker::Worker>>,
    job: Box<Bridge<job::Worker>>,
    context: Box<Bridge<context::Worker>>,
    context_2: Box<Bridge<context::Worker>>,
}

pub enum Msg {
    SendToThread,
    SendToSharedThread,
    SendToJob,
    SendToContext,
    DataReceived,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::DataReceived);
        let thread_worker = thread_worker::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let shared_worker = shared_worker::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Model { thread_worker, shared_worker, job, context, context_2 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToThread => {
                self.thread_worker.send(thread_worker::Request::GetDataFromServer);
            }
            Msg::SendToSharedThread => {
                self.shared_worker.send(shared_worker::Request::GetDataFromServer);
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
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::SendToThread,>{ "Send to Thread" }</button>
                    <button onclick=|_| Msg::SendToSharedThread,>{ "Send to Shared Thread" }</button>
                    <button onclick=|_| Msg::SendToJob,>{ "Send to Job" }</button>
                    <button onclick=|_| Msg::SendToContext,>{ "Send to Context" }</button>
                </nav>
            </div>
        }
    }
}

