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

async fn find_nav_button_by_text(
    client: &fantoccini::Client,
    text: &str,
) -> fantoccini::elements::Element {
    let xpath = format!("//nav//button[contains(text(), '{text}')]");
    client
        .find(Locator::XPath(&xpath))
        .await
        .unwrap_or_else(|_| panic!("nav button containing '{text}' not found"))
}

async fn assert_version_selector(client: &fantoccini::Client, expected: &str) {
    let btn = find_nav_button_by_text(client, expected).await;
    let text = btn.text().await.unwrap();
    assert!(
        text.contains(expected),
        "version selector mismatch: expected '{expected}', got '{text}'"
    );
}

async fn assert_lang_selector(client: &fantoccini::Client, expected: &str) {
    let btn = find_nav_button_by_text(client, expected).await;
    let text = btn.text().await.unwrap();
    assert!(
        text.contains(expected),
        "language selector mismatch: expected '{expected}', got '{text}'"
    );
}

async fn find_lang_button(client: &fantoccini::Client) -> fantoccini::elements::Element {
    let known_langs = ["English", "日本語", "简体中文", "繁體中文"];
    let btns = client.find_all(Locator::Css("nav button")).await.unwrap();
    for btn in btns {
        let text = btn.text().await.unwrap();
        if known_langs.iter().any(|l| text.contains(l)) {
            return btn;
        }
    }
    panic!("language selector button not found");
}

async fn click_lang_option(client: &fantoccini::Client, label: &str) {
    let btn = find_lang_button(client).await;
    btn.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;

    let item = client
        .find(Locator::LinkText(label))
        .await
        .unwrap_or_else(|_| panic!("language option '{label}' not found"));
    item.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
}

