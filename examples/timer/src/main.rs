#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::html::*;
use yew::services::Task;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;
use yew::services::console::{ConsoleService};

struct Model {
    job: Option<Box<Task>>,
    messages: Vec<&'static str>,
}

enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
}

fn update(context: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    let console = context.get_console();
    match msg {
        Msg::StartTimeout => {
            let handle = context.timeout(Duration::from_secs(3), || Msg::Done);
            model.job = Some(Box::new(handle));
            model.messages.clear();
            console.clear();
            model.messages.push("Timer started!!");
            console.time_named("Timer");
        }
        Msg::StartInterval => {
            let handle = context.interval(Duration::from_secs(1), || Msg::Tick);
            model.job = Some(Box::new(handle));
            model.messages.clear();
            console.clear();
            model.messages.push("Interval started!");
            console.log("Interval started!");
        }
        Msg::Cancel => {
            if let Some(mut task) = model.job.take() {
                task.cancel();
            }
            model.messages.push("Canceled!");
            console.warn("Canceled!");
            console.trace();
            console.assert(model.job.is_none(), "Job still exists!");
        }
        Msg::Done => {
            model.messages.push("Done!");
            console.group();
            console.info("Done!");
            console.time_named_end("Timer");
            console.group_end();
            model.job = None;
        }
        Msg::Tick => {
            model.messages.push("Tick...");
            console.count_named("Tick");
        }
    }
}

fn view(model: &Model) -> Html<Msg> {
    let view_message = |message| {
        html! { <p>{ message }</p> }
    };
    let has_job = model.job.is_some();
    html! {
        <div>
            <button disabled= has_job, onclick=|_| Msg::StartTimeout,>{ "Start Timeout" }</button>
            <button disabled= has_job, onclick=|_| Msg::StartInterval,>{ "Start Interval" }</button>
            <button disabled=!has_job, onclick=|_| Msg::Cancel,>{ "Cancel!" }</button>
            <div>
                { for model.messages.iter().map(view_message) }
            </div>
        </div>
    }
}

fn main() {
    let model = Model {
        job: None,
        messages: Vec::new(),
    };
    program(model, update, view);
}
