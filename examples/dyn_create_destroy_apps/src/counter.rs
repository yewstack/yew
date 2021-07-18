use gloo::timers::callback::Interval;
use yew::prelude::*;

pub struct CounterModel {
    counter: usize,
    props: CounterProps,
    _interval: Interval,
}

#[derive(Clone, Properties)]
pub struct CounterProps {
    pub destroy_callback: Callback<()>,
}

pub enum CounterMessage {
    Tick,
}

impl Component for CounterModel {
    type Message = CounterMessage;

    type Properties = CounterProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Create a Tick message every second
        let interval = Interval::new(1, move || link.send_message(Self::Message::Tick));
        Self {
            counter: 0,
            props,
            _interval: interval,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Count our internal state up by one
            Self::Message::Tick => {
                self.counter += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let destroy_callback = self.props.destroy_callback.clone();

        html! {
            <>
                // Display the current value of the counter
                <p class="counter">
                    { "App has lived for " }
                    { self.counter }
                    { " ticks" }
                </p>

                // Add button to send a destroy command to the parent app
                <button class="destroy" onclick=Callback::from(move |_| destroy_callback.emit(()))>
                    { "Destroy this app" }
                </button>
            </>
        }
    }

    fn destroy(&mut self) {
        weblog::console_log!("CounterModel app destroyed");
    }
}
