use std::convert::Infallible;
use std::path::PathBuf;

use clap::Parser;
use futures::stream::{self, Stream, StreamExt};
use simple_ssr::App;
use warp::Filter;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long)]
    dir: PathBuf,
}

async fn render(
    index_html_before: String,
    index_html_after: String,
) -> impl Stream<Item = Result<String, Infallible>> {
    let renderer = yew::ServerRenderer::<App>::new();

    stream::once(async move { Ok(index_html_before) })
        .chain(renderer.render_stream().map(Ok))
        .chain(stream::once(async move { Ok(index_html_after) }))
}

#[tokio::main]
async fn main() {
    let opts = Opt::parse();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let (index_html_before, index_html_after) = index_html_s.split_once("<body>").unwrap();
    let mut index_html_before = index_html_before.to_owned();
    index_html_before.push_str("<body>");
    let index_html_after = index_html_after.to_owned();

    let html = warp::path::end().then(move || {
        let index_html_before = index_html_before.clone();
        let index_html_after = index_html_after.clone();

        async move {
            let body = render(index_html_before, index_html_after).await;
            warp::reply::with_header(
                warp::reply::stream(body),
                "content-type",
                "text/html; charset=utf-8",
            )
        }
    });

    let routes = html.or(warp::fs::dir(opts.dir));

    println!("You can view the website at: http://localhost:8080/");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
