use std::net::SocketAddr;
use std::path::Path;

use axum::Router;
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
