use std::time::Duration;

use fantoccini::Locator;
use website_e2e::{
    assert_nav_button, assert_path, build_dir, make_client, start_file_server, wait_for_page,
};

async fn click_content_link_by_href(client: &fantoccini::Client, href_contains: &str) {
    let css = format!("main a[href*='{href_contains}']");
    let link = client.find(Locator::Css(&css)).await.unwrap_or_else(|_| {
        panic!("content link with href containing '{href_contains}' not found")
    });
    link.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(800)).await;
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

async fn assert_tutorial_reached(
    client: &fantoccini::Client,
    expected_path: &str,
    expected_version: &str,
    expected_lang: &str,
) {
    assert_path(&client.current_url().await.unwrap(), expected_path);

    let main = client.find(Locator::Css("main")).await.unwrap();
    let h1 = main.find(Locator::Css("h1")).await.unwrap();
    let text = h1.text().await.unwrap();
    assert!(
        text.contains("Tutorial"),
        "expected tutorial page at {expected_path}, got h1: '{text}'"
    );

    assert_nav_button(client, expected_version).await;
    assert_nav_button(client, expected_lang).await;
}

#[tokio::test]
async fn tutorial_links_version_and_locale_aware() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    let cases: &[(&str, &str, &str, &str)] = &[
        ("/docs/getting-started", "/tutorial", "0.23", "English"),
        (
            "/docs/next/getting-started",
            "/next/tutorial",
            "Next",
            "English",
        ),
        (
            "/docs/0.22/getting-started",
            "/0.22/tutorial",
            "0.22",
            "English",
        ),
        (
            "/docs/0.21/getting-started",
            "/0.21/tutorial",
            "0.21",
            "English",
        ),
        ("/ja/docs/getting-started", "/ja/tutorial", "0.23", "日本語"),
        (
            "/ja/docs/0.20/getting-started",
            "/ja/0.20/tutorial",
            "0.20",
            "日本語",
        ),
        (
            "/zh-Hans/docs/0.21/getting-started",
            "/zh-Hans/0.21/tutorial",
            "0.21",
            "简体中文",
        ),
    ];

    for &(start, expected_path, expected_version, expected_lang) in cases {
        client.goto(&format!("{base}{start}")).await.unwrap();
        wait_for_page(&client).await;

        click_content_link_by_href(&client, "tutorial").await;
        wait_for_page(&client).await;

        assert_tutorial_reached(&client, expected_path, expected_version, expected_lang).await;
    }

    client.close().await.unwrap();
}
