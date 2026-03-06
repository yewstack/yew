use std::time::Duration;

use gloo::utils::document;
use js_sys::{Array, Function, Reflect};
use wasm_bindgen::prelude::*;

/// Returns the `<div id="output">` element used by wasm-bindgen-test as the
/// test output container.
pub fn output_element() -> web_sys::Element {
    document().get_element_by_id("output").unwrap()
}

/// Fetches an SSR-rendered page from the server and returns the inner HTML
/// between `<body>` and `</body>`.
pub async fn fetch_ssr_html(server_base: &str, path: &str) -> String {
    let url = format!("{server_base}{path}");
    let resp = gloo::net::http::Request::get(&url)
        .send()
        .await
        .expect("failed to fetch SSR page");
    let html = resp.text().await.expect("failed to read SSR response body");
    let body_start = html.find("<body>").expect("no <body> in SSR HTML") + "<body>".len();
    let body_end = html.find("</body>").expect("no </body> in SSR HTML");
    html[body_start..body_end].to_string()
}

/// Polls `predicate` every 100ms until it returns `true` or `timeout_ms`
/// elapses, in which case it panics with `desc`.
pub async fn wait_for<F: Fn() -> bool>(predicate: F, timeout_ms: u64, desc: &str) {
    let step = Duration::from_millis(100);
    let mut elapsed = Duration::ZERO;
    let timeout = Duration::from_millis(timeout_ms);
    while elapsed < timeout {
        if predicate() {
            return;
        }
        gloo::timers::future::sleep(step).await;
        elapsed += step;
    }
    panic!("{desc} did not become true within {timeout_ms}ms");
}

/// Fetches SSR HTML for `path`, injects it into the output element, and
/// pushes the route onto the browser history stack.
pub async fn setup_ssr_page(server_base: &str, path: &str) {
    let body_html = fetch_ssr_html(server_base, path).await;
    output_element().set_inner_html(&body_html);
    push_route(path);
}

/// Pushes a new entry onto the browser's history stack without navigation.
pub fn push_route(path: &str) {
    web_sys::window()
        .unwrap()
        .history()
        .unwrap()
        .push_state_with_url(&JsValue::NULL, "", Some(path))
        .unwrap();
}

fn window_js() -> JsValue {
    web_sys::window().unwrap().into()
}

fn get_prop(key: &str) -> JsValue {
    Reflect::get(&window_js(), &JsValue::from_str(key)).unwrap()
}

fn set_prop(key: &str, val: &JsValue) {
    Reflect::set(&window_js(), &JsValue::from_str(key), val).unwrap();
}

fn delete_prop(key: &str) {
    Reflect::delete_property(&window_js().into(), &JsValue::from_str(key)).unwrap();
}

/// Monkey-patches `window.fetch` to record all calls in
/// `window.__fetchCalls`.
pub fn patch_fetch() {
    let original: Function = get_prop("fetch").into();
    let calls = Array::new();

    set_prop("__originalFetch", &original);
    set_prop("__fetchCalls", &calls);

    let wrapper = Closure::<dyn Fn(JsValue, JsValue) -> JsValue>::new(
        move |input: JsValue, init: JsValue| {
            let args = Array::new();
            args.push(&input);
            if !init.is_undefined() {
                args.push(&init);
            }
            get_prop("__fetchCalls")
                .unchecked_ref::<Array>()
                .push(&args);
            Reflect::apply(&original, &window_js(), &args).unwrap()
        },
    );

    set_prop("fetch", wrapper.as_ref().unchecked_ref());
    wrapper.forget();
}

/// Returns the number of fetch calls recorded since the last `patch_fetch`.
pub fn fetch_count() -> u32 {
    get_prop("__fetchCalls").unchecked_ref::<Array>().length()
}

/// Restores the original `window.fetch` and removes the call log.
pub fn restore_fetch() {
    set_prop("fetch", &get_prop("__originalFetch"));
    delete_prop("__originalFetch");
    delete_prop("__fetchCalls");
}
