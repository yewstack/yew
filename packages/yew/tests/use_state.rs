#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

mod common;

use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_state_works() {
    #[component(UseComponent)]
    fn use_state_comp() -> Html {
        let counter = use_state(|| 0);
        if *counter < 5 {
            counter.set(*counter + 1)
        }
        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "5");
}

#[wasm_bindgen_test]
async fn multiple_use_state_setters() {
    #[component(UseComponent)]
    fn use_state_comp() -> Html {
        let counter = use_state(|| 0);
        let counter_clone = counter.clone();
        use_effect_with((), move |_| {
            // 1st location
            counter_clone.set(*counter_clone + 1);
            || {}
        });
        let another_scope = {
            let counter = counter.clone();
            move || {
                if *counter < 11 {
                    // 2nd location
                    counter.set(*counter + 10)
                }
            }
        };
        another_scope();
        html! {
            <div>
                { "Test Output: " }
                // expected output
                <div id="result">{ *counter }</div>
                { "\n" }
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "11");
}

#[wasm_bindgen_test]
async fn use_state_eq_works() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static RENDER_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[component(UseComponent)]
    fn use_state_comp() -> Html {
        RENDER_COUNT.fetch_add(1, Ordering::Relaxed);
        let counter = use_state_eq(|| 0);
        counter.set(1);

        html! {
            <div>
                {"Test Output: "}
                <div id="result">{*counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "1");
    assert_eq!(RENDER_COUNT.load(Ordering::Relaxed), 2);
}

/// Regression test for issue #3796
/// Tests that state handles always read the latest value even when accessed
/// from callbacks before a rerender occurs.
///
/// The bug occurred when:
/// 1. State A is updated via set()
/// 2. State B is updated via set()
/// 3. A callback reads both states before rerender
/// 4. The callback would see stale value for B because the handle was caching a snapshot instead of
///    reading from the RefCell
#[wasm_bindgen_test]
async fn use_state_handles_read_latest_value_issue_3796() {
    use std::cell::RefCell;

    use gloo::utils::document;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    // Shared storage for the values read by the submit handler
    thread_local! {
        static CAPTURED_VALUES: RefCell<Option<(String, String)>> = const { RefCell::new(None) };
    }

    #[component(FormComponent)]
    fn form_comp() -> Html {
        let field_a = use_state(String::new);
        let field_b = use_state(String::new);

        let update_a = {
            let field_a = field_a.clone();
            Callback::from(move |_| {
                field_a.set("value_a".to_string());
            })
        };

        let update_b = {
            let field_b = field_b.clone();
            Callback::from(move |_| {
                field_b.set("value_b".to_string());
            })
        };

        // This callback reads both states - the bug caused field_b to be stale
        let submit = {
            let field_a = field_a.clone();
            let field_b = field_b.clone();
            Callback::from(move |_| {
                let a = (*field_a).clone();
                let b = (*field_b).clone();
                CAPTURED_VALUES.with(|v| {
                    *v.borrow_mut() = Some((a.clone(), b.clone()));
                });
            })
        };

        html! {
            <div>
                <button id="update-a" onclick={update_a}>{"Update A"}</button>
                <button id="update-b" onclick={update_b}>{"Update B"}</button>
                <button id="submit" onclick={submit}>{"Submit"}</button>
                <div id="result">{format!("a={}, b={}", *field_a, *field_b)}</div>
            </div>
        }
    }

    yew::Renderer::<FormComponent>::with_root(document().get_element_by_id("output").unwrap())
        .render();
    sleep(Duration::ZERO).await;

    // Initial state
    let result = obtain_result();
    assert_eq!(result.as_str(), "a=, b=");

    // Click update-a, then update-b, then submit WITHOUT waiting for rerender
    // This simulates rapid user interaction (like the Firefox bug in issue #3796)
    document()
        .get_element_by_id("update-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    document()
        .get_element_by_id("update-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    document()
        .get_element_by_id("submit")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    // Now wait for rerenders to complete
    sleep(Duration::ZERO).await;

    // Check the values captured by the submit handler
    // Before the fix, field_b would be empty because the callback captured a stale handle
    let captured = CAPTURED_VALUES.with(|v| v.borrow().clone());
    assert_eq!(
        captured,
        Some(("value_a".to_string(), "value_b".to_string())),
        "Submit handler should see latest values for both fields"
    );

    // Also verify the DOM shows correct values after rerender
    let result = obtain_result();
    assert_eq!(result.as_str(), "a=value_a, b=value_b");
}
