#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate yew;

pub mod worker;
pub mod job;

use yew::prelude::*;

pub struct Model {
    worker: Box<Bridge<worker::Worker>>,
    job: Box<Bridge<job::Worker>>,
}

pub enum Msg {
    SendToWorker,
    SendToJob,
    DataReceived,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|_| Msg::DataReceived);
        let mut addr = worker::Worker::spawn();
        let worker = addr.bridge(callback);

        let callback = link.send_back(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        Model { worker, job }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendToWorker => {
                self.worker.send(worker::Request::GetDataFromServer);
            }
            Msg::SendToJob => {
                self.job.send(job::Request::GetDataFromServer);
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
                    <button onclick=|_| Msg::SendToWorker,>{ "Send to Thread" }</button>
                    <button onclick=|_| Msg::SendToJob,>{ "Send to Job" }</button>
                </nav>
            </div>
        }
    }
}

