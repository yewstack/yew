#![cfg(target_arch = "wasm32")]

mod common;

use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn props_are_passed() {
    #[derive(Properties, Clone, PartialEq)]
    struct PropsPassedFunctionProps {
        value: String,
    }

    #[function_component]
    fn PropsComponent(props: &PropsPassedFunctionProps) -> Html {
        assert_eq!(&props.value, "props");
        html! {
            <div id="result">
                {"done"}
            </div>
        }
    }

    yew::Renderer::<PropsComponent>::with_root_and_props(
        gloo::utils::document().get_element_by_id("output").unwrap(),
        PropsPassedFunctionProps {
            value: "props".to_string(),
        },
    )
    .render();

    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "done");
}
