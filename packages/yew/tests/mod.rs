#![cfg(target_arch = "wasm32")]

mod common;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::tests::{TestCase, TestRunner};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn props_are_passed() {
    #[derive(Properties, Clone, PartialEq)]
    struct PropsPassedFunctionProps {
        value: String,
    }

    #[function_component]
    fn PropsComponent(props: &PropsPassedFunctionProps) -> Html {
        html! {
            {&props.value}
        }
    }

    TestRunner::new()
        .render(html! {
            <PropsComponent value="props123" />
        })
        .await
        .assert_inner_html("props123");
}
