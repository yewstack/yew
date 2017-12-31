#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::html::*;
use yew::services::Task;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;
use yew::services::console::ConsoleService;

struct Context {
    interval: IntervalService<Msg>,
    timeout: TimeoutService<Msg>,
    console: ConsoleService,
}

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

fn update(context: &mut Context, model: &mut Model, msg: Msg) {
    match msg {
        Msg::StartTimeout => {
            let handle = context.timeout.spawn(Duration::from_secs(3), || Msg::Done);
            model.job = Some(Box::new(handle));
            model.messages.clear();
            context.console.clear();
            model.messages.push("Timer started!!");
            context.console.time_named("Timer");
        }
        Msg::StartInterval => {
            let handle = context.interval.spawn(Duration::from_secs(1), || Msg::Tick);
            model.job = Some(Box::new(handle));
            model.messages.clear();
            context.console.clear();
            model.messages.push("Interval started!");
            context.console.log("Interval started!");
        }
        Msg::Cancel => {
            if let Some(mut task) = model.job.take() {
                task.cancel();
            }
            model.messages.push("Canceled!");
            context.console.warn("Canceled!");
            context.console.assert(model.job.is_none(), "Job still exists!");
        }
        Msg::Done => {
            model.messages.push("Done!");
            context.console.group();
            context.console.info("Done!");
            context.console.time_named_end("Timer");
            context.console.group_end();
            model.job = None;
        }
        Msg::Tick => {
            model.messages.push("Tick...");
            context.console.count_named("Tick");
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
    let mut app = App::new();
    let context = Context {
        interval: IntervalService::new(app.sender()),
        timeout: TimeoutService::new(app.sender()),
        console: ConsoleService,
    };
    let model = Model {
        job: None,
        messages: Vec::new(),
    };
    app.run(context, model, update, view);
}
