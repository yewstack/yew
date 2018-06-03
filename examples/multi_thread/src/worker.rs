use std::time::Duration;
use yew::prelude::*;
// TODO use yew::services::{IntervalService, FetchService, Task};
use yew::services::Task;
use yew::services::interval::IntervalService;
use yew::services::fetch::FetchService;

#[derive(Serialize, Deserialize, Debug)]
pub enum Request {
    GetDataFromServer,
}

impl Message for Request {
}

pub enum Msg {
    Updating,
}

pub struct Worker {
    interval: IntervalService,
    task: Box<Task>,
    fetch: FetchService,
}

impl Agent for Worker {
    type Message = Msg;
    type Input = Request;
    type Output = Msg;

    fn create(link: AgentLink<Self>) -> Self {
        let mut interval = IntervalService::new();
        let duration = Duration::from_secs(3);
        let callback = link.send_back(|_| Msg::Updating);
        let task = interval.spawn(duration, callback);
        Worker {
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

    fn handle(&mut self, msg: Self::Input) {
        info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
            }
        }
    }
}
