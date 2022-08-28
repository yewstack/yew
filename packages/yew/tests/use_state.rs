#![cfg(target_arch = "wasm32")]

mod common;

use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_state_works() {
    #[function_component(UseComponent)]
    fn use_state_comp() -> Html {
        let counter = use_state(|| 0);
        if *counter < 5 {
            counter.set(*counter + 1)
        }
        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "5");
}

#[wasm_bindgen_test]
async fn multiple_use_state_setters() {
    #[function_component(UseComponent)]
    fn use_state_comp() -> Html {
        let counter = use_state(|| 0);
        let counter_clone = counter.clone();
        use_effect_with_deps(
            move |_| {
                // 1st location
                counter_clone.set(*counter_clone + 1);
                || {}
            },
            (),
        );
        let another_scope = {
            let counter = counter.clone();
            move || {
                if *counter < 11 {
                    // 2nd location
                    counter.set(*counter + 10)
                }
            }
        };
        another_scope();
        html! {
            <div>
                { "Test Output: " }
                // expected output
                <div id="result">{ *counter }</div>
                { "\n" }
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "11");
}

#[wasm_bindgen_test]
async fn use_state_eq_works() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[function_component(UseComponent)]
    fn use_state_comp() -> Html {
        RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
        let counter = use_state_eq(|| 0);
        counter.set(1);

        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "1");
    assert_eq!(RENDER_COUNT.load(Ordering::Relaxed), 2);
}
