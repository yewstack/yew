#![cfg(target_arch = "wasm32")]

mod common;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::tests::{TestCase, TestRunner};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_state_works() {
    #[function_component]
    fn UseComponent() -> Html {
        let counter = use_state(|| 0);
        if *counter < 5 {
            counter.set(*counter + 1)
        }
        html! {
            <>
                { "Test Output: " }
                { *counter }
            </>
        }
    }

    TestRunner::new()
        .render(html! {
            <UseComponent />
        })
        .await
        .assert_inner_html("Test Output: 5");
}

#[wasm_bindgen_test]
async fn multiple_use_state_setters() {
    #[function_component]
    fn UseComponent() -> Html {
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
            <>
                { "Test Output: " }
                { *counter }
            </>
        }
    }

    TestRunner::new()
        .render(html! {
            <UseComponent />
        })
        .await
        .assert_inner_html("Test Output: 11");
}

#[wasm_bindgen_test]
async fn use_state_eq_works() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[function_component]
    fn UseComponent() -> Html {
        RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
        let counter = use_state_eq(|| 0);
        counter.set(1);

        html! {
            <>
                { "Test Output: " }
                { *counter }
            </>
        }
    }

    TestRunner::new()
        .render(html! {
            <UseComponent />
        })
        .await
        .assert_inner_html("Test Output: 1");
    assert_eq!(RENDER_COUNT.load(Ordering::Relaxed), 2);
}
