use clock::Clock;
use wasm_bindgen_futures::spawn_local;
use yew::{html, Component, Context, Html};

mod clock;

/// The ClockComponent displays the current time. Its main purpose is to demonstrate the use of
/// async code in a yew component.
pub struct ClockComponent {
    current_time: Option<String>,
}

pub enum Msg {
    ClockTicked(String),
}

impl Component for ClockComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // create a callback that will be called when the clock ticks.
        let clock_ticked = ctx.link().callback(Msg::ClockTicked);

        // spawn_local starts a new async task. The task will send a message to the component when
        // the time changes.
        spawn_local(async move {
            let clock = Clock::new();

            // The stream_time method returns a stream of time updates. We use the while let loop to
            // consume the stream.
            let mut time_steam = clock.stream_time();

            // Endless loop that consumes the stream.
            while let Some(current_time) = time_steam.recv().await {
                // Send the current time to the component.
                clock_ticked.emit(current_time);
            }
        });

        Self { current_time: None }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockTicked(current_time) => {
                self.current_time = Some(current_time);
                true
            }
        }
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
