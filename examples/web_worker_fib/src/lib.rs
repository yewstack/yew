#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]

pub mod agent;

use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::agent::{Worker, WorkerInput, WorkerOutput};

pub struct App {
    clicker_value: u32,
    input_ref: NodeRef,
    worker: Box<dyn Bridge<Worker>>,
    fibonacci_output: String,
}

pub enum Message {
    Click,
    RunWorker,
    WorkerMsg(WorkerOutput),
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let cb = {
            let link = ctx.link().clone();
            move |e| link.send_message(Self::Message::WorkerMsg(e))
        };
        let worker = Worker::bridge(Rc::new(cb));

        Self {
            clicker_value: 0,
            input_ref: NodeRef::default(),
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
                if let Some(input) = self.input_ref.cast::<HtmlInputElement>() {
                    // start the worker off!
                    self.worker.send(WorkerInput {
                        n: input.value_as_number() as u32,
                    });
                }
            }
            Self::Message::WorkerMsg(output) => {
                // the worker is done!
                self.fibonacci_output = format!("Fibonacci value: {}", output.value);
            }
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{ "Web worker demo" }</h1>
                <p>{ "Submit a value to calculate, then increase the counter on the main thread!"} </p>
                <p>{ "Large numbers will take some time!" }</p>
                <h3>{ "Output: " } { &self.fibonacci_output }</h3>
                <br />
                <input ref={self.input_ref.clone()} type="number" value="44" max="50"/>
                <button onclick={ctx.link().callback(|_| Message::RunWorker)}>{ "submit" }</button>
                <br /> <br />
                <h3>{ "Main thread value: " } { self.clicker_value }</h3>
                <button onclick={ctx.link().callback(|_| Message::Click)}>{ "click!" }</button>
            </>
        }
    }
}
