use std::rc::Rc;

use gloo::console::log;
use gloo::timers::callback::{Interval, Timeout};
use yew::prelude::*;

fn get_current_time() -> String {
    let date = js_sys::Date::new_0();
    String::from(date.to_locale_time_string("en-US"))
}

enum TimerAction {
    Add(&'static str),
    Clear,
}

#[derive(Clone, Debug, PartialEq)]
struct TimerState {
    messages: Vec<&'static str>,
}

impl Reducible for TimerState {
    type Action = TimerAction;

    fn reduce(self: Rc<Self>, action: TimerAction) -> Rc<Self> {
        match action {
            TimerAction::Add(message) => {
                log!("add called");
                let mut messages = self.messages.clone();
                messages.push(message);
                Rc::new(TimerState { messages })
            }
            TimerAction::Clear => Rc::new(TimerState {
                messages: Vec::new(),
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
    });

    let messages: Html = state
        .messages
        .iter()
        .map(|message| html! { <p>{ message }</p> })
        .collect();

    let on_add = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            state.dispatch(TimerAction::Add("Timeout called"));
            Timeout::new(3000, move || {
                state.dispatch(TimerAction::Add("Timeout done."));
            })
            .forget();
        })
    };

    let on_add_interval = {
        let state = state.clone();

        Callback::from(move |_: MouseEvent| {
            let state = state.clone();
            state.dispatch(TimerAction::Add("Interval called"));
            let _i = Interval::new(3000, move || {
                state.dispatch(TimerAction::Add("Interval done."));
            })
            .forget();
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
                <button onclick={on_add}>{ "Start Timeout" }</button>
                <button onclick={on_add_interval}>{ "Start Interval" }</button>
                <button onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div id="wrapper">
                <Clock />
            </div>
            <div id="messages">
                { messages }
            </div>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
