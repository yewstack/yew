pub mod agent;
use agent::{ControlSignal, PrimeReactor};
use yew::prelude::*;
use yew_agent::reactor::{use_reactor_subscription, ReactorProvider};

#[function_component]
fn Main() -> Html {
    let prime_sub = use_reactor_subscription::<PrimeReactor>();
    let started = use_state_eq(|| false);
    let skip_len = use_state_eq(|| 0);

    let result_s = prime_sub
        .iter()
        // Skip results in previous runs.
        .skip(*skip_len)
        .fold("".to_string(), |mut output, item| {
            if !output.is_empty() {
                output.push_str(", ");
            }

            output.push_str(&item.to_string());

            output
        });

    let start_prime_calc = use_callback(
        (prime_sub.clone(), started.setter(), skip_len.setter()),
        |_input, (prime_sub, started_setter, skip_len)| {
            skip_len.set(prime_sub.len());
            prime_sub.send(ControlSignal::Start);
            started_setter.set(true);
        },
    );

    let stop_prime_calc = use_callback(
        (prime_sub, started.setter()),
        |_input, (prime_sub, started_setter)| {
            prime_sub.send(ControlSignal::Stop);
            started_setter.set(false);
        },
    );

    html! {
        <>
            <h1>{"Find Prime"}</h1>
            <p>{"This page demonstrates how to calculate prime in a web worker."}</p>
            if *started {
                <button onclick={stop_prime_calc}>{"Stop"}</button>
            } else {
                <button onclick={start_prime_calc}>{"Start"}</button>
            }
            <div id="result">{result_s}</div>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <ReactorProvider<PrimeReactor> path="/worker.js">
            <Main />
        </ReactorProvider<PrimeReactor>>
    }
}
