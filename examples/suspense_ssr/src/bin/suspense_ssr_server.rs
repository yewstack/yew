use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::Request;
use axum::handler::HandlerWithoutStateExt;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use futures::stream::{self, StreamExt};
use hyper::body::Incoming;
use hyper_util::rt::TokioIo;
use hyper_util::server;
use suspense_ssr::App;
use tokio::net::TcpListener;
use tower::Service;
use tower_http::services::ServeDir;
use yew::platform::Runtime;

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

#[derive(Parser, Debug)]
struct Opt {
    #[clap(short, long)]
    dir: PathBuf,
}

async fn render(index_html_before: String, index_html_after: String) -> impl IntoResponse {
    let renderer = yew::ServerRenderer::<App>::new();

    Body::from_stream(
        stream::once(async move { index_html_before })
            .chain(renderer.render_stream())
            .chain(stream::once(async move { index_html_after }))
            .map(Result::<_, Infallible>::Ok),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let exec = Executor::default();
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
                get(move || render(index_html_before.clone(), index_html_after.clone()))
                    .into_service(),
            ),
    );

    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();
    println!("You can view the website at: http://localhost:8080/");

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _remote_addr) = listener.accept().await.unwrap();
        let tower_service = app.clone();
        let exec = exec.clone();

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);
            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().call(request)
            });

            let _result = server::conn::auto::Builder::new(exec)
                .serve_connection_with_upgrades(socket, hyper_service)
                .await;
        });
    }
}
