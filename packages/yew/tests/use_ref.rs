#![cfg(target_arch = "wasm32")]

mod common;

use std::ops::DerefMut;

use wasm_bindgen_test::*;
use yew::prelude::*;
use yew::tests::{TestCase, TestRunner};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_ref_works() {
    #[function_component]
    fn UseRefComponent() -> Html {
        let ref_example = use_mut_ref(|| 0);
        *ref_example.borrow_mut().deref_mut() += 1;
        let counter = use_state(|| 0);
        if *counter < 5 {
            counter.set(*counter + 1)
        }
        html! {
            <>
                {"The test result is: "}
                { *ref_example.borrow_mut().deref_mut() > 4 }
            </>
        }
    }

    TestRunner::new()
        .render(html! {
            <UseRefComponent />
        })
        .await
        .assert_inner_html("The test result is: true");
}
