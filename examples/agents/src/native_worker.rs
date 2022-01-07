use gloo_timers::callback::Interval;
use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, WorkerLink};

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    DataFetched,
}

pub enum Msg {
    Updating,
}

pub struct Worker {
    link: WorkerLink<Worker>,
    _interval: Interval,
}

impl yew_agent::Worker for Worker {
    type Reach = Public<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: WorkerLink<Self>) -> Self {
        let duration = 3;

        let interval = {
            let link = link.clone();
            Interval::new(duration, move || link.send_message(Msg::Updating))
        };
        Self {
            link,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                log::info!("Tick...");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
                // TODO fetch actual data
                self.link.respond(who, Response::DataFetched);
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }
}
