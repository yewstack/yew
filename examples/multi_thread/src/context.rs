use gloo::timers::callback::Interval;
use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, Context, HandlerId};

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
    _interval: Interval,
}

impl Agent for Worker {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
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
}
