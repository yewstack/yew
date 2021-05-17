use std::time::Duration;
use yew::prelude::*;
use yew_services::{
    interval::{IntervalService, IntervalTask},
    ConsoleService,
};

pub struct CounterModel {
    counter: usize,
    props: CounterProps,
    _interval_task: IntervalTask,
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
        let interval_task = IntervalService::spawn(
            Duration::from_secs(1),
            link.callback(|()| Self::Message::Tick),
        );
        Self {
            counter: 0,
            props,
            _interval_task: interval_task,
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
        ConsoleService::log("CounterModel app destroyed");
    }
}
