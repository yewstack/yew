use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use axum::Router;
use fantoccini::{ClientBuilder, Locator};
use tower_http::services::ServeDir;

pub async fn start_file_server(build_dir: &Path) -> SocketAddr {
    let app = Router::new().fallback_service(ServeDir::new(build_dir));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    addr
}

pub fn build_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../yew-rs/build")
        .canonicalize()
        .expect("yew-rs/build directory not found; run `cargo run -p yew-site-ssg` first")
}

pub async fn make_client() -> fantoccini::Client {
    let webdriver =
        std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".into());
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

    for _ in 0..3 {
        match ClientBuilder::native()
            .capabilities(caps.clone())
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

pub async fn wait_for_page(client: &fantoccini::Client) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    client.find(Locator::Css("main")).await.unwrap();
}

pub fn assert_path(url: &url::Url, expected: &str) {
    let actual = url.path().trim_end_matches('/');
    let expected = expected.trim_end_matches('/');
    assert_eq!(
        actual,
        expected,
        "expected path {expected}, got {}",
        url.path()
    );
}

pub async fn find_nav_button(
    client: &fantoccini::Client,
    text: &str,
) -> fantoccini::elements::Element {
    let xpath = format!("//nav//button[contains(text(), '{text}')]");
    client
        .find(Locator::XPath(&xpath))
        .await
        .unwrap_or_else(|_| panic!("nav button containing '{text}' not found"))
}

pub async fn assert_nav_button(client: &fantoccini::Client, text: &str) {
    find_nav_button(client, text).await;
}

pub fn page_looks_japanese(text: &str) -> bool {
    let kana_count = text
        .chars()
        .filter(|c| ('\u{3040}'..='\u{309F}').contains(c) || ('\u{30A0}'..='\u{30FF}').contains(c))
        .count();
    kana_count >= 50
}

pub fn page_looks_chinese(text: &str) -> bool {
    let cjk_count = text
        .chars()
        .filter(|c| ('\u{4E00}'..='\u{9FFF}').contains(c))
        .count();
    cjk_count >= 50
}
