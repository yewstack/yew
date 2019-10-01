#![recursion_limit = "128"]

use std::time::Duration;
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    timeout: TimeoutService,
    interval: IntervalService,
    console: ConsoleService,
    callback_tick: Callback<()>,
    callback_done: Callback<()>,
    job: Option<Box<Task>>,
    messages: Vec<&'static str>,
    _standalone: Box<Task>,
}

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        // This callback doesn't send any message to a scope
        let callback = |_| {
            println!("Example of a standalone callback.");
        };
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_secs(10), callback.into());

        Model {
            timeout: TimeoutService::new(),
            interval,
            console: ConsoleService::new(),
            callback_tick: link.send_back(|_| Msg::Tick),
            callback_done: link.send_back(|_| Msg::Done),
            job: None,
            messages: Vec::new(),
            _standalone: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                {
                    let handle = self
                        .timeout
                        .spawn(Duration::from_secs(3), self.callback_done.clone());
                    self.job = Some(Box::new(handle));
                }
                self.messages.clear();
                self.console.clear();
                self.messages.push("Timer started!");
                self.console.time_named("Timer");
            }
            Msg::StartInterval => {
                {
                    let handle = self
                        .interval
                        .spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                self.messages.clear();
                self.console.clear();
                self.messages.push("Interval started!");
                self.console.log("Interval started!");
            }
            Msg::Cancel => {
                if let Some(mut task) = self.job.take() {
                    task.cancel();
                }
                self.messages.push("Canceled!");
                self.console.warn("Canceled!");
                self.console.assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Done => {
                self.messages.push("Done!");
                self.console.group();
                self.console.info("Done!");
                self.console.time_named_end("Timer");
                self.console.group_end();
                self.job = None;
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                self.console.count_named("Tick");
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        let view_message = |message| {
            html! { <p>{ message }</p> }
        };
        let has_job = self.job.is_some();
        html! {
            <div>
                <button disabled=has_job onclick=|_| Msg::StartTimeout>{ "Start Timeout" }</button>
                <button disabled=has_job onclick=|_| Msg::StartInterval>{ "Start Interval" }</button>
                <button disabled=!has_job onclick=|_| Msg::Cancel>{ "Cancel!" }</button>
                <div>
                    { for self.messages.iter().map(view_message) }
                </div>
            </div>
        }
    }
}
