#![cfg(target_arch = "wasm32")]

mod common;

use std::ops::DerefMut;
use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_ref_works() {
    #[function_component(UseRefComponent)]
    fn use_ref_comp() -> Html {
        let ref_example = use_mut_ref(|| 0);
        *ref_example.borrow_mut().deref_mut() += 1;
        let counter = use_state(|| 0);
        if *counter < 5 {
            counter.set(*counter + 1)
        }
        html! {
            <div>
                {"The test output is: "}
                <div id="result">{*ref_example.borrow_mut().deref_mut() > 4}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseRefComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "true");
}
