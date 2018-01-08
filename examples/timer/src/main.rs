#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::html::*;
use yew::services::Task;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;
use yew::services::console::ConsoleService;

struct Context {
    interval: IntervalService,
    timeout: TimeoutService,
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

impl Component<Context> for Model {
    type Msg = Msg;

    fn create(_: &mut ScopeRef<Context, Msg>) -> Self {
        Model {
            job: None,
            messages: Vec::new(),
        }
    }

    fn update(&mut self, msg: Msg, context: &mut ScopeRef<Context, Msg>) {
        match msg {
            Msg::StartTimeout => {
                let callback = context.send_back(|_| Msg::Done);
                let handle = context.timeout.spawn(Duration::from_secs(3), callback);
                self.job = Some(Box::new(handle));
                self.messages.clear();
                context.console.clear();
                self.messages.push("Timer started!");
                context.console.time_named("Timer");
            }
            Msg::StartInterval => {
                let callback = context.send_back(|_| Msg::Tick);
                let handle = context.interval.spawn(Duration::from_secs(1), callback);
                self.job = Some(Box::new(handle));
                self.messages.clear();
                context.console.clear();
                self.messages.push("Interval started!");
                context.console.log("Interval started!");
            }
            Msg::Cancel => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.messages.push("Canceled!");
                context.console.warn("Canceled!");
                context.console.assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Done => {
                self.messages.push("Done!");
                context.console.group();
                context.console.info("Done!");
                context.console.time_named_end("Timer");
                context.console.group_end();
                self.job = None;
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                context.console.count_named("Tick");
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        let view_message = |message| {
            html! { <p>{ message }</p> }
        };
        let has_job = self.job.is_some();
        html! {
            <div>
                <button disabled= has_job, onclick=|_| Msg::StartTimeout,>{ "Start Timeout" }</button>
                <button disabled= has_job, onclick=|_| Msg::StartInterval,>{ "Start Interval" }</button>
                <button disabled=!has_job, onclick=|_| Msg::Cancel,>{ "Cancel!" }</button>
                <div>
                    { for self.messages.iter().map(view_message) }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        interval: IntervalService::new(),
        timeout: TimeoutService::new(),
        console: ConsoleService,
    };
    let app = Scope::new(context);
    app.mount_to_body::<Model>();
    yew::run_loop();
}
