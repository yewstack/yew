use website_e2e::{build_dir, start_file_server};

async fn get_description(base: &str, path: &str) -> Option<String> {
    let url = format!("{base}{path}");
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let prefix = r#"<meta name="description" content=""#;
    let start = body.find(prefix)? + prefix.len();
    let end = start + body[start..].find('"')?;
    Some(body[start..end].to_string())
}

async fn assert_description(base: &str, path: &str, expected: &str) {
    let desc = get_description(base, path)
        .await
        .unwrap_or_else(|| panic!("no description meta tag found at {path}"));
    assert_eq!(desc, expected, "description mismatch at {path}");
}

async fn assert_description_contains(base: &str, path: &str, substring: &str) {
    let desc = get_description(base, path)
        .await
        .unwrap_or_else(|| panic!("no description meta tag found at {path}"));
    assert!(
        desc.contains(substring),
        "description at {path} should contain '{substring}', got '{desc}'"
    );
}

#[tokio::test]
async fn custom_description() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    assert_description(&base, "/docs/more/deployment", "Deploying Yew applications").await;
}

#[tokio::test]
async fn auto_extracted_first_paragraph() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    assert_description_contains(&base, "/docs/more/css", "Yew does not").await;
}

#[tokio::test]
async fn home_page_description() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    assert_description(
        &base,
        "/",
        "A framework for creating reliable and efficient web applications.",
    )
    .await;
}

#[tokio::test]
async fn localized_description() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    assert_description_contains(&base, "/ja/", "信頼性が高く効率的な").await;
    assert_description_contains(
        &base,
        "/ja/docs/more/deployment",
        "Yew アプリケーションのデプロイ",
    )
    .await;
}

#[tokio::test]
async fn versioned_description() {
    let addr = start_file_server(&build_dir()).await;
    let base = format!("http://{addr}");
    assert_description(
        &base,
        "/docs/0.21/more/deployment",
        "Deploying Yew applications",
    )
    .await;
    assert_description(
        &base,
        "/docs/next/more/deployment",
        "Deploying Yew applications",
    )
    .await;
}
