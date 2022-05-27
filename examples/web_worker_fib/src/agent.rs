use serde::{Deserialize, Serialize};
use yew_agent::{HandlerId, Public, WorkerLink};

pub struct Worker {
    link: WorkerLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerInput {
    pub n: u32,
}

#[derive(Serialize, Deserialize)]
pub struct WorkerOutput {
    pub value: u32,
}

impl yew_agent::Worker for Worker {
    type Input = WorkerInput;
    type Message = ();
    type Output = WorkerOutput;
    type Reach = Public<Self>;

    fn create(link: WorkerLink<Self>) -> Self {
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
        "worker.js"
    }
}
