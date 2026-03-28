use fantoccini::Locator;
use website_e2e::{build_dir, make_client, start_file_server, wait_for_page};

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
