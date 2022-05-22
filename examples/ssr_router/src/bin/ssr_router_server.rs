use std::collections::HashMap;
use std::path::PathBuf;

use axum::body::Body;
use axum::error_handling::HandleError;
use axum::extract::Query;
use axum::handler::Handler;
use axum::http::{Request, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Extension, Router};
use clap::Parser;
use function_router::{ServerApp, ServerAppProps};
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
    Extension(index_html_s): Extension<String>,
    url: Request<Body>,
    Query(queries): Query<HashMap<String, String>>,
) -> Html<String> {
    let url = url.uri().to_string();

    let server_app_props = ServerAppProps { url, queries };

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(server_app_props);

    let content = renderer.render().await;

    // Good enough for an example, but developers should avoid the replace and extra allocation
    // here in an actual app.
    Html(index_html_s.replace("<body>", &format!("<body>{}", content)))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let handle_error = |e| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("error occurred: {}", e),
        )
    };

    let app = Router::new()
        .route("/api/test", get(|| async move { "Hello World" }))
        // needed because https://github.com/tower-rs/tower-http/issues/262
        .route("/", get(render))
        .fallback(HandleError::new(
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(false)
                .fallback(
                    render
                        .layer(Extension(index_html_s))
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
