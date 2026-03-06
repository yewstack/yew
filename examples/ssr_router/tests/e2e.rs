#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use std::time::Duration;

use function_router::App;
use gloo::utils::document;
use ssr_e2e_harness::{output_element, setup_ssr_page, wait_for};
use wasm_bindgen_test::*;
use yew::platform::time::sleep;

wasm_bindgen_test_configure!(run_in_browser);

const SERVER_BASE: &str = "http://127.0.0.1:8080";

fn get_title_text() -> Option<String> {
    document()
        .query_selector("h1.title")
        .ok()
        .flatten()
        .map(|el| el.text_content().unwrap_or_default())
}

#[wasm_bindgen_test]
async fn hydrate_post_page() {
    setup_ssr_page(SERVER_BASE, "/posts/0").await;
    yew::Renderer::<App>::with_root(output_element()).hydrate();

    wait_for(
        || {
            let html = output_element().inner_html();
            html.contains("<h1 class=\"title\">") && !html.contains("Loading post...")
        },
        5000,
        "post page content",
    )
    .await;

    let title = get_title_text().expect("h1.title should be present on the post page");
    assert!(!title.is_empty(), "post title should not be empty");
}

#[wasm_bindgen_test]
async fn hydrate_posts_list() {
    setup_ssr_page(SERVER_BASE, "/posts").await;
    yew::Renderer::<App>::with_root(output_element()).hydrate();

    wait_for(
        || {
            document()
                .query_selector("a.title.is-block")
                .ok()
                .flatten()
                .is_some()
        },
        10000,
        "post links to appear on /posts",
    )
    .await;
}

#[wasm_bindgen_test]
async fn hydrate_home() {
    setup_ssr_page(SERVER_BASE, "/").await;
    yew::Renderer::<App>::with_root(output_element()).hydrate();

    sleep(Duration::from_secs(2)).await;
    let html = output_element().inner_html();
    assert!(
        html.contains("Welcome"),
        "home page should have content after hydration"
    );
}
