#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use gloo::utils::document;
use ssr_e2e_harness::{
    clear_resource_timings, fetch_ssr_html, navigate, output_element, resource_request_count,
    setup_ssr_page, wait_for,
};
use ssr_router::{App, AppProps, LINK_ENDPOINT};
use wasm_bindgen_test::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

const SERVER_BASE: &str = "http://127.0.0.1:8080";

fn endpoint() -> String {
    format!("{SERVER_BASE}{LINK_ENDPOINT}")
}

fn make_renderer() -> Renderer<App> {
    Renderer::<App>::with_root_and_props(
        output_element(),
        AppProps {
            endpoint: endpoint().into(),
        },
    )
}

fn get_title_text() -> Option<String> {
    document()
        .query_selector("h1.title")
        .ok()
        .flatten()
        .map(|el| el.text_content().unwrap_or_default())
}

fn post_body_text() -> String {
    output_element()
        .query_selector(".section.container")
        .ok()
        .flatten()
        .map(|el| el.text_content().unwrap_or_default())
        .unwrap_or_default()
}

fn extract_text_from_html(html: &str, selector: &str) -> Option<String> {
    let container = document().create_element("div").unwrap();
    container.set_inner_html(html);
    container
        .query_selector(selector)
        .ok()
        .flatten()
        .and_then(|el| el.text_content())
}

#[wasm_bindgen_test]
async fn ssr_hydration_and_client_navigation() {
    // -- Part 1: Direct SSR visit to /posts/0 triggers no fetch to /api/link --

    let ssr_html = fetch_ssr_html(SERVER_BASE, "/posts/0").await;
    let ssr_title = extract_text_from_html(&ssr_html, "h1.title")
        .expect("SSR HTML for /posts/0 should contain h1.title");
    let ssr_body = extract_text_from_html(&ssr_html, ".section.container").unwrap_or_default();

    clear_resource_timings();

    output_element().set_inner_html(&ssr_html);
    ssr_e2e_harness::push_route("/posts/0");
    let app = make_renderer().hydrate();

    wait_for(
        || {
            let html = output_element().inner_html();
            html.contains("<h1 class=\"title\">") && !html.contains("Loading post...")
        },
        5000,
        "post page content after SSR hydration",
    )
    .await;

    let link_fetches = resource_request_count(LINK_ENDPOINT);
    let title = get_title_text();

    assert_eq!(
        link_fetches, 0,
        "direct SSR visit to /posts/0 should not trigger any fetch to {LINK_ENDPOINT}"
    );
    let title = title.expect("h1.title should be present on the SSR post page");
    assert!(!title.is_empty(), "SSR post title should not be empty");

    // -- Part 2: Navigate to /posts within the same app, then to /posts/0 --

    // Yield to ensure effects (router history listener) are registered.
    gloo::timers::future::sleep(std::time::Duration::from_millis(500)).await;

    clear_resource_timings();

    // Navigate to /posts first, then to /posts/0 to trigger a client-side fetch.
    navigate("/posts");

    wait_for(
        || {
            document()
                .query_selector("a.title.is-block")
                .ok()
                .flatten()
                .is_some()
                && get_title_text().as_deref() == Some("Posts")
        },
        15000,
        "posts list after client-side navigation to /posts",
    )
    .await;

    clear_resource_timings();

    navigate("/posts/0");

    wait_for(
        || {
            document()
                .query_selector("h2.subtitle")
                .ok()
                .flatten()
                .map(|el| el.text_content().unwrap_or_default())
                .is_some_and(|text| text.starts_with("by "))
        },
        15000,
        "post page content after client-side navigation to /posts/0",
    )
    .await;

    // -- Part 3: Verify fetch happened and content matches SSR --

    let nav_link_fetches = resource_request_count(LINK_ENDPOINT);
    let nav_title = get_title_text();
    let nav_body = post_body_text();

    assert!(
        nav_link_fetches >= 1,
        "client-side navigation to /posts/0 should trigger at least one fetch to {LINK_ENDPOINT}, \
         got {nav_link_fetches}"
    );

    let nav_title = nav_title.expect("h1.title should be present after client-side navigation");
    assert_eq!(
        ssr_title, nav_title,
        "post title should match between SSR and client-side navigation"
    );
    assert_eq!(
        ssr_body, nav_body,
        "post body should match between SSR and client-side navigation"
    );

    app.destroy();
    output_element().set_inner_html("");
}

#[wasm_bindgen_test]
async fn hydrate_home() {
    setup_ssr_page(SERVER_BASE, "/").await;
    let app = make_renderer().hydrate();

    wait_for(
        || output_element().inner_html().contains("Welcome"),
        5000,
        "home page content after hydration",
    )
    .await;

    app.destroy();
    output_element().set_inner_html("");
}
