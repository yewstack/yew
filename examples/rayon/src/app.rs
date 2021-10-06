use std::sync::Mutex;

use once_cell::sync::Lazy;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::worker::{Request, Response, Worker};

pub(crate) static STATE: Lazy<Mutex<State>> = Lazy::new(|| {
    Mutex::new(State {
        n: 0,
        computed_sum: None,
    })
});

// must be thread-safe but doesn't need to be serializable or cloneable
pub(crate) struct State {
    pub(crate) n: u32,
    pub(crate) computed_sum: Option<u32>,
}

pub(crate) struct Model {
    worker: Box<dyn Bridge<Worker>>,
}

pub(crate) enum Message {
    Click,
    RunWorker,
    WorkerMsg(Response),
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let worker = Worker::bridge(ctx.link().callback(Self::Message::WorkerMsg));

        Self { worker }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Click => {
                let mut s = STATE.lock().unwrap();
                s.n += 1;
                s.computed_sum = None;
                log::debug!("Main thread: updated n");
                true
            }
            Self::Message::RunWorker => {
                self.worker.send(Request::Compute);
                log::debug!("Main thread: sent request to worker");
                false
            }
            Self::Message::WorkerMsg(Response::Acknowledged) => {
                log::debug!("Main thread: waiting for worker to finish");
                false
            }
            Self::Message::WorkerMsg(Response::Finished) => {
                log::debug!("Main thread: worker finished");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            <h1> { "Rayon demo" } </h1>
            <h3>
                { "n: " } { STATE.lock().unwrap().n }
                <button onclick={ctx.link().callback(|_| Message::Click)}> { "+1" } </button>
            </h3>
            <h3>
                { "Computed sum from 1 to n: " }
                { STATE.lock().unwrap().computed_sum.map(|s| s.to_string()).unwrap_or_else(|| "…".into()) }
                <button onclick={ctx.link().callback(|_| Message::RunWorker)}> { "Compute" } </button>
            </h3>
        </>
        }
    }
}
