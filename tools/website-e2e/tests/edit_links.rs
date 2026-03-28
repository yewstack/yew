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

async fn wait_for_page(client: &fantoccini::Client) {
    tokio::time::sleep(Duration::from_millis(1000)).await;
    client.find(Locator::Css("main")).await.unwrap();
}

const GITHUB_BASE: &str = "https://github.com/yewstack/yew/blob/master/yew-rs";

async fn get_edit_link_href(client: &fantoccini::Client) -> String {
    let link = client
        .find(Locator::XPath("//a[contains(., 'Edit this page')]"))
        .await
        .expect("'Edit this page' link not found");
    link.attr("href")
        .await
        .unwrap()
        .expect("edit link has no href")
}

#[tokio::test]
async fn edit_link_stable_regular_page() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!("http://{addr}/docs/concepts/agents"))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs-0-23/src/pages/concepts/agents.rs")
    );
}

#[tokio::test]
async fn edit_link_stable_section_index() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!("http://{addr}/docs/getting-started"))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs-0-23/src/pages/getting_started/introduction.rs")
    );
}

#[tokio::test]
async fn edit_link_next_version() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!("http://{addr}/docs/next/concepts/agents"))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs/src/pages/concepts/agents.rs")
    );
}

#[tokio::test]
async fn edit_link_old_version() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!("http://{addr}/docs/0.22/concepts/agents"))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs-0-22/src/pages/concepts/agents.rs")
    );
}

#[tokio::test]
async fn edit_link_locale_with_version() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!("http://{addr}/ja/docs/0.22/concepts/agents"))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs-ja-0-22/src/pages/concepts/agents.rs")
    );
}

#[tokio::test]
async fn edit_link_migration_guide() {
    let addr = start_file_server(&build_dir()).await;
    let client = make_client().await;

    client
        .goto(&format!(
            "http://{addr}/docs/migration-guides/yew/from-0-22-0-to-0-23-0"
        ))
        .await
        .unwrap();
    wait_for_page(&client).await;
    let href = get_edit_link_href(&client).await;
    client.close().await.unwrap();

    assert_eq!(
        href,
        format!("{GITHUB_BASE}/docs/src/pages/migration_guides/yew/from_0_22_0_to_0_23_0.rs")
    );
}
