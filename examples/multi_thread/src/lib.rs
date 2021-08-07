pub mod context;
pub mod job;
pub mod native_worker;

use yew::{html, Component, Context, Html};
use yew_agent::{Bridge, Bridged};

pub enum Msg {
    SendToWorker,
    SendToJob,
    SendToContext,
    DataReceived,
}

pub struct Model {
    worker: Box<dyn Bridge<native_worker::Worker>>,
    job: Box<dyn Bridge<job::Worker>>,
    context: Box<dyn Bridge<context::Worker>>,
    context_2: Box<dyn Bridge<context::Worker>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let callback = link.callback(|_| Msg::DataReceived);
        let worker = native_worker::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let job = job::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context = context::Worker::bridge(callback);

        let callback = link.callback(|_| Msg::DataReceived);
        let context_2 = context::Worker::bridge(callback);

        Self {
            worker,
            job,
            context,
            context_2,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SendToWorker => {
                self.worker.send(native_worker::Request::GetDataFromServer);
                false
            }
            Msg::SendToJob => {
                self.job.send(job::Request::GetDataFromServer);
                false
            }
            Msg::SendToContext => {
                self.context.send(context::Request::GetDataFromServer);
                self.context_2.send(context::Request::GetDataFromServer);
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
                    <button on:click={ctx.link().callback(|_| Msg::SendToWorker)}>{ "Send to Thread" }</button>
                    <button on:click={ctx.link().callback(|_| Msg::SendToJob)}>{ "Send to Job" }</button>
                    <button on:click={ctx.link().callback(|_| Msg::SendToContext)}>{ "Send to Context" }</button>
                </nav>
            </div>
        }
    }
}
