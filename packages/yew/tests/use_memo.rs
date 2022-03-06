use std::sync::atomic::{AtomicBool, Ordering};

mod common;

use common::obtain_result;
use gloo::timers::future::sleep;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_memo_works() {
    #[function_component(UseMemoComponent)]
    fn use_memo_comp() -> Html {
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
            <div>
                {"The test output is: "}
                <div id="result">{*memoed_val}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseMemoComponent>::with_root(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "true");
}
