use std::collections::HashMap;
use std::io::Result as IoResult;
use std::path::PathBuf;

use actix_ssr_router::{
    LinkedAuthor, LinkedPost, LinkedPostMeta, ServerApp, ServerAppProps, LINK_ENDPOINT,
};
use actix_web::http::Uri;
use actix_web::web::{get, post, Data, Query};
use actix_web::{App, Error, HttpResponse, HttpServer};
use bytes::Bytes;
use clap::Parser;
use function_router::{route_meta, Route};
use futures::stream::{self, StreamExt};
use yew_link::actix_web::linked_state_handler;
use yew_link::{Resolver, ResolverProp};
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

#[derive(Clone)]
struct AppState {
    index_html_before: String,
    index_html_after: String,
    resolver: ResolverProp,
}

async fn render(
    url: Uri,
    Query(queries): Query<HashMap<String, String>>,
    state: Data<AppState>,
) -> HttpResponse {
    let state = state.into_inner();

    let path = url.path().to_owned();

    // Inject route-specific <head> tags before </head>, outside of Yew rendering.
    let before = state
        .index_html_before
        .replace("</head>", &format!("{}</head>", head_tags_for(&path)));
    let resolver = state.resolver.clone();

    let renderer = yew::ServerRenderer::<ServerApp>::with_props(move || ServerAppProps {
        url: path.into(),
        queries,
        resolver,
    });

    HttpResponse::Ok().streaming(
        stream::once(async move { Bytes::from(before) })
            .chain(renderer.render_stream().map(Bytes::from))
            .chain(stream::once(async move {
                Bytes::from(state.index_html_after.clone())
            }))
            .map(Ok::<Bytes, Error>),
    )
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    env_logger::init();
    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let app_state = Data::new(AppState {
        index_html_before,
        index_html_after,
        resolver: Resolver::new()
            .register_linked::<LinkedPost>(())
            .register_linked::<LinkedAuthor>(())
            .register_linked::<LinkedPostMeta>(())
            .into(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route(LINK_ENDPOINT, post().to(linked_state_handler))
            .default_service(get().to(render))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
