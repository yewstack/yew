#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use std::time::Duration;

use function_router::App;
use gloo::utils::document;
use ssr_e2e_harness::{fetch_ssr_html, output_element, push_route, wait_for};
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
    let body_html = fetch_ssr_html(SERVER_BASE, "/posts/0").await;

    output_element().set_inner_html(&body_html);
    push_route("/posts/0");
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
    let body_html = fetch_ssr_html(SERVER_BASE, "/posts").await;

    output_element().set_inner_html(&body_html);
    push_route("/posts");
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
    let body_html = fetch_ssr_html(SERVER_BASE, "/").await;

    output_element().set_inner_html(&body_html);
    push_route("/");
    yew::Renderer::<App>::with_root(output_element()).hydrate();

    sleep(Duration::from_secs(2)).await;
    let html = output_element().inner_html();
    assert!(
        html.contains("Welcome"),
        "home page should have content after hydration"
    );
}
