use std::collections::HashMap;
use std::convert::Infallible;
use std::path::PathBuf;

use axum::body::{Body, StreamBody};
use axum::error_handling::HandleError;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use clap::Parser;
use function_router::{ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use tower::ServiceExt;
use tower_http::services::ServeDir;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(
    Extension((index_html_before, index_html_after)): Extension<(String, String)>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let url = url.uri().to_string();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    StreamBody::new(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream().await)
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {}", e),
        )
    };

    let app = Router::new()
        .route("/api/test", get(|| async move { "Hello World" }))
        .fallback(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    render
                        .layer(Extension((
                            index_html_before.clone(),
                            index_html_after.clone(),
                        )))
                        .into_service()
                        .map_err(|err| -> std::io::Error { match err {} }),
                ),
            handle_error,
        ));

    println!("You can view the website at: http://localhost:8080/");

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
