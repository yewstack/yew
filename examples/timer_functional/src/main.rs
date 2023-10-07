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
    SetInterval(Interval),
    SetTimeout(Timeout),
    TimeoutDone,
}

#[derive(Clone, Debug)]
struct TimerState {
    messages: Vec<&'static str>,
    interval_handle: Option<Rc<Interval>>,
    timeout_handle: Option<Rc<Timeout>>,
}

impl PartialEq for TimerState {
    fn eq(&self, other: &Self) -> bool {
        self.messages == other.messages
            && self.interval_handle.is_some() == other.interval_handle.is_some()
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
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: self.timeout_handle.clone(),
                })
            }
            TimerAction::SetInterval(t) => Rc::new(TimerState {
                messages: vec!["Interval started!"],
                interval_handle: Some(Rc::from(t)),
                timeout_handle: self.timeout_handle.clone(),
            }),
            TimerAction::SetTimeout(t) => Rc::new(TimerState {
                messages: vec!["Timer started!!"],
                interval_handle: self.interval_handle.clone(),
                timeout_handle: Some(Rc::from(t)),
            }),
            TimerAction::TimeoutDone => {
                let mut messages = self.messages.clone();
                messages.push("Done!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: self.interval_handle.clone(),
                    timeout_handle: None,
                })
            }
            TimerAction::Cancel => {
                let mut messages = self.messages.clone();
                messages.push("Canceled!");
                Rc::new(TimerState {
                    messages,
                    interval_handle: None,
                    timeout_handle: None,
                })
            }
        }
    }
}

#[function_component(Clock)]
fn clock() -> Html {
    let time = use_state(get_current_time);

    {
        let time = time.clone();
        use_effect_with((), |_| {
            Interval::new(1000, move || time.set(get_current_time())).forget();
        });
    }
    html!(
        <div id="time">{ time.as_str() }</div>
    )
}

#[function_component]
fn App() -> Html {
    let state = use_reducer(|| TimerState {
        messages: Vec::new(),
        interval_handle: None,
        timeout_handle: None,
    });

    let mut key = 0;
    let messages: Html = state
        .messages
        .iter()
        .map(|message| {
            key += 1;
            html! { <p key={ key }>{ *message }</p> }
        })
        .collect();

    let has_job = state.interval_handle.is_some() || state.timeout_handle.is_some();

    let on_add_timeout = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let timeout_state = state.clone();
            let message_state = state.clone();
            let t = Timeout::new(3000, move || {
                message_state.dispatch(TimerAction::TimeoutDone);
            });

            timeout_state.dispatch(TimerAction::SetTimeout(t));
        })
    };

    let on_add_interval = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let interval_state = state.clone();
            let message_state = state.clone();
            let i = Interval::new(1000, move || {
                message_state.dispatch(TimerAction::Add("Tick.."));
            });

            interval_state.dispatch(TimerAction::SetInterval(i));
        })
    };

    let on_cancel = {
        Callback::from(move |_: MouseEvent| {
            state.dispatch(TimerAction::Cancel);
        })
    };

    html!(
        <>
            <div id="buttons">
                <button disabled={has_job} onclick={on_add_timeout}>{ "Start Timeout" }</button>
                <button disabled={has_job} onclick={on_add_interval}>{ "Start Interval" }</button>
                <button disabled={!has_job} onclick={on_cancel}>{ "Cancel"}</button>
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
