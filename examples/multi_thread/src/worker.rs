use yew::prelude::*;
use yew::services::fetch::FetchService;

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    RequestDataFromServer,
}

impl Message for Msg {
}

pub struct Worker {
    fetch: FetchService,
}

impl Agent for Worker {
    type Message = Msg;
    type Input = Msg;
    type Output = Msg;

    fn create(link: AgentLink<Self>) -> Self {
        Worker {
            fetch: FetchService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle(&mut self, msg: Self::Input) {
        info!("Incoming: {:?}", msg);
        match msg {
            Msg::RequestDataFromServer => {
                /*
                 * let callback = env.send_back(|_| ___);
                 * let request = ...;
                 * self.fetch.fetch(callback);
                 */
            },
        }
    }
}
