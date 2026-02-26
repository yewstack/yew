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
        assert_eq!(*counter.get(), *counter);
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

/// Exercises the exact pattern that causes use-after-free in the original PR #3963
/// fix, where `UseReducerHandle::deref()` drops the `Ref` guard but returns a
/// pointer derived from it.
///
/// The dangerous sequence within a single callback:
///   1. `handle.set(v1)` — dispatch puts a *new* `Rc` (refcount=1) in the shared `RefCell`,
///      replacing the one from render time.
///   2. `let r: &T = &*handle` — `deref()` borrows the RefCell, grabs a raw pointer into the Rc
///      (refcount still 1), and **drops the `Ref` guard**.
///   3. `handle.set(v2)` — dispatch replaces that Rc. Because its refcount was 1, it is freed. `r`
///      is now dangling.
///   4. Allocate objects of similar size to encourage the allocator to reuse the freed memory,
///      overwriting the old `T`.
///   5. Read through `r` — **use-after-free**.
///
/// With the `deref_history` fix, step 2 clones the Rc into a `Vec` kept alive by
/// the handle, bumping the refcount to 2. Step 3 only drops it to 1, so the
/// allocation survives and `r` remains valid.
#[wasm_bindgen_test]
async fn deref_remains_valid_across_multiple_dispatches_in_callback() {
    use std::cell::RefCell;

    use gloo::utils::document;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlElement;

    thread_local! {
        static DEREF_RESULT: RefCell<Option<String>> = const { RefCell::new(None) };
    }

    #[component(UBTestComponent)]
    fn ub_test_comp() -> Html {
        let state = use_state(|| "initial".to_string());

        let trigger = {
            let state = state.clone();
            Callback::from(move |_| {
                // Step 1: dispatch. The RefCell now contains a *new* Rc whose only
                // owner is the RefCell itself (refcount = 1).
                state.set("first_dispatch".to_string());

                // Step 2: deref. In the original fix the Ref guard is dropped
                // immediately, leaving us with a bare pointer into the refcount-1
                // Rc. With deref_history, the Rc is cloned into the Vec so the
                // refcount is bumped to 2.
                let borrowed: &String = &*state;

                // Step 3: dispatch again. The RefCell's old Rc is replaced.
                // Original fix: refcount was 1 → drops to 0 → freed → `borrowed`
                //   dangles.
                // deref_history fix: refcount was 2 → drops to 1 (still in Vec)
                //   → allocation survives → `borrowed` is valid.
                state.set("second_dispatch".to_string());

                // Step 4: churn the allocator. Create and drop many heap objects
                // of ~32 bytes (the size of the freed Rc+UseStateReducer+String
                // struct on wasm32) to maximize the chance that the allocator
                // hands out the freed address to one of these, overwriting the
                // memory `borrowed` points into.
                for _ in 0..256 {
                    // Each Box<[u8; 32]> is roughly the same size as the freed Rc
                    // allocation containing UseStateReducer<String>.
                    let overwrite = Box::new([0xFFu8; 32]);
                    std::hint::black_box(&*overwrite);
                    drop(overwrite);
                }

                // Also allocate Strings whose *buffers* might reuse the freed
                // String buffer from step 1.
                let _noise: Vec<String> = (0..64).map(|i| format!("noise_{:032}", i)).collect();

                // Step 5: read through the potentially-dangling reference.
                // With the original fix this is UB: the memory behind `borrowed`
                // may have been reused by the allocations above, so `.clone()`
                // could read a garbage ptr/len/cap triple and trap, or silently
                // return corrupted data.
                // With deref_history, this always reads "first_dispatch".
                let value = borrowed.clone();

                DEREF_RESULT.with(|r| {
                    *r.borrow_mut() = Some(value);
                });
            })
        };

        html! {
            <div>
                <button id="ub-trigger" onclick={trigger}>{"Trigger"}</button>
                <div id="result">{(*state).clone()}</div>
            </div>
        }
    }

    yew::Renderer::<UBTestComponent>::with_root(document().get_element_by_id("output").unwrap())
        .render();
    sleep(Duration::ZERO).await;

    // Fire the callback
    document()
        .get_element_by_id("ub-trigger")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    sleep(Duration::ZERO).await;

    // The reference obtained between the two dispatches must still read the
    // value from the first dispatch, not garbage or "second_dispatch".
    let captured = DEREF_RESULT.with(|r| r.borrow().clone());
    assert_eq!(
        captured,
        Some("first_dispatch".to_string()),
        "deref() reference must remain valid across subsequent dispatches"
    );
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
///    reading from the shared RefCell
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

    // Click update-a, then update-b, then submit WITHOUT waiting for rerender.
    // This simulates rapid user interaction (like the Firefox bug in issue #3796).
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

    // Check the values captured by the submit handler.
    // Before the fix, field_b would be empty because the callback captured a stale handle.
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
