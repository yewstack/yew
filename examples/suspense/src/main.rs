use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod use_sleep;

use use_sleep::use_sleep;

#[function_component(PleaseWait)]
fn please_wait() -> Html {
    html! {<div class="content-area">{"Please wait 5 Seconds..."}</div>}
}

#[function_component(AppContent)]
fn app_content() -> HtmlResult {
    let resleep = use_sleep()?;

    let value = use_state(|| "I am writing a long story...".to_string());

    let on_text_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();

            value.set(input.value());
        })
    };

    let on_take_a_break = Callback::from(move |_| resleep());

    Ok(html! {
        <div class="content-area">
            <textarea value={value.to_string()} oninput={on_text_input}></textarea>
            <div class="action-area">
                <button onclick={on_take_a_break}>{"Take a break!"}</button>
                <div class="hint">{"You can take a break at anytime"}<br />{"and your work will be preserved."}</div>
            </div>
        </div>
    })
}

#[function_component(App)]
fn app() -> Html {
    let fallback = html! {<PleaseWait />};

    html! {
        <div class="layout">
            <div class="content">
                <h1>{"Yew Suspense Demo"}</h1>
                <Suspense fallback={fallback}>
                    <AppContent />
                </Suspense>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
