use crate::utils::{base_url, build_path_with_base};
use crate::Routable;

// re-export Router because the macro needs to access it
pub type Router = route_recognizer::Router<String>;

/// Build a `route_recognizer::Router` from a `Routable` type.
pub fn build_router<R: Routable>() -> Router {
    let base = base_url();
    let mut router = Router::new();
    R::routes().iter().for_each(|path| {
        let path = match &base {
            Some(base) if base != "/" => build_path_with_base(path),
            _ => path.to_string(),
        };
        router.add(&path, path.clone());
    });

    router
}

/// Use a `route_recognizer::Router` to build the route of a `Routable`
pub fn recognize_with_router<R: Routable>(router: &Router, pathname: &str) -> Option<R> {
    let matched = router.recognize(&pathname.strip_suffix("/").unwrap_or(&pathname));

    match matched {
        Ok(matched) => R::from_path(matched.handler(), &matched.params().into_iter().collect()),
        Err(_) => R::not_found_route(),
    }
}