async fn click_sidebar_category(client: &fantoccini::Client, label: &str) {
    let xpath = format!("//aside//a[normalize-space(text())='{label}']");
    let link = client
        .find(Locator::XPath(&xpath))
        .await
        .unwrap_or_else(|_| panic!("sidebar category '{label}' not found"));
    link.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
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

async fn assert_no_element_css(client: &fantoccini::Client, css: &str) {
    let els = client.find_all(Locator::Css(css)).await.unwrap();
    assert!(els.is_empty(), "expected no elements for: {css}");
}

async fn assert_no_element_xpath(client: &fantoccini::Client, xpath: &str) {
    let els = client.find_all(Locator::XPath(xpath)).await.unwrap();
    assert!(els.is_empty(), "expected no elements for xpath: {xpath}");
}

async fn click_version_option(client: &fantoccini::Client, label: &str) {
    // Find any nav button that looks like a version selector (contains a version-like text)
    // We look for all nav buttons and click the first one that's not a lang button
    let btns = client.find_all(Locator::Css("nav button")).await.unwrap();
    let known_langs = ["English", "日本語", "简体中文", "繁體中文"];
    let mut version_btn = None;
    for btn in btns {
        let text = btn.text().await.unwrap();
        let trimmed = text.trim();
        if !trimmed.is_empty()
            && !known_langs.iter().any(|l| trimmed.contains(l))
            && !trimmed.contains("Toggle")
            && !trimmed.contains("Search")
            && !trimmed.contains("menu")
        {
            version_btn = Some(btn);
            break;
        }
    }
    let btn = version_btn.expect("version selector button not found");
    btn.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let item = client
        .find(Locator::LinkText(label))
        .await
        .unwrap_or_else(|_| panic!("version option '{label}' not found"));
    item.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
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

    // Step 1: visit /docs/concepts/router (now 0.23, latest stable)
    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_version_selector(&client, "0.23").await;
    assert_lang_selector(&client, "English").await;

    // Step 2: expand version selector, verify options, click Next
    {
        let btn = find_nav_button_by_text(&client, "0.23").await;
        btn.click().await.unwrap();
        tokio::time::sleep(Duration::from_millis(200)).await;

        let items = client.find_all(Locator::Css("nav ul li a")).await.unwrap();
        let labels: Vec<String> = {
            let mut v = Vec::new();
            for item in &items {
                v.push(item.text().await.unwrap().trim().to_string());
            }
            v
        };
        assert!(
            labels.contains(&"Next".to_string()),
            "Next not in version options: {:?}",
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

        let next_link = client
            .find(Locator::LinkText("Next"))
            .await
            .expect("Next link not found in dropdown");
        next_link.click().await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // Step 3: should be at /docs/next/concepts/router
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/next/concepts/router");
    assert_version_selector(&client, "Next").await;
    assert_lang_selector(&client, "English").await;

    // Step 4: switch language to Japanese
    click_lang_option(&client, "日本語").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/next/concepts/router");
    assert_lang_selector(&client, "日本語").await;
    assert_version_selector(&client, "Next").await;

    let body_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        page_looks_japanese(&body_text),
        "page at /ja/docs/next/concepts/router does not look Japanese"
    );

    // Step 5: click "Getting Started" sidebar category (translated to Japanese)
    click_sidebar_category(&client, "はじめる").await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/next/getting-started");
    assert_version_selector(&client, "Next").await;
    assert_lang_selector(&client, "日本語").await;

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

    assert_lang_selector(&client, "简体中文").await;

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

    assert_lang_selector(&client, "繁體中文").await;

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

    assert_no_element_css(&client, r#"meta[name="docsearch:language"]"#).await;
    assert_no_element_css(&client, r#"meta[name="docsearch:version"]"#).await;
    assert_no_element_css(&client, r#"script[type="application/ld+json"]"#).await;

    // --- English docs page (0.23, latest stable) ---
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
        "0.23",
    )
    .await;

    assert_meta_exists(&client, r#"meta[name="description"]"#).await;
    assert_meta_exists(&client, r#"meta[property="og:description"]"#).await;
    assert_meta_exists(&client, r#"script[type="application/ld+json"]"#).await;

    // --- English Next docs page ---
    client
        .goto(&format!("{base}/docs/next/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "en").await;
    assert_meta_attr(
        &client,
        r#"meta[name="docsearch:version"]"#,
        "content",
        "next",
    )
    .await;

    // --- Japanese docs page (0.23, latest stable) ---
    client
        .goto(&format!("{base}/ja/docs/concepts/router"))
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
        "https://yew.rs/ja/docs/concepts/router",
    )
    .await;
    assert_meta_attr(
        &client,
        r#"link[rel="canonical"]"#,
        "href",
        "https://yew.rs/ja/docs/concepts/router",
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

    let full_hreflangs: &[&str] = &["en", "ja", "x-default", "zh-Hans", "zh-Hant"];
    let en_only: &[&str] = &["en", "x-default"];

    client.goto(&format!("{base}/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, full_hreflangs).await;

    client.goto(&format!("{base}/ja/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, full_hreflangs).await;

    client.goto(&format!("{base}/next/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, full_hreflangs).await;

    client.goto(&format!("{base}/ja/0.22/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, full_hreflangs).await;

    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, full_hreflangs).await;

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

    client.goto(&format!("{base}/blog")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_hreflang_set(&client, en_only).await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn home_page_versioned_urls_exist() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    assert_status(&base, "/", 200).await;
    assert_status(&base, "/ja/", 200).await;
    assert_status(&base, "/zh-Hans/", 200).await;
    assert_status(&base, "/zh-Hant/", 200).await;

    assert_status(&base, "/next/", 200).await;
    assert_status(&base, "/ja/next/", 200).await;
    assert_status(&base, "/zh-Hans/next/", 200).await;
    assert_status(&base, "/zh-Hant/next/", 200).await;

    assert_status(&base, "/0.22/", 200).await;
    assert_status(&base, "/ja/0.22/", 200).await;
    assert_status(&base, "/zh-Hans/0.22/", 200).await;
    assert_status(&base, "/zh-Hant/0.22/", 200).await;

    assert_status(&base, "/0.21/", 200).await;
    assert_status(&base, "/ja/0.21/", 200).await;

    assert_status(&base, "/0.20/", 200).await;
    assert_status(&base, "/ja/0.20/", 200).await;
}

#[tokio::test]
async fn tutorial_version_and_language_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    assert_status(&base, "/tutorial", 200).await;
    assert_status(&base, "/next/tutorial", 200).await;
    assert_status(&base, "/0.22/tutorial", 200).await;
    assert_status(&base, "/0.21/tutorial", 200).await;
    assert_status(&base, "/0.20/tutorial", 200).await;
    assert_status(&base, "/ja/tutorial", 200).await;
    assert_status(&base, "/ja/next/tutorial", 200).await;
    assert_status(&base, "/zh-Hans/tutorial", 200).await;
    assert_status(&base, "/zh-Hant/tutorial", 200).await;

    client.goto(&format!("{base}/tutorial")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_version_selector(&client, "0.23").await;
    assert_lang_selector(&client, "English").await;

    click_version_option(&client, "Next").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/next/tutorial");

    client.close().await.unwrap();
}

#[tokio::test]
async fn home_page_version_selector() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client.goto(&format!("{base}/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_version_selector(&client, "0.23").await;
    assert_lang_selector(&client, "English").await;

    assert_no_element_xpath(&client, "//span[contains(., 'Version:')]").await;
    assert_no_element_css(&client, "[role='alert']").await;

    click_version_option(&client, "Next").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/next");
    assert_version_selector(&client, "Next").await;
    assert_no_element_xpath(&client, "//span[contains(., 'Version:')]").await;
    assert_no_element_css(&client, "[role='alert']").await;

    click_lang_option(&client, "日本語").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/next");
    assert_version_selector(&client, "Next").await;
    assert_lang_selector(&client, "日本語").await;

    click_version_option(&client, "0.22").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/0.22");
    assert_version_selector(&client, "0.22").await;

    click_version_option(&client, "0.23").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja");
    assert_version_selector(&client, "0.23").await;

    client.close().await.unwrap();
}

#[tokio::test]
async fn home_page_learn_more_links() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client.goto(&format!("{base}/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let links = client.find_all(Locator::Css("section a")).await.unwrap();
    assert!(!links.is_empty(), "no Learn more links found");
    for link in &links {
        let href = link.attr("href").await.unwrap().unwrap();
        assert!(
            href.starts_with("/docs/"),
            "Learn more link should start with /docs/, got: {href}"
        );
        assert!(
            !href.contains("/docs/docs/"),
            "Learn more link has double /docs/: {href}"
        );
    }

    client.goto(&format!("{base}/ja/0.22/")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let links = client.find_all(Locator::Css("section a")).await.unwrap();
    for link in &links {
        let href = link.attr("href").await.unwrap().unwrap();
        assert!(
            href.starts_with("/ja/docs/0.22/"),
            "ja/0.22 Learn more link should start with /ja/docs/0.22/, got: {href}"
        );
    }

    client.close().await.unwrap();
}

#[tokio::test]
async fn home_page_seo() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    let full_hreflangs: &[&str] = &["en", "ja", "x-default", "zh-Hans", "zh-Hant"];

    struct HomeCase {
        path: &'static str,
        og_url: &'static str,
        canonical: &'static str,
        og_locale: &'static str,
        html_lang: &'static str,
    }

    let cases = [
        HomeCase {
            path: "/",
            og_url: "https://yew.rs/",
            canonical: "https://yew.rs/",
            og_locale: "en",
            html_lang: "en",
        },
        HomeCase {
            path: "/ja/",
            og_url: "https://yew.rs/ja/",
            canonical: "https://yew.rs/ja/",
            og_locale: "ja",
            html_lang: "ja",
        },
        HomeCase {
            path: "/zh-Hans/",
            og_url: "https://yew.rs/zh-Hans/",
            canonical: "https://yew.rs/zh-Hans/",
            og_locale: "zh_Hans",
            html_lang: "zh-Hans",
        },
        HomeCase {
            path: "/zh-Hant/",
            og_url: "https://yew.rs/zh-Hant/",
            canonical: "https://yew.rs/zh-Hant/",
            og_locale: "zh_Hant",
            html_lang: "zh-Hant",
        },
        HomeCase {
            path: "/next/",
            og_url: "https://yew.rs/next/",
            canonical: "https://yew.rs/next/",
            og_locale: "en",
            html_lang: "en",
        },
        HomeCase {
            path: "/ja/next/",
            og_url: "https://yew.rs/ja/next/",
            canonical: "https://yew.rs/ja/next/",
            og_locale: "ja",
            html_lang: "ja",
        },
        HomeCase {
            path: "/0.22/",
            og_url: "https://yew.rs/0.22/",
            canonical: "https://yew.rs/0.22/",
            og_locale: "en",
            html_lang: "en",
        },
        HomeCase {
            path: "/zh-Hans/0.22/",
            og_url: "https://yew.rs/zh-Hans/0.22/",
            canonical: "https://yew.rs/zh-Hans/0.22/",
            og_locale: "zh_Hans",
            html_lang: "zh-Hans",
        },
        HomeCase {
            path: "/0.21/",
            og_url: "https://yew.rs/0.21/",
            canonical: "https://yew.rs/0.21/",
            og_locale: "en",
            html_lang: "en",
        },
        HomeCase {
            path: "/ja/0.21/",
            og_url: "https://yew.rs/ja/0.21/",
            canonical: "https://yew.rs/ja/0.21/",
            og_locale: "ja",
            html_lang: "ja",
        },
        HomeCase {
            path: "/0.20/",
            og_url: "https://yew.rs/0.20/",
            canonical: "https://yew.rs/0.20/",
            og_locale: "en",
            html_lang: "en",
        },
        HomeCase {
            path: "/zh-Hant/0.20/",
            og_url: "https://yew.rs/zh-Hant/0.20/",
            canonical: "https://yew.rs/zh-Hant/0.20/",
            og_locale: "zh_Hant",
            html_lang: "zh-Hant",
        },
    ];

    for case in &cases {
        client.goto(&format!("{base}{}", case.path)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;

        assert_eq!(
            client.title().await.unwrap(),
            "Yew",
            "{}: title should be 'Yew'",
            case.path
        );

        assert_meta_attr(&client, "html", "lang", case.html_lang).await;
        assert_meta_attr(&client, r#"meta[property="og:title"]"#, "content", "Yew").await;
        assert_meta_attr(
            &client,
            r#"meta[property="og:url"]"#,
            "content",
            case.og_url,
        )
        .await;
        assert_meta_attr(&client, r#"link[rel="canonical"]"#, "href", case.canonical).await;
        assert_meta_attr(
            &client,
            r#"meta[property="og:locale"]"#,
            "content",
            case.og_locale,
        )
        .await;
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
        assert_hreflang_set(&client, full_hreflangs).await;

        assert_no_element_css(&client, r#"meta[name="docsearch:language"]"#).await;
        assert_no_element_css(&client, r#"meta[name="docsearch:version"]"#).await;
        assert_no_element_css(&client, r#"script[type="application/ld+json"]"#).await;
    }

    client.close().await.unwrap();
}

#[tokio::test]
async fn migration_guide_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    assert_status(
        &base,
        "/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;
    assert_status(
        &base,
        "/ja/docs/migration-guides/yew/from-0-22-0-to-0-23-0",
        200,
    )
    .await;

    client
        .goto(&format!(
            "{base}/docs/migration-guides/yew/from-0-22-0-to-0-23-0"
        ))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "en").await;
    assert_no_element_css(&client, "[role='alert']").await;
    assert_no_element_xpath(&client, "//span[contains(., 'Version:')]").await;

    click_lang_option(&client, "日本語").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/ja/docs/migration-guides/yew/from-0-22-0-to-0-23-0");

    let body_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        page_looks_japanese(&body_text),
        "Japanese migration guide page does not look Japanese"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn blog_pages_exist_and_render() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    assert_status(&base, "/blog", 200).await;
    assert_status(&base, "/blog/2022/01/20/hello-yew", 200).await;
    assert_status(&base, "/blog/2022/11/24/release-0-20", 200).await;
    assert_status(&base, "/blog/2023/09/23/release-0-21", 200).await;
    assert_status(&base, "/blog/2024/10/14/release-0-22", 200).await;
    assert_status(&base, "/blog/2025/11/29/release-0-22", 200).await;

    client.goto(&format!("{base}/blog")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_meta_attr(&client, "html", "lang", "en").await;
    assert_eq!(client.title().await.unwrap(), "Blog | Yew");

    let links = client.find_all(Locator::Css("main a")).await.unwrap();
    assert!(
        links.len() >= 3,
        "blog index should have at least 3 post links, found {}",
        links.len()
    );

    client
        .goto(&format!("{base}/blog/2022/01/20/hello-yew"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_eq!(client.title().await.unwrap(), "Hello Yew | Yew");
    assert_meta_attr(
        &client,
        r#"meta[property="og:url"]"#,
        "content",
        "https://yew.rs/blog/2022/01/20/hello-yew",
    )
    .await;

    let main_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        main_text.contains("Yew"),
        "blog post should mention Yew in body"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn blog_feeds_exist() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    let rss = reqwest::get(format!("{base}/blog/rss.xml")).await.unwrap();
    assert_eq!(rss.status().as_u16(), 200);
    let rss_body = rss.text().await.unwrap();
    assert!(
        rss_body.contains("<rss"),
        "rss.xml should contain <rss element"
    );
    assert!(
        rss_body.contains("hello-yew"),
        "rss.xml should reference hello-yew post"
    );

    let atom = reqwest::get(format!("{base}/blog/atom.xml")).await.unwrap();
    assert_eq!(atom.status().as_u16(), 200);
    let atom_body = atom.text().await.unwrap();
    assert!(
        atom_body.contains("<feed"),
        "atom.xml should contain <feed element"
    );
    assert!(
        atom_body.contains("hello-yew"),
        "atom.xml should reference hello-yew post"
    );
}

#[tokio::test]
async fn community_pages_exist_and_render() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    assert_status(&base, "/community/awesome", 200).await;
    assert_status(&base, "/community/external-libs", 200).await;

    client
        .goto(&format!("{base}/community/awesome"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    assert_eq!(client.title().await.unwrap(), "Awesome Yew | Yew");
    assert_meta_attr(&client, "html", "lang", "en").await;

    let main_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        main_text.contains("Projects") || main_text.contains("Templates"),
        "awesome page should have Projects or Templates section"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn search_page_exists() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    assert_status(&base, "/search", 200).await;
}

#[tokio::test]
async fn not_found_page_renders() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client.goto(&format!("{base}/404")).await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let body = client
        .find(Locator::Css("body"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        body.contains("Page Not Found") || body.contains("404"),
        "404 page should indicate page not found"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn all_doc_versions_serve_pages() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");

    let versions = [
        "/docs/getting-started",
        "/docs/next/getting-started",
        "/docs/0.22/getting-started",
        "/docs/0.21/getting-started",
        "/docs/0.20/getting-started",
    ];

    for path in &versions {
        assert_status(&base, path, 200).await;
    }

    let locales = ["ja", "zh-Hans", "zh-Hant"];
    for locale in &locales {
        for path in &versions {
            let localized = path.replacen("/docs/", &format!("/{locale}/docs/"), 1);
            assert_status(&base, &localized, 200).await;
        }
    }

    assert_status(&base, "/docs/concepts/router", 200).await;
    assert_status(&base, "/docs/next/concepts/router", 200).await;
    assert_status(&base, "/docs/0.22/concepts/router", 200).await;
    assert_status(&base, "/docs/0.21/concepts/router", 200).await;
    assert_status(&base, "/docs/0.20/concepts/router", 200).await;

    assert_status(&base, "/ja/docs/concepts/router", 200).await;
    assert_status(&base, "/zh-Hans/docs/concepts/router", 200).await;
    assert_status(&base, "/zh-Hant/docs/concepts/router", 200).await;

    assert_status(&base, "/docs/advanced-topics/server-side-rendering", 200).await;
    assert_status(&base, "/docs/more/css", 200).await;
    assert_status(&base, "/docs/concepts/html/elements", 200).await;
    assert_status(
        &base,
        "/docs/concepts/function-components/hooks/custom-hooks",
        200,
    )
    .await;
}

#[tokio::test]
async fn version_banner_on_old_versions() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_no_element_css(&client, "[role='alert']").await;

    client
        .goto(&format!("{base}/docs/0.22/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    let alerts = client
        .find_all(Locator::Css("[role='alert']"))
        .await
        .unwrap();
    assert!(
        !alerts.is_empty(),
        "old version (0.22) should show a version banner"
    );
    let alert_text = alerts[0].text().await.unwrap();
    assert!(
        alert_text.contains("0.22"),
        "version banner should mention the version, got: {alert_text}"
    );

    client
        .goto(&format!("{base}/docs/next/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;
    let alerts = client
        .find_all(Locator::Css("[role='alert']"))
        .await
        .unwrap();
    assert!(
        !alerts.is_empty(),
        "Next version should show a version banner"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn sidebar_structure_on_doc_pages() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/getting-started"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let aside = client.find_all(Locator::Css("aside")).await.unwrap();
    assert!(!aside.is_empty(), "doc page should have a sidebar (aside)");

    let sidebar_text = aside[0].text().await.unwrap();
    let expected_categories = ["Getting Started", "Concepts", "Advanced Topics", "More"];
    for cat in &expected_categories {
        assert!(
            sidebar_text.contains(cat),
            "sidebar should contain category '{cat}', got: {sidebar_text}"
        );
    }

    let sidebar_links = client.find_all(Locator::Css("aside a")).await.unwrap();
    assert!(
        sidebar_links.len() >= 5,
        "sidebar should have at least 5 links, found {}",
        sidebar_links.len()
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_page_has_breadcrumbs() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let breadcrumbs = client
        .find_all(Locator::Css("nav[aria-label='Breadcrumbs'] a"))
        .await
        .unwrap();
    assert!(
        !breadcrumbs.is_empty(),
        "doc page should have breadcrumb links"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_page_has_toc() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/concepts/router"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let asides = client.find_all(Locator::Css("aside")).await.unwrap();
    assert!(
        asides.len() >= 2,
        "doc page should have at least 2 aside elements (sidebar + TOC), found {}",
        asides.len()
    );
    let toc_aside = &asides[asides.len() - 1];
    let toc_links = toc_aside.find_all(Locator::Css("a")).await.unwrap();
    assert!(!toc_links.is_empty(), "TOC aside should have links");

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_page_has_code_blocks() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/getting-started/build-a-sample-app"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let code_blocks = client.find_all(Locator::Css("pre code")).await.unwrap();
    assert!(
        !code_blocks.is_empty(),
        "build-a-sample-app should have code blocks"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn spa_client_side_navigation() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/getting-started"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(1000)).await;

    let link = client
        .find(Locator::XPath(
            "//aside//a[contains(@href, '/docs/getting-started/build-a-sample-app')]",
        ))
        .await
        .expect("sidebar link to build-a-sample-app not found");
    link.click().await.unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/getting-started/build-a-sample-app");

    let main_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        main_text.len() > 100,
        "navigated page should have substantial content"
    );

    click_version_option(&client, "0.22").await;
    let url = client.current_url().await.unwrap();
    assert_path(&url, "/docs/0.22/getting-started/build-a-sample-app");

    let main_text = client
        .find(Locator::Css("main"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(
        main_text.len() > 100,
        "version-switched page should have substantial content"
    );

    client.close().await.unwrap();
}

#[tokio::test]
async fn doc_page_pagination() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    let client = make_client().await;

    client
        .goto(&format!("{base}/docs/getting-started"))
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(500)).await;

    let pagination = client
        .find_all(Locator::XPath(
            "//main//nav//a[.//span[text()='Previous'] or .//span[text()='Next']]",
        ))
        .await
        .unwrap();
    assert!(
        !pagination.is_empty(),
        "doc page should have pagination links (Previous/Next)"
    );

    client.close().await.unwrap();
}
