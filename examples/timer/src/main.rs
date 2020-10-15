use std::time::Duration;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::services::{ConsoleService, Task, TimeoutService};
use yew::{html, Callback, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    StartClock,
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    Tock,
}

pub struct Model {
    link: ComponentLink<Self>,
    job: Option<Box<dyn Task>>,
    clock_job: Option<Box<dyn Task>>,
    time: String,
    messages: Vec<&'static str>,
    _standalone: IntervalTask,
}

impl Model {
    fn get_current_time(&self) -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::StartClock);
        let handle = IntervalService::spawn(
            Duration::from_secs(10),
            // This callback doesn't send any message to a scope
            Callback::from(|_| {
                ConsoleService::debug("Example of a standalone callback.");
            }),
        );

        Self {
            link,
            job: None,
            clock_job: None,
            time: "".to_string(),
            messages: Vec::new(),
            _standalone: handle,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartClock => {
                self.time = self.get_current_time();
                let handle = IntervalService::spawn(
                    Duration::from_secs(1),
                    self.link.callback(|_| Msg::Tock),
                );
                self.clock_job = Some(Box::new(handle));

                true
            }
            Msg::StartTimeout => {
                let handle = TimeoutService::spawn(
                    Duration::from_secs(3),
                    self.link.callback(|_| Msg::Done),
                );
                self.job = Some(Box::new(handle));

                self.messages.clear();
                ConsoleService::clear();

                self.messages.push("Timer started!");
                ConsoleService::time_named("Timer");
                true
            }
            Msg::StartInterval => {
                let handle = IntervalService::spawn(
                    Duration::from_secs(1),
                    self.link.callback(|_| Msg::Tick),
                );
                self.job = Some(Box::new(handle));

                self.messages.clear();
                ConsoleService::clear();

                self.messages.push("Interval started!");
                true
            }
            Msg::Cancel => {
                self.job = None;
                self.messages.push("Canceled!");
                ConsoleService::warn("Canceled!");
                true
            }
            Msg::Done => {
                self.job = None;
                self.messages.push("Done!");

                ConsoleService::group();
                ConsoleService::info("Done!");
                ConsoleService::time_named_end("Timer");
                ConsoleService::group_end();
                true
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                ConsoleService::count_named("Tick");
                true
            }
            Msg::Tock => {
                self.time = self.get_current_time();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let has_job = self.job.is_some();
        html! {
            <>
                <div id="buttons">
                    <button disabled=has_job onclick=self.link.callback(|_| Msg::StartTimeout)>
                        { "Start Timeout" }
                    </button>
                    <button disabled=has_job onclick=self.link.callback(|_| Msg::StartInterval)>
                        { "Start Interval" }
                    </button>
                    <button disabled=!has_job onclick=self.link.callback(|_| Msg::Cancel)>
                        { "Cancel!" }
                    </button>
                </div>
                <div id="wrapper">
                    <div id="time">
                    { &self.time }
                    </div>
                    <div id="messages">
                        { for self.messages.iter().map(|message| html! { <p>{ message }</p> }) }
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
