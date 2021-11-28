use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

use log::Level;

mod use_sleep;

use use_sleep::use_sleep;

#[function_component(PleaseWait)]
fn please_wait() -> Html {
    html! {<div>{"Please wait..."}</div>}
}

#[function_component(AppContent)]
fn app_content() -> Html {
    let resleep = use_sleep()?;

    let value = use_state(|| "".to_string());

    let on_text_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();

            value.set(input.value());
        })
    };

    let on_take_a_break = Callback::from(move |_| (resleep.clone())());

    html! {
        <>
            <textarea value={value.to_string()} oninput={on_text_input}></textarea>
            <button onclick={on_take_a_break}>{"Take a break!"}</button>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<PleaseWait />};

    html! {
        <Suspense fallback={fallback}>
            <AppContent />
        </Suspense>
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
