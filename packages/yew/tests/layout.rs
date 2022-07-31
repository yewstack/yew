#![cfg(target_arch = "wasm32")]

mod common;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

#[wasm_bindgen_test]
async fn change_nested_after_append() {
    #[function_component]
    fn Nested() -> Html {
        let delayed_trigger = use_state(|| true);

        {
            let delayed_trigger = delayed_trigger.clone();
            use_effect_with_deps(
                move |_| {
                    spawn_local(async move {
                        sleep(Duration::from_millis(50)).await;
                        delayed_trigger.set(false);
                    });
                    || {}
                },
                (),
            );
        }

        if *delayed_trigger {
            html! { <div>{"failure"}</div> }
        } else {
            html! { <><i></i><span id="result">{"success"}</span></> }
        }
    }

    #[function_component]
    fn Top() -> Html {
        html! { <Nested /> }
    }

    #[function_component]
    fn App() -> Html {
        let show_bottom = use_state_eq(|| false);

        {
            let show_bottom = show_bottom.clone();

            use_effect_with_deps(
                move |_| {
                    show_bottom.set(true);
                    || {}
                },
                (),
            );
        }

        html! {
            <>
                <Top />
                if *show_bottom {
                    <div>{"<div>Bottom</div>"}</div>
                }
            </>
        }
    }

    yew::Renderer::<App>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();

    sleep(Duration::from_millis(100)).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "success");
}
