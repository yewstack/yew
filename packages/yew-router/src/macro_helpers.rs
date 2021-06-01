use crate::utils::{base_url, strip_slash_suffix};
use crate::Routable;
pub use web_sys::Location;
use std::collections::HashMap;

// re-export Router because the macro needs to access it
pub type Router = route_recognizer::Router<String>;

/// Build a `route_recognizer::Router` from a `Routable` type.
pub fn build_router<R: Routable>() -> Router {
    let base = base_url();
    let mut router = Router::new();
    R::routes().iter().for_each(|path| {
        match &base {
            Some(base) => {
                let route = format!("{}{}", base, path);
                let route = strip_slash_suffix(&route);
                router.add(route, path.to_string());
            }
            _ => {
                router.add(&path, path.to_string());
            }
        };
    });

    router
}

#[derive(Debug, Copy, Clone)]
pub enum Binding {
    Query,
    QueryCollection,
}

#[derive(Debug, Clone)]
pub enum Query {
    Single(HashMap<String, String>),
    Collection(HashMap<String, Vec<String>>),
}

/// Use a `route_recognizer::Router` to build the route of a `Routable`
pub fn recognize_with_router<R: Routable>(router: &Router, location: web_sys::Location) -> Option<R> {
    let bindings = {
        let mut map = HashMap::with_capacity(1);
        map.insert("collection_key", Binding::QueryCollection);
        map.insert("key", Binding::Query);
        map
    };

    let pathname = location.pathname().unwrap();
    let pathname = strip_slash_suffix(&pathname);

    let search = location.search();
    let query = search.as_deref().map(|query| query.strip_prefix("?").unwrap_or(""));
    let mut queries = HashMap::new();
    let mut query_collections = HashMap::new();
    if let Ok(_query) = query {
        let query_iter = fuck(); // TODO parse query ...
        for (k, v) in query_iter {
            let binding = bindings[k.as_str()];
            match binding {
                Binding::Query => {
                    queries.insert(k, v);
                }
                Binding::QueryCollection => {
                    match query_collections.get_mut(&k) {
                        Some(map) => {
                            map.push(v);
                        },
                        None => {
                            query_collections.insert(k, vec![v]);
                        }
                    }
                }
            };
        }
    }


    let matched = router.recognize(pathname);

    match matched {
        Ok(matched) => R::from_path(
            matched.handler(),
            &matched.params().into_iter().collect(),
            queries,
        ),
        Err(_) => R::not_found_route(),
    }
}

fn fuck() -> impl Iterator<Item = (String, String)> {
    std::iter::once((String::new(), String::new()))
}
