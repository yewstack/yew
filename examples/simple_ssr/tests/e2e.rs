#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use simple_ssr::App;
use ssr_e2e_harness::{fetch_ssr_html, output_element, wait_for};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SERVER_BASE: &str = "http://127.0.0.1:8080";

#[wasm_bindgen_test]
async fn hydration_succeeds() {
    let body_html = fetch_ssr_html(SERVER_BASE, "/").await;
    output_element().set_inner_html(&body_html);
    yew::Renderer::<App>::with_root(output_element()).hydrate();

    wait_for(
        || {
            let html = output_element().inner_html();
            html.contains("Random UUID:")
        },
        5000,
        "SSR content with UUID",
    )
    .await;

    let html = output_element().inner_html();
    assert!(
        html.contains("Random UUID:"),
        "hydrated content should contain the UUID text"
    );
}
