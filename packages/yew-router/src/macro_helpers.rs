use crate::utils::{base_url, build_path_with_base, strip_slash_suffix};
use crate::Routable;

// re-export Router because the macro needs to access it
pub type Router = route_recognizer::Router<String>;

/// Build a `route_recognizer::Router` from a `Routable` type.
pub fn build_router<R: Routable>() -> Router {
    let base = base_url();
    let mut router = Router::new();
    R::routes().iter().for_each(|path| {
        match &base {
            Some(base) if base != "/" => {
                let route = build_path_with_base(path);
                let mut dest = route.replace(base, "");
                if dest.is_empty() {
                    dest = "/".to_string();
                }
                router.add(&route, dest);
            }
            _ => {
                router.add(&path, path.to_string());
            }
        };
    });

    router
}

/// Use a `route_recognizer::Router` to build the route of a `Routable`
pub fn recognize_with_router<R: Routable>(router: &Router, pathname: &str) -> Option<R> {
    let pathname = strip_slash_suffix(pathname);
    let matched = router.recognize(pathname);

    match matched {
        Ok(matched) => R::from_path(matched.handler(), &matched.params().into_iter().collect()),
        Err(_) => R::not_found_route(),
    }
}
