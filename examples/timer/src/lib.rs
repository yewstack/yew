#[macro_use]
extern crate yew;

use std::time::Duration;
use yew::prelude::*;
use yew::services::Task;
use yew::services::timeout::TimeoutService;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::console::ConsoleService;

pub struct Model {
    job: Option<Box<Task>>,
    messages: Vec<&'static str>,
    _standalone: IntervalTask,
}

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
}

impl<CTX> Component<CTX> for Model
where
    CTX: AsMut<IntervalService> + AsMut<TimeoutService> + AsMut<ConsoleService> + 'static,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<CTX, Self>) -> Self {
        // This callback doesn't send any message to a scope
        let callback = |_| {
            println!("Example of a standalone callback.");
        };
        let mut context = env.context();
        let interval: &mut IntervalService = context.as_mut();
        let handle = interval.spawn(Duration::from_secs(10), callback.into());

        Model {
            job: None,
            messages: Vec::new(),
            _standalone: handle,
        }
    }

    fn update(&mut self, msg: Self::Msg, env: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                {
                    let callback = env.send_back(|_| Msg::Done);
                    let mut context = env.context();
                    let timeout: &mut TimeoutService = context.as_mut();
                    let handle = timeout.spawn(Duration::from_secs(3), callback);
                    self.job = Some(Box::new(handle));
                }
                let mut context = env.context();
                let console: &mut ConsoleService = context.as_mut();
                self.messages.clear();
                console.clear();
                self.messages.push("Timer started!");
                console.time_named("Timer");
            }
            Msg::StartInterval => {
                {
                    let callback = env.send_back(|_| Msg::Tick);
                    let mut context = env.context();
                    let interval: &mut IntervalService = context.as_mut();
                    let handle = interval.spawn(Duration::from_secs(1), callback);
                    self.job = Some(Box::new(handle));
                }
                let mut context = env.context();
                let console: &mut ConsoleService = context.as_mut();
                self.messages.clear();
                console.clear();
                self.messages.push("Interval started!");
                console.log("Interval started!");
            }
            Msg::Cancel => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.messages.push("Canceled!");
                let mut context = env.context();
                let console: &mut ConsoleService = context.as_mut();
                console.warn("Canceled!");
                console.assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Done => {
                self.messages.push("Done!");
                let mut context = env.context();
                let console: &mut ConsoleService = context.as_mut();
                console.group();
                console.info("Done!");
                console.time_named_end("Timer");
                console.group_end();
                self.job = None;
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                let mut context = env.context();
                let console: &mut ConsoleService = context.as_mut();
                console.count_named("Tick");
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model
where
    CTX: AsMut<IntervalService> + AsMut<TimeoutService> + AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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
