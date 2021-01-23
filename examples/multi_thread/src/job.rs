use serde::{Deserialize, Serialize};
use std::time::Duration;
use yew::worker::{Agent, AgentLink, HandlerId, Job};
use yew_services::interval::{IntervalService, IntervalTask};

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
    _task: IntervalTask,
}

impl Agent for Worker {
    type Reach = Job<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        let duration = Duration::from_secs(3);
        let callback = link.callback(|_| Msg::Updating);

        link.send_message(Msg::Initialized);
        Self {
            link,
            _task: IntervalService::spawn(duration, callback),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Initialized => {
                log::info!("Initialized!");
            }
            Msg::Updating => {
                log::info!("Tick...");
            }
            Msg::DataFetched => {
                log::info!("Data was fetched");
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        log::info!("Request: {:?}", msg);
        match msg {
            Request::GetDataFromServer => {
                // TODO fetch actual data
                self.link.respond(who, Response::DataFetched);
                self.link.send_message(Msg::DataFetched);
            }
        }
    }
}
