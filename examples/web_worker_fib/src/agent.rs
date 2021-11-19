use yew_agent::{Agent, AgentLink, HandlerId, Public};
use serde::{Deserialize, Serialize};

pub struct Worker {
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInput {
    pub n: u32,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOutput {
    pub value: u32,
}

impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = ();
    type Input = WorkerInput;
    type Output = WorkerOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        let n = msg.n;

        fn fib(n: u32) -> u32 {
            if n <= 1 {
                1
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }

        let output = Self::Output { value: fib(n) };

        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "wasm.js"
    }
}
