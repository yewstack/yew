use std::path::PathBuf;
use std::time::Duration;

use fantoccini::{ClientBuilder, Locator};
use website_e2e::{page_looks_chinese, page_looks_japanese, start_file_server};

async fn assert_status(base: &str, path: &str, expected: u16) {
    let url = format!("{base}{path}");
    let resp = reqwest::get(&url).await.unwrap();
    assert_eq!(
        resp.status().as_u16(),
        expected,
        "{path} returned {} (expected {expected})",
        resp.status()
    );
}

fn build_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../website/build")
        .canonicalize()
        .expect("website/build directory not found; run `cargo run -p yew-site-ssg` first")
}

fn webdriver_url() -> String {
    std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".into())
}

async fn make_client() -> fantoccini::Client {
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

    ClientBuilder::native()
        .capabilities(caps)
        .connect(&webdriver)
        .await
        .expect("failed to connect to WebDriver")
}

async fn assert_version_selector(client: &fantoccini::Client, expected: &str) {
    let btn = client
        .find(Locator::Css(
            ".items > .dropdown:nth-of-type(1) .dropdown-btn",
        ))
        .await
        .expect("version selector not found");
    let text = btn.text().await.unwrap();
    assert_eq!(text.trim(), expected, "version selector mismatch");
}

async fn assert_lang_selector(client: &fantoccini::Client, expected: &str) {
    let btn = client
        .find(Locator::Css(
            ".items > .dropdown:nth-of-type(2) .dropdown-btn",
        ))
        .await
        .expect("language selector not found");
    let text = btn.text().await.unwrap();
    assert_eq!(text.trim(), expected, "language selector mismatch");
}

async fn click_lang_option(client: &fantoccini::Client, label: &str) {
    let btn = client
        .find(Locator::Css(
            ".items > .dropdown:nth-of-type(2) .dropdown-btn",
        ))
        .await
        .unwrap();
    btn.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;

    let items = client
        .find_all(Locator::Css(
            ".items > .dropdown:nth-of-type(2) .dropdown-item",
        ))
        .await
        .unwrap();
    for item in items {
        if item.text().await.unwrap().trim() == label {
            item.click().await.unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
            return;
        }
    }
    panic!("language option '{}' not found", label);
}

async fn click_sidebar_category(client: &fantoccini::Client, label: &str) {
    let links = client
        .find_all(Locator::Css(".cat-label--link"))
        .await
        .unwrap();
    for link in links {
        if link.text().await.unwrap().trim() == label {
            link.click().await.unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
            return;
        }
    }
    panic!("sidebar category '{}' not found", label);
}

fn assert_path(url: &::url::Url, expected: &str) {
    let actual = url.path().trim_end_matches('/');
    let expected = expected.trim_end_matches('/');
    assert_eq!(
        actual,
        expected,
        "expected path {expected}, got {}",
        url.path()
    );
}

async fn assert_meta_attr(client: &fantoccini::Client, css: &str, attr: &str, expected: &str) {
    let el = client
        .find(Locator::Css(css))
        .await
        .unwrap_or_else(|_| panic!("element not found: {css}"));
    let val = el
        .attr(attr)
        .await
        .unwrap()
        .unwrap_or_else(|| panic!("attribute '{attr}' missing on {css}"));
    assert_eq!(val, expected, "{css} [{attr}] mismatch");
}

async fn assert_meta_exists(client: &fantoccini::Client, css: &str) {
    let els = client.find_all(Locator::Css(css)).await.unwrap();
    assert!(!els.is_empty(), "expected at least one element: {css}");
}

async fn assert_no_element(client: &fantoccini::Client, css: &str) {
    let els = client.find_all(Locator::Css(css)).await.unwrap();
    assert!(els.is_empty(), "expected no elements for: {css}");
}

async fn assert_hreflang_set(client: &fantoccini::Client, expected: &[&str]) {
    let els = client
        .find_all(Locator::Css("link[hreflang]"))
        .await
        .unwrap();
    let mut langs: Vec<String> = Vec::new();
    for el in &els {
        if let Some(v) = el.attr("hreflang").await.unwrap() {
            langs.push(v);
        }
    }
    langs.sort();
    let mut expected: Vec<&str> = expected.to_vec();
    expected.sort();
    assert_eq!(langs, expected, "hreflang set mismatch");
}

