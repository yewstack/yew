use yew::agent::{Agent, Message};
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
    type Input = Msg;
    type Output = Msg;

    fn create() -> Self {
        Worker {
            fetch: FetchService::new(),
        }
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
