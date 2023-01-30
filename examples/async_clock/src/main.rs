use chrono::{DateTime, Local};
use futures::{FutureExt, StreamExt};
use services::compute_fun_score;
use yew::platform::pinned::mpsc::UnboundedSender;
use yew::{html, AttrValue, Component, Context, Html};

use crate::services::{emit_jokes, initialize_atomic_clocks, stream_time};

mod services;

/// The AsyncComponent displays the current time and some silly jokes. Its main purpose is to
/// demonstrate the use of async code in a yew component. It uses the following async features:
/// - send_future
/// - send_stream
/// - spawn_local
/// - mpsc::unbounded channels
pub struct AsyncComponent {
    clock: Option<AttrValue>,
    joke: Option<AttrValue>,
    fun_score: Option<i16>,
    fun_score_channel: UnboundedSender<AttrValue>,
}

pub enum Msg {
    ClockInitialized(()),
    ClockTicked(DateTime<Local>),
    Joke(AttrValue),
    FunScore(i16),
}

impl Component for AsyncComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Demonstrate how we can send a message to the component when a future completes.
        // This is the most straightforward way to use async code in a yew component.
        let is_initialized = initialize_atomic_clocks();
        ctx.link()
            .send_future(is_initialized.map(Msg::ClockInitialized));

        // The compute_fun_score launches a background task that is ready to compute the fun score
        // from jokes that are delivered on this channel. The outcome of the computation is
        // sent back to the component via the Msg::FunScore callback.
        let fun_score_cb = ctx.link().callback(Msg::FunScore);
        let fun_score_channel = compute_fun_score(fun_score_cb);

        Self {
            clock: None,
            joke: None,
            fun_score: None,
            fun_score_channel,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(current_time) => {
                // Update the clock display
                self.clock = Some(AttrValue::from(current_time.to_rfc2822()));
            }
            Msg::ClockInitialized(_) => {
                // Now that the clock is initialized, we can start the time stream.
                self.clock = Some(AttrValue::from("Initialized"));

                // The stream_time method returns a stream of time updates. We use send_stream to
                // update the component with a Msg::ClockTicked message every time
                // the stream produces a new value.
                let time_steam = stream_time();
                ctx.link().send_stream(time_steam.map(Msg::ClockTicked));

                // In parallel we launch a background task that produces jokes to make the clock
                // more fun to watch. The jokes are emitted back to the component
                // throught the Msg::Joke callback.
                let joke_cb = ctx.link().callback(Msg::Joke);
                emit_jokes(joke_cb);
            }
            Msg::Joke(joke) => {
                // Update the joke
                self.joke = Some(joke.clone());

                // Reset the fun score
                self.fun_score = None;

                // Send the joke to the background task that computes the fun score.
                self.fun_score_channel
                    .send_now(joke)
                    .expect("failed to send joke");
            }
            Msg::FunScore(score) => {
                self.fun_score = Some(score);
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let display = self.clock.as_deref().unwrap_or("Loading...");
        let joke = self.joke.as_deref().unwrap_or("Loading...");
        let fun_score = self
            .fun_score
            .map(|score| format!("Fun score: {score}"))
            .unwrap_or_else(|| "Computing...".to_string());

        html! {
            <div class="app">
                <div class="clock">
                    <h2>{ "Asynchronous Examples" }</h2>
                    <div class="time-display">
                        { display }
                    </div>
                    <div class="joke-display">
                        { joke }
                    </div>
                    <div class="fun-score-display">
                        { fun_score }
                    </div>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<AsyncComponent>::new().render();
}
