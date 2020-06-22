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
    Initialized,
    Updating,
    DataFetched,
}

pub struct Worker {
    link: AgentLink<Worker>,
    _task: Box<dyn Task>,
}

impl Agent for Worker {
    type Reach = Job<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_secs(3);
        let callback = link.callback(|_| Msg::Updating);
        let task = IntervalService::spawn(duration, callback);

        link.send_message(Msg::Initialized);
        Worker {
            link,
            _task: Box::new(task),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Initialized => {
                info!("Initialized!");
            }
            Msg::Updating => {
                info!("Tick...");
            }
            Msg::DataFetched => {
                info!("Data was fetched");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
                // TODO fetch actual data
                self.link.respond(who, Response::DataFetched);
                self.link.send_message(Msg::DataFetched);
            }
        }
    }
}
