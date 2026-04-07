use std::time::Duration;

use gloo::utils::document;
use wasm_bindgen::prelude::*;
use yew::Renderer;
use yew::html::BaseComponent;

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

fn performance() -> web_sys::Performance {
    web_sys::window().unwrap().performance().unwrap()
}

/// Counts completed network requests to URLs containing `needle` using the
/// Performance Resource Timing API. This works regardless of how the request
/// was initiated (gloo-net, window.fetch, XMLHttpRequest, etc.) because it
/// observes the browser's actual network activity.
pub fn resource_request_count(needle: &str) -> u32 {
    let entries = performance().get_entries_by_type("resource");
    let mut count = 0;
    for i in 0..entries.length() {
        let entry: web_sys::PerformanceEntry = entries.get(i).unchecked_into();
        if entry.name().contains(needle) {
            count += 1;
        }
    }
    count
}

/// Clears all resource timing entries so that subsequent calls to
/// [`resource_request_count`] only see new requests.
pub fn clear_resource_timings() {
    performance().clear_resource_timings();
}

/// Returns the text content of the first `<h1 class="title">` in the document.
pub fn get_title_text() -> Option<String> {
    document()
        .query_selector("h1.title")
        .ok()
        .flatten()
        .map(|el| el.text_content().unwrap_or_default())
}

/// Returns the text content of the first `.section.container` inside the
/// test output element.
pub fn post_body_text() -> String {
    output_element()
        .query_selector(".section.container")
        .ok()
        .flatten()
        .map(|el| el.text_content().unwrap_or_default())
        .unwrap_or_default()
}

/// Parses `html` into a detached container element and returns the text
/// content of the first element matching `selector`, if any.
pub fn extract_text_from_html(html: &str, selector: &str) -> Option<String> {
    let container = document().create_element("div").unwrap();
    container.set_inner_html(html);
    container
        .query_selector(selector)
        .ok()
        .flatten()
        .and_then(|el| el.text_content())
}

/// Shared e2e scenario used by the yew-link SSR router examples.
///
/// Phases:
/// 1. Directly visit `/posts/0` by fetching its SSR HTML, hydrate, and assert that hydration did
///    not trigger any fetch to `link_endpoint`.
/// 2. Click the "Posts" navbar link, then the post 0 card, and wait for the post page to render.
/// 3. Assert at least one fetch to `link_endpoint` happened during the client-side navigation and
///    that the rendered title/body match the original SSR HTML.
///
/// `make_renderer` is a closure that builds a `Renderer<COMP>` rooted at
/// [`output_element()`]. It is invoked after the SSR HTML has been injected
/// so that hydration picks it up.
pub async fn assert_ssr_hydration_and_client_navigation<COMP>(
    make_renderer: impl FnOnce() -> Renderer<COMP>,
    server_base: &str,
    link_endpoint: &str,
) where
    COMP: BaseComponent,
{
    // -- Part 1: Direct SSR visit to /posts/0 triggers no fetch to link_endpoint --

    let ssr_html = fetch_ssr_html(server_base, "/posts/0").await;
    let ssr_title = extract_text_from_html(&ssr_html, "h1.title")
        .expect("SSR HTML for /posts/0 should contain h1.title");
    let ssr_body = extract_text_from_html(&ssr_html, ".section.container").unwrap_or_default();

    clear_resource_timings();

    output_element().set_inner_html(&ssr_html);
    push_route("/posts/0");
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

    let link_fetches = resource_request_count(link_endpoint);
    let title = get_title_text();

    assert_eq!(
        link_fetches, 0,
        "direct SSR visit to /posts/0 should not trigger any fetch to {link_endpoint}"
    );
    let title = title.expect("h1.title should be present on the SSR post page");
    assert!(!title.is_empty(), "SSR post title should not be empty");

    // -- Part 2: Navigate to /posts within the same app, then to /posts/0 --

    yew::scheduler::flush().await;

    clear_resource_timings();

    let posts_link: web_sys::HtmlElement = output_element()
        .query_selector("a.navbar-item[href='/posts']")
        .unwrap()
        .expect("Posts navbar link should exist")
        .dyn_into()
        .unwrap();
    posts_link.click();
    yew::scheduler::flush().await;

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

    wait_for(
        || {
            output_element()
                .query_selector("a.title.is-block[href='/posts/0']")
                .ok()
                .flatten()
                .is_some()
        },
        15000,
        "post 0 card link on posts list",
    )
    .await;

    let post_link: web_sys::HtmlElement = output_element()
        .query_selector("a.title.is-block[href='/posts/0']")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    post_link.click();
    yew::scheduler::flush().await;

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

    let nav_link_fetches = resource_request_count(link_endpoint);
    let nav_title = get_title_text();
    let nav_body = post_body_text();

    assert!(
        nav_link_fetches >= 1,
        "client-side navigation to /posts/0 should trigger at least one fetch to {link_endpoint}, \
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
    yew::scheduler::flush().await;
}

/// Shared e2e scenario that asserts hydrating the home page ("/") produces
/// HTML containing the word "Welcome".
pub async fn assert_hydrate_home<COMP>(
    make_renderer: impl FnOnce() -> Renderer<COMP>,
    server_base: &str,
) where
    COMP: BaseComponent,
{
    setup_ssr_page(server_base, "/").await;
    let app = make_renderer().hydrate();

    wait_for(
        || output_element().inner_html().contains("Welcome"),
        5000,
        "home page content after hydration",
    )
    .await;

    app.destroy();
    yew::scheduler::flush().await;
}
