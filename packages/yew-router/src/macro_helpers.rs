pub use urlencoding::{decode as decode_for_url, encode as encode_for_url};

pub fn encode_path_for_url(path: &str) -> String {
    path.split('/')
        .map(encode_for_url)
        .collect::<Vec<_>>()
        .join("/")
}

use std::collections::HashMap;

use crate::utils::strip_slash_suffix;
use crate::Routable;

// re-export Router because the macro needs to access it
pub type Router = matchit::Router<String>;

/// Build a `matchit::Router` from a `Routable` type.
pub fn build_router<R: Routable>() -> Router {
    let mut router = Router::new();
    R::routes().iter().for_each(|path| {
        let stripped_route = strip_slash_suffix(path);
        router
            .insert(stripped_route, path.to_string())
            .unwrap_or_else(|e| panic!("failed to insert route {stripped_route:?}: {e}"));
    });

    router
}

/// Use a `matchit::Router` to match the route of a `Routable`
pub fn recognize_with_router<R: Routable>(router: &Router, pathname: &str) -> Option<R> {
    let pathname = strip_slash_suffix(pathname);
    let matched = router.at(pathname);

    match matched {
        Ok(matched) => {
            let params: HashMap<&str, &str> = matched.params.iter().collect();
            R::from_path(matched.value, &params).or_else(R::not_found_route)
        }
        Err(_) => R::not_found_route(),
    }
}
