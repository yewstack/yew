use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;

pub enum Msg {
    StartTimeout,
    StartInterval,
    Cancel,
    Done,
    Tick,
    UpdateTime,
}

pub struct App {
    time: String,
    messages: Vec<AttrValue>,
    _standalone: (Interval, Interval),
    interval: Option<Interval>,
    timeout: Option<Timeout>,
    console_timer: Option<Timer<'static>>,
}

impl App {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("en-US"))
    }

    fn cancel(&mut self) {
        self.timeout = None;
        self.interval = None;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let standalone_handle =
            Interval::new(10, || console::debug!("Example of a standalone callback."));

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            time: App::get_current_time(),
            messages: Vec::new(),
            _standalone: (standalone_handle, clock_handle),
            interval: None,
            timeout: None,
            console_timer: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartTimeout => {
                let handle = {
                    let link = ctx.link().clone();
                    Timeout::new(3000, move || link.send_message(Msg::Done))
                };

                self.timeout = Some(handle);

                self.messages.clear();
                console::clear!();

                self.log("Timer started!");
                self.console_timer = Some(Timer::new("Timer"));
                true
            }
            Msg::StartInterval => {
                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);

                self.messages.clear();
                console::clear!();

                self.log("Interval started!");
                true
            }
            Msg::Cancel => {
                self.cancel();
                self.log("Canceled!");
                console::warn!("Canceled!");
                true
            }
            Msg::Done => {
                self.cancel();
                self.log("Done!");

                // todo weblog
                // ConsoleService::group();
                console::info!("Done!");
                if let Some(timer) = self.console_timer.take() {
                    drop(timer);
                }

                // todo weblog
                // ConsoleService::group_end();
                true
            }
            Msg::Tick => {
                self.log("Tick...");
                // todo weblog
                // ConsoleService::count_named("Tick");
                true
            }
            Msg::UpdateTime => {
                self.time = App::get_current_time();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let has_job = self.timeout.is_some() || self.interval.is_some();
        html! {
            <>
                <div id="buttons">
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartTimeout)}>
                        { "Start Timeout" }
                    </button>
                    <button disabled={has_job} onclick={ctx.link().callback(|_| Msg::StartInterval)}>
                        { "Start Interval" }
                    </button>
                    <button disabled={!has_job} onclick={ctx.link().callback(|_| Msg::Cancel)}>
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

impl App {
    fn log(&mut self, message: impl Into<AttrValue>) {
        self.messages.push(message.into());
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
