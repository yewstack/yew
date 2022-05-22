#![cfg(target_arch = "wasm32")]

use std::sync::atomic::{AtomicBool, Ordering};

mod common;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::tests::{TestCase, TestRunner};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_callback_works() {
    #[derive(Properties, PartialEq)]
    struct Props {
        callback: Callback<String, String>,
    }

    #[function_component(MyComponent)]
    fn my_component(props: &Props) -> Html {
        let greeting = props.callback.emit("Yew".to_string());

        static CTR: AtomicBool = AtomicBool::new(false);

        if CTR.swap(true, Ordering::Relaxed) {
            panic!("multiple times rendered!");
        }

        html! {
            <>{"Callback output: "}{&greeting}</>
        }
    }

    #[function_component]
    fn UseCallbackComponent() -> Html {
        let state = use_state(|| 0);

        let callback = use_callback(move |name, _| format!("Hello, {}!", name), ());

        use_effect(move || {
            if *state < 5 {
                state.set(*state + 1);
            }

            || {}
        });

        html! {
            <MyComponent {callback} />
        }
    }

    let mut trun = TestRunner::new();
    trun.render(html! { <UseCallbackComponent /> })
        .await
        .assert_inner_html("Callback output: Hello, Yew!");
}
