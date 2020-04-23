use std::path::PathBuf;

#[rustfmt::skip]
use warp::{
    filters::BoxedFilter,
    fs::File,
    path::Peek,
    path,
    Filter, Reply,
};

fn main() {
    let localhost = [0, 0, 0, 0];
    let port = 8000;
    let addr = (localhost, port);

    // You will need to change this if you use this as a template for your application.

    const ASSETS_DIR: &str = "../../../target/deploy";
    let assets_dir: PathBuf = PathBuf::from(ASSETS_DIR);

    let routes = api().or(static_files_handler(assets_dir));

    warp::serve(routes).run(addr);
}

const API_STRING: &str = "api";

pub fn api() -> BoxedFilter<(impl Reply,)> {
    warp::path(API_STRING)
        .and(path!(String))
        .and(warp::get2())
        .map(std::convert::identity) // Echos the string back in the response body
        .boxed()
}

/// Expose filters that work with static files
pub fn static_files_handler(assets_dir: PathBuf) -> BoxedFilter<(impl Reply,)> {
    const INDEX_HTML: &str = "index.html";

    let files =
        assets(assets_dir.clone()).or(index_static_file_redirect(assets_dir.join(INDEX_HTML)));

    warp::any().and(files).boxed()
}

/// If the path does not start with /api, return the index.html, so the app will bootstrap itself
/// regardless of whatever the frontend-specific path is.
fn index_static_file_redirect(index_file_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::path::peek())
        .and(warp::fs::file(index_file_path))
        .and_then(|segments: Peek, file: File| {
            // Reject the request if the path starts with /api/
            if let Some(first_segment) = segments.segments().next() {
                if first_segment == API_STRING {
                    return Err(warp::reject::not_found());
                }
            }
            Ok(file)
        })
        .boxed()
}

/// Gets the file within the specified dir.
fn assets(dir_path: PathBuf) -> BoxedFilter<(impl Reply,)> {
    warp::get2()
        .and(warp::fs::dir(dir_path))
        .and(warp::path::end())
        .boxed()
}
