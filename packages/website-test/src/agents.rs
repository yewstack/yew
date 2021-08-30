//! Agent types that compile to be used by website code snippets

use yew_agent::{Agent, AgentLink, Context, HandlerId};

pub struct EventBus;

impl Agent for EventBus {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ();
    type Output = String;

    fn create(_link: AgentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) {
        // impl
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: HandlerId) {
        // impl
    }
}

pub enum WorkerMsg {
    Process,
}

pub struct MyWorker;

impl Agent for MyWorker {
    type Reach = Context<Self>;

    type Message = ();
    type Input = WorkerMsg;
    type Output = ();

    fn create(_link: AgentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) {
        // impl
    }

    fn handle_input(&mut self, _msg: Self::Input, _id: HandlerId) {
        // impl
    }
}
