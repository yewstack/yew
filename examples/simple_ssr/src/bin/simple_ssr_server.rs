use simple_ssr::App;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;
use warp::Filter;

/// A basic example
#[derive(StructOpt, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[structopt(short, long, parse(from_os_str))]
    dir: PathBuf,
}

async fn render(index_html_s: &str) -> String {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
    })
    .await
    .expect("the thread has failed.");

    // Good enough for an example, but developers should print their html properly in actual
    // application.
    index_html_s.replace("<body>", &format!("<body>{}", content))
}

#[tokio::main]
async fn main() {
    let opts = Opt::from_args();

    let index_html_s = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html");

    let html = warp::path::end().then(move || {
        let index_html_s = index_html_s.clone();

        async move { warp::reply::html(render(&index_html_s).await) }
    });

    let routes = html.or(warp::fs::dir(opts.dir));

    println!("You can view the website at: http://localhost:8080/");

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
