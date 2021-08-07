use gloo::console_timer::ConsoleTimer;
use gloo::timers::callback::{Interval, Timeout};
use weblog::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    UpdateTime,
}

pub struct Model {
    link: ComponentLink<Self>,
    time: String,
    messages: Vec<&'static str>,
    _standalone: (Interval, Interval),
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<ConsoleTimer<'static>>,
}

impl Model {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }

    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console_debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = link.clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            link,
            time: Model::get_current_time(),
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
            interval: None,
            timeout: None,
            console_timer: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartTimeout => {
                let handle = {
                    let link = self.link.clone();
                    Timeout::new(3, move || link.send_message(Msg::Done))
                };

                self.timeout = Some(handle);

                self.messages.clear();
                console_clear!();

                self.messages.push("Timer started!");
                self.console_timer = Some(ConsoleTimer::new("Timer"));
                true
            }
            Msg::StartInterval => {
                let handle = {
                    let link = self.link.clone();
                    Interval::new(1, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                self.messages.clear();
                console_clear!();

                self.messages.push("Interval started!");
                true
            }
            Msg::Cancel => {
                self.cancel();
                self.messages.push("Canceled!");
                console_warn!("Canceled!");
                true
            }
            Msg::Done => {
                self.cancel();
                self.messages.push("Done!");

                // todo weblog
                // ConsoleService::group();
                console_info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }

                // todo weblog
                // ConsoleService::group_end();
                true
            }
            Msg::Tick => {
                self.messages.push("Tick...");
                // todo weblog
                // ConsoleService::count_named("Tick");
                true
            }
            Msg::UpdateTime => {
                self.time = Model::get_current_time();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let has_job = self.timeout.is_some() || self.interval.is_some();
        html! {
            <>
                <div id="buttons">
                    <button disabled={has_job} onclick={self.link.callback(|_| Msg::StartTimeout)}>
                        { "Start Timeout" }
                    </button>
                    <button disabled={has_job} onclick={self.link.callback(|_| Msg::StartInterval)}>
                        { "Start Interval" }
                    </button>
                    <button disabled={!has_job} onclick={self.link.callback(|_| Msg::Cancel)}>
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
