mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn props_are_passed() {
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

    yew::start_app_with_props_in_element::<PropsComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
        PropsPassedFunctionProps {
            value: "props".to_string(),
        },
    );
    let result = obtain_result();
    assert_eq!(result.as_str(), "done");
}
