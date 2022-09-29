use chrono::{DateTime, Local};
use futures::FutureExt;
use tokio_stream::StreamExt;
use yew::{html, AttrValue, Component, Context, Html};

use crate::clock::{initialized_atomic_clocks, Clock};

mod clock;

/// The ClockComponent displays the current time. Its main purpose is to demonstrate the use of
/// async code in a yew component.
pub struct ClockComponent {
    display: Option<AttrValue>,
}

pub enum Msg {
    ClockInitialized(()),
    ClockTicked(DateTime<Local>),
}

impl Component for ClockComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let clock = Clock::new();

        // Demonstrate how we can send a message to the component when a future completes.
        let is_initialized = initialized_atomic_clocks();
        ctx.link()
            .send_future(is_initialized.map(Msg::ClockInitialized));

        // The stream_time method returns a stream of time updates. We use send_stream to update the
        // component with a Msg::ClockTicked message every time the stream produces a new
        // value.
        let time_steam = clock.stream_time();
        ctx.link().send_stream(time_steam.map(Msg::ClockTicked));

        Self { display: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(current_time) => {
                self.display = Some(AttrValue::from(current_time.to_rfc2822()));
            }
            Msg::ClockInitialized(_) => {
                self.display = Some(AttrValue::from("Initialized"));
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let display = self.display.as_deref().unwrap_or("Loading...");
        html! {
            <div class="app">
                <div class="clock">
                    <h2>{ "Asynchronous Clock" }</h2>
                    <div class="time-display">
                        <h1>{ display }</h1>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<ClockComponent>::new().render();
}
