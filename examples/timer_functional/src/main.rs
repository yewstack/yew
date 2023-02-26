use std::rc::Rc;
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

enum TimerAction {
    Add(&'static str),
    Cancel,
    Clear,
    SetInterval(Interval),
}

#[derive(Clone, Debug)]
struct TimerState {
    messages: Vec<&'static str>,
    handle: Option<Rc<Interval>>,
}

impl PartialEq for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages && self.handle.is_some() == other.handle.is_some()
    }
}

impl Reducible for TimerState {
    type Action = TimerAction;

    fn reduce(self: Rc<Self>, action: TimerAction) -> Rc<Self> {
        match action {
            TimerAction::Add(message) => {
                let mut messages = self.messages.clone();
                messages.push(message);
                Rc::new(TimerState {
                    messages,
                    handle: self.handle.clone(),
                })
            }
            TimerAction::SetInterval(t) => Rc::new(TimerState {
                messages: self.messages.clone(),
                handle: Some(Rc::from(t)),
            }),
            TimerAction::Cancel => {
                let mut messages = self.messages.clone();
                messages.push("Canceled!");
                Rc::new(TimerState {
                    messages,
                    handle: None,
                })
            }
            TimerAction::Clear => Rc::new(TimerState {
                messages: Vec::new(),
                handle: self.handle.clone(),
            }),
        }
    }
}

#[function_component(Clock)]
fn clock() -> Html {
    let time = use_state(|| get_current_time());

    {
        let time = time.clone();
        use_effect_with_deps(
            |_| {
                Interval::new(1000, move || time.set(get_current_time())).forget();
            },
            (),
        );
    }
    html!(
        <div id="time">{ time.as_str() }</div>
    )
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| TimerState {
        messages: Vec::new(),
        handle: None,
    });

    let messages: Html = state
        .messages
        .iter()
        .map(|message| html! { <p>{ message }</p> })
        .collect();

    let on_add_timeout = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            state.dispatch(TimerAction::Add("Timer Started!"));
            Timeout::new(3000, move || {
                state.dispatch(TimerAction::Add("Done!"));
            })
            .forget();
        })
    };

    let on_add_interval = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let interval_state = state.clone();
            let message_state = state.clone();
            message_state.dispatch(TimerAction::Add("Interval started!"));
            let i = Interval::new(3000, move || {
                message_state.dispatch(TimerAction::Add("Tick.."));
            });

            interval_state.dispatch(TimerAction::SetInterval(i));
        })
    };

    let on_cancel = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Cancel);
        })
    };

    let on_clear = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Clear);
        })
    };

    html!(
        <>
            <div id="buttons">
                <button onclick={on_add_timeout}>{ "Start Timeout" }</button>
                <button onclick={on_add_interval}>{ "Start Interval" }</button>
                <button onclick={on_cancel}>{ "Cancel"}</button>
                <button onclick={on_clear}>{ "Clear"}</button>
            </div>
            <div id="wrapper">
                <Clock />
                <div id="messages">
                    { messages }
                </div>
            </div>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
