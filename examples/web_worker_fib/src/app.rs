use yew::prelude::*;
use yew_agent::{
    Agent, AgentLink, HandlerId, Public, Bridge, Bridged
};
use yew::web_sys::HtmlInputElement;
use serde::{Serialize, Deserialize};


pub(crate) struct Model {
    clicker_value: u32,
    n_ref: NodeRef,
    worker: Box<dyn Bridge<Worker>>,
    fibonacci_output: String,
}

pub(crate) enum Message {
    Click,
    RunWorker,
    WorkerMessage(WorkerOutput),
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let worker = Worker::bridge(
            ctx.link().callback(|worker_output| Self::Message::WorkerMessage(worker_output))
        );

        Self {
            clicker_value: 0,
            n_ref: NodeRef::default(),
            worker,
            fibonacci_output: String::from("Try out some fibonacci calculations!"),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Click => {
                self.clicker_value += 1;
            }
            Self::Message::RunWorker => {
                if let Some(input) = self.n_ref.cast::<HtmlInputElement>() {
                    if let Ok(value) = input.value().parse::<u32>() {
                        // start the worker off!
                        self.worker.send(WorkerInput {
                            n: value,
                        });
                    }
                }
            }
            Self::Message::WorkerMessage(output) => {
                // the worker is done!
                self.fibonacci_output = format!("Fibonacci value: {}", output.value);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1> { "Web worker demo" } </h1>
                <h3> { "Output: " } { &self.fibonacci_output } </h3>
                <br />
                <input ref={self.n_ref.clone()} type="number" />
                <button onclick={ctx.link().callback(|_| Message::RunWorker)}> { "submit" } </button>
                <br /> <br />
                <h3> { "Main thread value: " } { self.clicker_value } </h3>
                <button onclick={ctx.link().callback(|_| Message::Click)}> { "click!" } </button>
            </>
        }
    }
}

pub(crate) struct Worker {
    link: AgentLink<Self>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WorkerInput {
    n: u32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct WorkerOutput {
    value: u32,
}


impl Agent for Worker {
    type Reach = Public<Self>;
    type Message = ();
    type Input = WorkerInput;
    type Output = WorkerOutput;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
        }
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
            if n <= 1 { 1 } else { fib(n - 1) + fib(n - 2) }
        }

        let output = Self::Output {
            value: fib(n),
        };

        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "wasm.js"
    }
}
