use futures::FutureExt;
use tokio_stream::StreamExt;
use yew::{html, Component, Context, Html};

use crate::clock::{initialized_atomic_clocks, Clock};

mod clock;

/// The ClockComponent displays the current time. Its main purpose is to demonstrate the use of
/// async code in a yew component.
pub struct ClockComponent {
    current_time: Option<String>,
}

pub enum Msg {
    ClockInitialized(String),
    ClockTicked(String),
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

        // The stream_time method returns a stream of time updates. We use the while let loop to
        // consume the stream.
        let time_steam = clock.stream_time();
        ctx.link().send_stream(time_steam.map(Msg::ClockTicked));

        Self { current_time: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(current_time) => {
                self.current_time = Some(current_time);
            }
            Msg::ClockInitialized(init_message) => {
                self.current_time = Some(init_message);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let now = self.current_time.as_deref().unwrap_or("Loading...");
        html! {
            <div class="app">
                <div class="clock">
                    <h2>{ "Asynchronous Clock" }</h2>
                    <div class="time-display">
                        <h1>{ now }</h1>
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<ClockComponent>::new().render();
}
