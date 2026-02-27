use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::{Query, Request, State};
use axum::handler::HandlerWithoutStateExt;
use axum::http::Uri;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use function_router::{route_meta, Route, ServerApp, ServerAppProps};
use futures::stream::{self, StreamExt};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use hyper_util::server;
use tokio::net::TcpListener;
use tower::Service;
use tower_http::services::ServeDir;
use yew::platform::Runtime;
use yew_router::prelude::Routable;

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

fn head_tags_for(path: &str) -> String {
    let route = Route::recognize(path).unwrap_or(Route::NotFound);
    let (title, description) = route_meta(&route);
    format!(
        "<title>{title} | Yew SSR Router</title><meta name=\"description\" \
         content=\"{description}\" />"
    )
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    State((index_html_before, index_html_after)): State<(String, String)>,
) -> impl IntoResponse {
    let path = url.path().to_owned();

    // Inject route-specific <head> tags before </head>, outside of Yew rendering.
    let before = index_html_before.replace("</head>", &format!("{}</head>", head_tags_for(&path)));

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: path.into(),
        queries,
    });

    Body::from_stream(
        stream::once(async move { before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

// An executor to process requests on the Yew runtime.
//
// By spawning requests on the Yew runtime,
// it processes request on the same thread as the rendering task.
//
// This increases performance in some environments (e.g.: in VM).
#[derive(Clone, Default)]
struct Executor {
    inner: Runtime,
}

impl<F> hyper::rt::Executor<F> for Executor
where
    F: Future + Send + 'static,
{
    fn execute(&self, fut: F) {
        self.inner.spawn_pinned(move || async move {
            fut.await;
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let exec = Executor::default();

    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");

    let index_html_after = index_html_after.to_owned();

    let app = Router::new().fallback_service(
        ServeDir::new(opts.dir)
            .append_index_html_on_directories(false)
            .fallback(
                get(render)
                    .with_state((index_html_before.clone(), index_html_after.clone()))
                    .into_service(),
            ),
    );

    let addr: SocketAddr = ([0, 0, 0, 0], 8080).into();

    println!("You can view the website at: http://localhost:8080/");

    let listener = TcpListener::bind(addr).await?;

    // Continuously accept new connections.
    loop {
        // In this example we discard the remote address. See `fn serve_with_connect_info` for how
        // to expose that.
        let (socket, _remote_addr) = listener.accept().await.unwrap();

        // We don't need to call `poll_ready` because `Router` is always ready.
        let tower_service = app.clone();

        let exec = exec.clone();
        // Spawn a task to handle the connection. That way we can handle multiple connections
        // concurrently.
        tokio::spawn(async move {
            // Hyper has its own `AsyncRead` and `AsyncWrite` traits and doesn't use tokio.
            // `TokioIo` converts between them.
            let socket = TokioIo::new(socket);

            // Hyper also has its own `Service` trait and doesn't use tower. We can use
            // `hyper::service::service_fn` to create a hyper `Service` that calls our app through
            // `tower::Service::call`.
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                // We have to clone `tower_service` because hyper's `Service` uses `&self` whereas
                // tower's `Service` requires `&mut self`.
                //
                // We don't need to call `poll_ready` since `Router` is always ready.
                tower_service.clone().call(request)
            });

            // `server::conn::auto::Builder` supports both http1 and http2.
            //
            // `TokioExecutor` tells hyper to use `tokio::spawn` to spawn tasks.
            if let Err(err) = server::conn::auto::Builder::new(exec)
                // `serve_connection_with_upgrades` is required for websockets. If you don't need
                // that you can use `serve_connection` instead.
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }
}
