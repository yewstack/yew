use log::info;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::services::interval::IntervalService;
use yew::services::Task;
use yew::worker::*;

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
    link: AgentLink<Worker>,
    _task: Box<dyn Task>,
}

impl Agent for Worker {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_secs(3);
        let callback = link.callback(|_| Msg::Updating);
        let task = IntervalService::new().spawn(duration, callback);
        Worker {
            link,
            _task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                info!("Tick...");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
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