#[tokio::test]
async fn version_and_language_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    // Step 1: visit /docs/concepts/router
    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_version_selector(&client, "Next").await;
    assert_lang_selector(&client, "English").await;

    // Step 2: expand version selector, verify options, click 0.23
    {
        let btn = client
            .find(Locator::Css(
                ".items > .dropdown:nth-of-type(1) .dropdown-btn",
            ))
            .await
            .unwrap();
        btn.click().await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;

        let items = client
            .find_all(Locator::Css(
                ".items > .dropdown:nth-of-type(1) .dropdown-item",
            ))
            .await
            .unwrap();
        let labels: Vec<String> = {
            let mut v = Vec::new();
            for item in &items {
                v.push(item.text().await.unwrap().trim().to_string());
            }
            v
        };
        assert!(
            labels.contains(&"0.23".to_string()),
            "0.23 not in version options: {:?}",
            labels
        );
        assert!(
            labels.contains(&"0.22".to_string()),
            "0.22 not in version options: {:?}",
            labels
        );
        assert!(
            labels.contains(&"0.21".to_string()),
            "0.21 not in version options: {:?}",
            labels
        );
        assert!(
            labels.contains(&"0.20".to_string()),
            "0.20 not in version options: {:?}",
            labels
        );

        for item in items {
            if item.text().await.unwrap().trim() == "0.23" {
                item.click().await.unwrap();
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Step 3: should be at /docs/0.23/concepts/router
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/0.23/concepts/router");
    assert_version_selector(&client, "0.23").await;
    assert_lang_selector(&client, "English").await;

    // Step 4: switch language to Japanese
    click_lang_option(&client, "\u{65e5}\u{672c}\u{8a9e}").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/0.23/concepts/router");
    assert_lang_selector(&client, "\u{65e5}\u{672c}\u{8a9e}").await;
    assert_version_selector(&client, "0.23").await;

    let body_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        page_looks_japanese(&body_text),
        "page at /ja/docs/0.23/concepts/router does not look Japanese"
    );

    // Step 5: click "Getting Started" sidebar category
    click_sidebar_category(&client, "Getting Started").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/0.23/getting-started");
    assert_version_selector(&client, "0.23").await;
    assert_lang_selector(&client, "\u{65e5}\u{672c}\u{8a9e}").await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn chinese_pages_look_chinese() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/zh-Hans/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_lang_selector(&client, "\u{7b80}\u{4f53}\u{4e2d}\u{6587}").await;

    let body_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        page_looks_chinese(&body_text),
        "zh-Hans page does not look Chinese"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn traditional_chinese_pages_look_chinese() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/zh-Hant/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_lang_selector(&client, "\u{7e41}\u{9ad4}\u{4e2d}\u{6587}").await;

    let body_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        page_looks_chinese(&body_text),
        "zh-Hant page does not look Chinese"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn category_urls_serve_introduction_pages() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    assert_status(&base, "/docs/concepts/function-components", 200).await;
    assert_status(
        &base,
        "/docs/concepts/function-components/introduction",
        404,
    )
    .await;

    assert_status(&base, "/docs/concepts/function-components/hooks", 200).await;
    assert_status(
        &base,
        "/docs/concepts/function-components/hooks/introduction",
        404,
    )
    .await;

    assert_status(&base, "/docs/concepts/html", 200).await;
    assert_status(&base, "/docs/concepts/html/introduction", 404).await;

    assert_status(&base, "/docs/getting-started", 200).await;
    assert_status(&base, "/docs/getting-started/introduction", 404).await;

    assert_status(&base, "/docs/advanced-topics/struct-components", 200).await;
    assert_status(
        &base,
        "/docs/advanced-topics/struct-components/introduction",
        404,
    )
    .await;
}

#[tokio::test]
async fn migration_guides_are_unversioned() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    // Unversioned English URLs return 200
    assert_status(
        &base,
        "/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/docs/migration-guides/yew/from-0-19-0-to-0-20-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/docs/migration-guides/yew-agent/from-0-4-0-to-0-5-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/docs/migration-guides/yew-router/from-0-19-0-to-0-20-0",
        200,
    )
    .await;

    // Versioned URLs return 404
    assert_status(
        &base,
        "/docs/0.23/migration-guides/yew/from-0-22-0-to-0-23-0",
        404,
    )
    .await;
    assert_status(
        &base,
        "/docs/0.20/migration-guides/yew/from-0-19-0-to-0-20-0",
        404,
    )
    .await;

    // i18n unversioned URLs return 200
    assert_status(
        &base,
        "/ja/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/zh-Hans/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/zh-Hant/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;

    // i18n versioned URLs return 404
    assert_status(
        &base,
        "/ja/docs/0.23/migration-guides/yew/from-0-22-0-to-0-23-0",
        404,
    )
    .await;
}

#[tokio::test]
async fn head_meta_tags() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    // --- Homepage ---
    client.goto(&format!("{base}/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "en").await;

    assert_eq!(
        client.title().await.unwrap(),
        "Yew",
        "homepage title mismatch"
    );

    assert_meta_attr(&client, r#"meta[property="og:title"]"#, "content", "Yew").await;
    assert_meta_attr(
        &client,
        r#"meta[property="og:url"]"#,
        "content",
        "https://yew.rs/",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"link[rel="canonical"]"#,
        "href",
        "https://yew.rs/",
    )
    .await;
    assert_meta_attr(&client, r#"meta[property="og:locale"]"#, "content", "en").await;
    assert_meta_attr(
        &client,
        r#"meta[name="twitter:card"]"#,
        "content",
        "summary_large_image",
    )
    .await;
    assert_meta_attr(&client, r#"link[rel="icon"]"#, "href", "/img/logo.svg").await;

    assert_meta_exists(&client, r#"meta[name="description"]"#).await;
    assert_meta_exists(&client, r#"meta[property="og:description"]"#).await;
    assert_meta_exists(&client, r#"link[type="application/rss+xml"]"#).await;
    assert_meta_exists(&client, r#"link[type="application/atom+xml"]"#).await;
    assert_meta_exists(&client, r#"link[rel="search"]"#).await;

    assert_no_element(&client, r#"meta[name="docsearch:language"]"#).await;
    assert_no_element(&client, r#"meta[name="docsearch:version"]"#).await;
    assert_no_element(&client, r#"script[type="application/ld+json"]"#).await;

    // --- English docs page ---
    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "en").await;

    assert_eq!(
        client.title().await.unwrap(),
        "Router | Yew",
        "docs title mismatch"
    );

    assert_meta_attr(
        &client,
        r#"meta[property="og:title"]"#,
        "content",
        "Router | Yew",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"meta[property="og:url"]"#,
        "content",
        "https://yew.rs/docs/concepts/router",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"link[rel="canonical"]"#,
        "href",
        "https://yew.rs/docs/concepts/router",
    )
    .await;
    assert_meta_attr(&client, r#"meta[property="og:locale"]"#, "content", "en").await;
    assert_meta_attr(
        &client,
        r#"meta[name="docsearch:language"]"#,
        "content",
        "en",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"meta[name="docsearch:version"]"#,
        "content",
        "next",
    )
    .await;

    assert_meta_exists(&client, r#"meta[name="description"]"#).await;
    assert_meta_exists(&client, r#"meta[property="og:description"]"#).await;
    assert_meta_exists(&client, r#"script[type="application/ld+json"]"#).await;

    // --- Japanese versioned docs page ---
    client
        .goto(&format!("{base}/ja/docs/0.23/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "ja").await;

    assert_eq!(
        client.title().await.unwrap(),
        "Router | Yew",
        "ja docs title mismatch"
    );

    assert_meta_attr(
        &client,
        r#"meta[property="og:title"]"#,
        "content",
        "Router | Yew",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"meta[property="og:url"]"#,
        "content",
        "https://yew.rs/ja/docs/0.23/concepts/router",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"link[rel="canonical"]"#,
        "href",
        "https://yew.rs/ja/docs/0.23/concepts/router",
    )
    .await;
    assert_meta_attr(&client, r#"meta[property="og:locale"]"#, "content", "ja").await;
    assert_meta_attr(
        &client,
        r#"meta[name="docsearch:language"]"#,
        "content",
        "ja",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"meta[name="docsearch:version"]"#,
        "content",
        "0.23",
    )
    .await;

    assert_meta_exists(&client, r#"meta[name="description"]"#).await;
    assert_meta_exists(&client, r#"script[type="application/ld+json"]"#).await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn hreflang_tags() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    // Homepage (non-docs): only en + x-default
    client.goto(&format!("{base}/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, &["en", "x-default"]).await;

    // Docs page: all languages + x-default
    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, &["en", "ja", "x-default", "zh-Hans", "zh-Hant"]).await;

    let xdefault = client
        .find(Locator::Css(r#"link[hreflang="x-default"]"#))
        .await
        .unwrap();
    let href = xdefault.attr("href").await.unwrap().unwrap();
    assert!(
        href.contains("/docs/concepts/router"),
        "x-default href should point to docs path, got: {href}"
    );
    assert!(
        !href.contains("/ja/") && !href.contains("/zh-"),
        "x-default href should not have locale prefix, got: {href}"
    );

    // Blog (non-docs): only en + x-default
    client.goto(&format!("{base}/blog")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, &["en", "x-default"]).await;

    client.close().await.unwrap();
}
