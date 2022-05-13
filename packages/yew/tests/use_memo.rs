#![cfg(target_arch = "wasm32")]

use std::sync::atomic::{AtomicBool, Ordering};

mod common;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::tests::{TestCase, TestRunner};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_memo_works() {
    #[function_component]
    fn UseMemoComponent() -> Html {
        let state = use_state(|| 0);

        let memoed_val = use_memo(
            |_| {
                static CTR: AtomicBool = AtomicBool::new(false);

                if CTR.swap(true, Ordering::Relaxed) {
                    panic!("multiple times rendered!");
                }

                "true"
            },
            (),
        );

        use_effect(move || {
            if *state < 5 {
                state.set(*state + 1);
            }

            || {}
        });

        html! {
            <>
                { "The test result is: " }
                { *memoed_val }
            </>
        }
    }

    TestRunner::new()
        .render(html! {
            <UseMemoComponent />
        })
        .await
        .assert_inner_html("The test result is: true");
}
