use yew::agent::{Agent, Message};

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {
    First,
}

impl Message for Msg {
}

pub struct Worker {
}

impl Agent for Worker {
    type Input = Msg;
    type Output = Msg;

    fn create() -> Self {
        Worker { }
    }

    fn handle(&mut self, msg: Self::Input) {
        info!("Incoming: {:?}", msg);
    }
}
