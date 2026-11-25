#![cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "ssr")))]

use std::time::Duration;

use gloo::utils::document;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use yew::platform::time::sleep;
use yew::prelude::*;
use yew_link::{LinkProvider, LinkedState, Never, use_linked_state};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(inline_js = "
const requests = [];
const fetch = window.fetch;
export function mock_fetch() {
    window.fetch = (request, ...args) => {
        if (!request.url?.endsWith('/linked-state-test')) {
            return fetch.call(window, request, ...args);
        }
        return new Promise(resolve => requests.push(resolve));
    };
}
export function request_count() {
    return requests.length;
}
export function respond(index, body) {
    requests[index](new Response(body));
}
export function restore_fetch() {
    window.fetch = fetch;
}
")]
extern "C" {
    fn mock_fetch();
    fn request_count() -> usize;
    fn respond(index: usize, body: &str);
    fn restore_fetch();
}

#[derive(Clone, Deserialize, Serialize)]
struct Value(u32);

impl LinkedState for Value {
    type Error = Never;
    type Input = u32;

    const TYPE_KEY: &'static str = "Value";
}

#[derive(Clone, PartialEq, Properties)]
struct WatchProps {
    id: &'static str,
    input: u32,
    revision: u32,
}

#[component]
fn Watch(props: &WatchProps) -> HtmlResult {
    let value = use_linked_state::<Value>(props.input)?;
    let text = match value.as_result() {
        Ok(value) => value.0.to_string(),
        Err(_) => "error".into(),
    };
    let refreshing = if value.is_refreshing() {
        " refreshing"
    } else {
        ""
    };
    let onclick = Callback::from(move |_| value.refresh());
    Ok(html! { <button id={props.id} {onclick}>{ format!("{text}{refreshing}") }</button> })
}

#[derive(Clone, PartialEq, Properties)]
struct AppProps {
    second: u32,
    revision: u32,
}

#[component]
fn App(props: &AppProps) -> Html {
    html! {
        <LinkProvider endpoint="/linked-state-test" cache_capacity={1}>
            <Suspense fallback={html! { "loading first" }}>
                <Watch id="first" input={1} revision={props.revision} />
            </Suspense>
            <Suspense fallback={html! { "loading second" }}>
                <Watch id="second" input={props.second} revision={props.revision} />
            </Suspense>
        </LinkProvider>
    }
}

async fn wait_for(mut condition: impl FnMut() -> bool) {
    for _ in 0..100 {
        if condition() {
            return;
        }
        sleep(Duration::from_millis(10)).await;
    }
    panic!("timed out waiting for linked state");
}

fn text(id: &str) -> Option<String> {
    document().get_element_by_id(id)?.text_content()
}

fn click(id: &str) {
    document()
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>()
        .click();
}

#[wasm_bindgen_test]
async fn coalesces_requests_and_keeps_results_for_each_query() {
    mock_fetch();
    struct RestoreFetch;
    impl Drop for RestoreFetch {
        fn drop(&mut self) {
            restore_fetch();
        }
    }
    let _restore = RestoreFetch;

    let root = document().create_element("div").unwrap();
    document().body().unwrap().append_child(&root).unwrap();
    let mut app = yew::Renderer::<App>::with_root_and_props(
        root.clone(),
        AppProps {
            second: 1,
            revision: 0,
        },
    )
    .render();

    wait_for(|| request_count() == 1).await;
    respond(0, "invalid JSON");
    wait_for(|| {
        text("first").as_deref() == Some("error") && text("second").as_deref() == Some("error")
    })
    .await;
    assert_eq!(request_count(), 1);

    click("first");
    click("first");
    click("second");
    wait_for(|| {
        text("first").as_deref() == Some("error refreshing")
            && text("second").as_deref() == Some("error refreshing")
    })
    .await;
    assert_eq!(request_count(), 2);
    respond(1, r#"{"ok": 10}"#);
    wait_for(|| text("first").as_deref() == Some("10") && text("second").as_deref() == Some("10"))
        .await;

    app.update(AppProps {
        second: 2,
        revision: 1,
    });
    wait_for(|| request_count() == 3).await;
    respond(2, r#"{"ok": 20}"#);
    wait_for(|| text("second").as_deref() == Some("20")).await;

    // The cache holds only input 2 now. Re-rendering the first hook keeps its result.
    app.update(AppProps {
        second: 2,
        revision: 2,
    });
    sleep(Duration::ZERO).await;
    assert_eq!(text("first").as_deref(), Some("10"));
    assert_eq!(request_count(), 3);

    click("first");
    wait_for(|| text("first").as_deref() == Some("10 refreshing") && request_count() == 4).await;
    respond(3, "invalid JSON");
    wait_for(|| text("first").as_deref() == Some("10")).await;
    assert_eq!(request_count(), 4);

    // A changed input is a new query and can fetch after eviction.
    app.update(AppProps {
        second: 1,
        revision: 3,
    });
    wait_for(|| request_count() == 5).await;
    respond(4, r#"{"ok": 11}"#);
    wait_for(|| text("second").as_deref() == Some("11")).await;
    assert_eq!(text("first").as_deref(), Some("10"));

    app.destroy();
    root.remove();
}
