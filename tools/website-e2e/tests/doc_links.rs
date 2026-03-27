use std::path::PathBuf;
use std::time::Duration;

use fantoccini::{ClientBuilder, Locator};
use website_e2e::start_file_server;

fn build_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../yew-rs/build")
        .canonicalize()
        .expect("yew-rs/build directory not found; run `cargo run -p yew-site-ssg` first")
}

fn webdriver_url() -> String {
    std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".into())
}

async fn make_client() -> fantoccini::Client {
    for _ in 0..3 {
        let webdriver = webdriver_url();
        let mut caps = serde_json::Map::new();

        if std::env::var("HEADLESS").is_ok() {
            let args = serde_json::json!(["--headless", "--no-sandbox", "--disable-gpu"]);
            caps.insert(
                "goog:chromeOptions".into(),
                serde_json::json!({ "args": args }),
            );
            caps.insert(
                "moz:firefoxOptions".into(),
                serde_json::json!({ "args": ["-headless"] }),
            );
        }

        match ClientBuilder::native()
            .capabilities(caps)
            .connect(&webdriver)
            .await
        {
            Ok(c) => return c,
            Err(_) => {
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
    panic!("failed to connect to WebDriver after retries");
}

fn assert_path(url: &url::Url, expected: &str) {
    let actual = url.path().trim_end_matches('/');
    let expected = expected.trim_end_matches('/');
    assert_eq!(
        actual,
        expected,
        "expected path {expected}, got {}",
        url.path()
    );
}

async fn click_content_link_by_href(client: &fantoccini::Client, href_contains: &str) {
    let css = format!("main a[href*='{href_contains}']");
    let link = client.find(Locator::Css(&css)).await.unwrap_or_else(|_| {
        panic!("content link with href containing '{href_contains}' not found")
    });
    link.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(800)).await;
}

async fn wait_for_page(client: &fantoccini::Client) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    client.find(Locator::Css("main")).await.unwrap();
}

async fn assert_element_visible(client: &fantoccini::Client, css_selector: &str) {
    let js = format!(
        r#"
        var el = document.querySelector('{}');
        if (!el) return {{ found: false }};
        var rect = el.getBoundingClientRect();
        var viewportHeight = window.innerHeight || document.documentElement.clientHeight;
        return {{
            found: true,
            top: rect.top,
            bottom: rect.bottom,
            viewportHeight: viewportHeight
        }};
        "#,
        css_selector.replace('\'', "\\'")
    );
    let result: serde_json::Value = client.execute(&js, vec![]).await.unwrap();
    assert!(
        result["found"].as_bool().unwrap_or(false),
        "element '{css_selector}' not found in DOM"
    );
    let top = result["top"].as_f64().unwrap();
    let bottom = result["bottom"].as_f64().unwrap();
    let vh = result["viewportHeight"].as_f64().unwrap();
    assert!(
        top >= 0.0 && top < vh && bottom > 0.0,
        "element '{css_selector}' is not visible in viewport: top={top}, bottom={bottom}, \
         viewportHeight={vh}"
    );
}

#[tokio::test]
async fn doc_link_preserves_stable_version() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_preserves_old_version() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/0.21/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/0.21/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_preserves_next_version() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/next/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/next/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_preserves_locale() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/ja/docs/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_preserves_locale_and_version() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/ja/docs/0.22/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/0.22/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_preserves_zh_hans_locale_and_version() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/zh-Hans/docs/0.22/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "concepts/router").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/zh-Hans/docs/0.22/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_is_spa_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/more/deployment"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    let js = r#"
        window.__page_reloaded = false;
        window.addEventListener('beforeunload', () => { window.__page_reloaded = true; });
    "#;
    client.execute(js, vec![]).await.unwrap();

    click_content_link_by_href(&client, "concepts/router").await;

    let reloaded: serde_json::Value = client
        .execute("return window.__page_reloaded ?? true", vec![])
        .await
        .unwrap();
    assert_eq!(
        reloaded,
        serde_json::Value::Bool(false),
        "doc_link click should be SPA navigation (no page reload)"
    );

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/concepts/router");

    client.close().await.unwrap();
}

#[tokio::test]
async fn fragment_heading_visible_on_direct_visit() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!(
            "{base}/docs/advanced-topics/server-side-rendering#rendering-head-tags"
        ))
        .await
        .unwrap();
    wait_for_page(&client).await;

    assert_element_visible(&client, "#rendering-head-tags").await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn fragment_heading_visible_after_spa_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/advanced-topics/portals"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "server-side-rendering").await;
    tokio::time::sleep(Duration::from_millis(500)).await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/advanced-topics/server-side-rendering");

    assert_element_visible(&client, "#rendering-head-tags").await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_link_fragment_preserved() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/advanced-topics/portals"))
        .await
        .unwrap();
    wait_for_page(&client).await;

    click_content_link_by_href(&client, "server-side-rendering").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/advanced-topics/server-side-rendering");
    assert_eq!(
        url.fragment().unwrap_or(""),
        "rendering-head-tags",
        "fragment should be preserved"
    );

    client.close().await.unwrap();
}
