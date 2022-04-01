#![cfg(feature = "wasm_test")]

use std::sync::atomic::{AtomicBool, Ordering};

mod common;

use common::obtain_result;
use gloo::timers::future::sleep;
use std::time::Duration;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_callback_works() {
    #[derive(Properties, PartialEq)]
    struct Props {
        callback: Callback<String, String>,
    }

    #[function_component(MyComponennt)]
    fn my_component(props: &Props) -> Html {
        let greeting = props.callback.emit("Yew".to_string());

        static CTR: AtomicBool = AtomicBool::new(false);

        if CTR.swap(true, Ordering::Relaxed) {
            panic!("multiple times rendered!");
        }

        html! {
            <div>
                {"The test output is: "}
                <div id="result">{&greeting}</div>
                {"\n"}
            </div>
        }
    }

    #[function_component(UseCallbackComponent)]
    fn use_callback_comp() -> Html {
        let state = use_state(|| 0);

        let callback = use_callback(move |name| format!("Hello, {}!", name), ());

        use_effect(move || {
            if *state < 5 {
                state.set(*state + 1);
            }

            || {}
        });

        html! {
            <div>
                <MyComponennt {callback} />
            </div>
        }
    }

    yew::Renderer::<UseCallbackComponent>::with_root(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "Hello, Yew!");
}
