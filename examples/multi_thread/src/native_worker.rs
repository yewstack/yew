use log::info;
use serde_derive::{Deserialize, Serialize};
use std::time::Duration;
use yew::worker::*;
// TODO use yew::services::{IntervalService, FetchService, Task};
use yew::services::fetch::FetchService;
use yew::services::interval::IntervalService;
use yew::services::Task;

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
    interval: IntervalService,
    task: Box<Task>,
    fetch: FetchService,
}

impl Agent for Worker {
    type Reach = Public;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let duration = Duration::from_secs(3);
        let callback = link.send_back(|_| Msg::Updating);
        let task = interval.spawn(duration, callback);
        Worker {
            link,
            interval,
            task: Box::new(task),
            fetch: FetchService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Updating => {
                info!("Tick...");
            }
        }
    }

    fn handle(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
                self.link.response(who, Response::DataFetched);
            }
        }
    }

    fn name_of_resource() -> &'static str {
        "bin/native_worker.js"
    }
}
