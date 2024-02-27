use std::collections::HashMap;
use std::convert::Infallible;
use std::path::PathBuf;

use axum::body::Body;
use axum::error_handling::HandleError;
use axum::extract::{Query, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::{StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use function_router::{ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use tower_http::services::ServeDir;
use yew::platform::Runtime;

// We use jemalloc as it produces better performance.
#[cfg(unix)]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long)]
    dir: PathBuf,
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let url = url.to_string();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: url.into(),
        queries,
    });

    Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
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
            format!("error occurred: {e}"),
        )
    };

    let app = Router::new().fallback_service(HandleError::new(
        ServeDir::new(opts.dir)
            .append_index_html_on_directories(false)
            .fallback(
                get(render)
                    .with_state((index_html_before.clone(), index_html_after.clone()))
                    .into_service(),
            ),
        handle_error,
    ));

    println!("You can view the website at: http://localhost:8080/");

    // Process requests on the Yew runtime.
    //
    // By spawning requests on the Yew runtime, it processes request on the same thread as the
    // rendering task.
    //
    // This increases performance in some environments (e.g.: in VM).

    let rt = Runtime::default();
    rt.spawn_pinned(move || async move {
        let tcp = tokio::net::TcpListener::bind("127.0.0.1:8080")
            .await
            .unwrap();
        axum::serve::serve(tcp, app).await.unwrap();
    })
}
