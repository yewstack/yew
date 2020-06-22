#![recursion_limit = "128"]

use std::time::Duration;
use yew::services::{ConsoleService, IntervalService, Task, TimeoutService};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    callback_tick: Callback<()>,
    callback_done: Callback<()>,
    job: Option<Box<dyn Task>>,
    messages: Vec<&'static str>,
    _standalone: Box<dyn Task>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        // This callback doesn't send any message to a scope
        let callback = |_| {
            println!("Example of a standalone callback.");
        };
        let handle = IntervalService::spawn(Duration::from_secs(10), callback.into());

        Model {
            link: link.clone(),
            callback_tick: link.callback(|_| Msg::Tick),
            callback_done: link.callback(|_| Msg::Done),
            job: None,
            messages: Vec::new(),
            _standalone: Box::new(handle),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                {
                    let handle =
                        TimeoutService::spawn(Duration::from_secs(3), self.callback_done.clone());
                    self.job = Some(Box::new(handle));
                }
                self.messages.clear();
                ConsoleService::clear();
                self.messages.push("Timer started!");
                ConsoleService::time_named("Timer");
            }
            Msg::StartInterval => {
                {
                    let handle =
                        IntervalService::spawn(Duration::from_secs(1), self.callback_tick.clone());
                    self.job = Some(Box::new(handle));
                }
                self.messages.clear();
                ConsoleService::clear();
                self.messages.push("Interval started!");
                ConsoleService::log("Interval started!");
            }
            Msg::Cancel => {
                self.job.take();
                self.messages.push("Canceled!");
                ConsoleService::warn("Canceled!");
                ConsoleService::assert(self.job.is_none(), "Job still exists!");
            }
            Msg::Done => {
                self.messages.push("Done!");
                ConsoleService::group();
                ConsoleService::info("Done!");
                ConsoleService::time_named_end("Timer");
                ConsoleService::group_end();
                self.job = None;
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                ConsoleService::count_named("Tick");
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let view_message = |message| {
            html! { <p>{ message }</p> }
        };
        let has_job = self.job.is_some();
        html! {
            <div>
                <button disabled=has_job
                        onclick=self.link.callback(|_| Msg::StartTimeout)>{ "Start Timeout" }</button>
                <button disabled=has_job
                        onclick=self.link.callback(|_| Msg::StartInterval)>{ "Start Interval" }</button>
                <button disabled=!has_job
                        onclick=self.link.callback(|_| Msg::Cancel)>{ "Cancel!" }</button>
                <div>
                    { for self.messages.iter().map(view_message) }
                </div>
            </div>
        }
    }
}
