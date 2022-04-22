use gloo::console;
use gloo::timers::callback::Interval;
use yew::prelude::*;

pub struct CounterModel {
    counter: usize,
    _interval: Interval,
}

#[derive(Clone, Properties, PartialEq)]
pub struct CounterProps {
    pub destroy_callback: Callback<()>,
}

pub enum CounterMessage {
    Tick,
}

impl Component for CounterModel {
    type Message = CounterMessage;
    type Properties = CounterProps;

    fn create(ctx: &Context<Self>) -> Self {
        // Create a Tick message every second
        let link = ctx.link().clone();
        let interval = Interval::new(1, move || link.send_message(Self::Message::Tick));
        Self {
            counter: 0,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Count our internal state up by one
            Self::Message::Tick => {
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let destroy_callback = ctx.props().destroy_callback.clone();

        html! {
            <>
                // Display the current value of the counter
                <p class="counter">
                    { "App has lived for " }
                    { self.counter }
                    { " ticks" }
                </p>

                // Add button to send a destroy command to the parent app
                <button class="destroy" onclick={Callback::from(move |_| destroy_callback.emit(()))}>
                    { "Destroy this app" }
                </button>
            </>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        console::log!("CounterModel app destroyed");
    }
}
