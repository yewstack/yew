use clap::Parser;
use function_router::{ServerApp, ServerAppProps};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio_util::task::LocalPoolHandle;
use warp::Filter;

// We spawn a local pool that is as big as the number of cpu threads.
static LOCAL_POOL: Lazy<LocalPoolHandle> = Lazy::new(|| LocalPoolHandle::new(num_cpus::get()));

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(index_html_s: &str, url: &str, queries: HashMap<String, String>) -> String {
    let url = url.to_string();

    let content = LOCAL_POOL
        .spawn_pinned(move || async move {
            let server_app_props = ServerAppProps {
                url: url.into(),
                queries,
            };

            let renderer = yew::ServerRenderer::<ServerApp>::with_props(server_app_props);

            renderer.render().await
        })
        .await
        .expect("the task has failed.");

    // Good enough for an example, but developers should avoid the replace and extra allocation
    // here in an actual app.
    index_html_s.replace("<body>", &format!("<body>{}", content))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let render = move |s: warp::filters::path::FullPath, queries: HashMap<String, String>| {
        let index_html_s = index_html_s.clone();

        async move { warp::reply::html(render(&index_html_s, s.as_str(), queries).await) }
    };

    let html = warp::path::end().and(
        warp::path::full()
            .and(warp::filters::query::query())
            .then(render.clone()),
    );

    let routes = html.or(warp::fs::dir(opts.dir)).or(warp::path::full()
        .and(warp::filters::query::query())
        .then(render));

    println!("You can view the website at: http://localhost:8080/");

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
