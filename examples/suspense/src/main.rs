use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

mod struct_consumer;
mod use_sleep;

pub use use_sleep::use_sleep;

#[derive(Debug, PartialEq, Properties)]
struct PleaseWaitProps {
    from: &'static str,
}

#[function_component(PleaseWait)]
fn please_wait(props: &PleaseWaitProps) -> Html {
    html! {<div class="content-area">{"Please wait 5 Seconds for "}{props.from}{" component to load..."}</div>}
}

#[function_component(AppContent)]
fn app_content() -> HtmlResult {
    let resleep = use_sleep()?;

    let value = use_state(|| "I am writing a long story...".to_string());

    let on_text_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();

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
    let fallback_fn = html! {<PleaseWait from="function" />};
    let fallback_struct = html! {<PleaseWait from="struct" />};

    html! {
        <div class="layout">
            <div class="content">
                <h2>{"  Yew Suspense Demo -- function component consumer"}</h2>
                    <Suspense fallback={fallback_fn}>
                        <AppContent  />
                    </Suspense>
            </div>
            <div class="content">
                <h2>{"Yew Suspense Demo -- struct component consumer"}</h2>
                <Suspense fallback={fallback_struct}>
                        <struct_consumer::AppContent />
                </Suspense>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
