use crate::utils::{base_url, strip_slash_suffix};
use crate::Routable;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
pub use web_sys::Location;

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

/// Use a `route_recognizer::Router` to build the route of a `Routable`
pub fn recognize_with_router<R: Routable>(
    router: &Router,
    location: web_sys::Location,
    bindings: HashMap<&str, HashMap<&str, Binding>>,
) -> Option<R> {
    let pathname = location.pathname().unwrap();
    let pathname = strip_slash_suffix(&pathname);
    let matched = router.recognize(pathname);

    match matched {
        Ok(matched) => {
            let path = matched.handler();
            let bindings = &bindings[path.as_str()];
            let search = location.search();
            let query = search
                .as_deref()
                .map(|query| query.strip_prefix("?").unwrap_or(""));
            let mut queries = HashMap::new();
            let mut query_collections: HashMap<String, Vec<String>> = HashMap::new();
            if let Ok(_query) = query {
                let query_iter = parse_query();
                for (k, v) in query_iter {
                    let binding = bindings[k.as_str()];
                    match binding {
                        Binding::Query => {
                            queries.insert(k, v);
                        }
                        Binding::QueryCollection => match query_collections.get_mut(&k) {
                            Some(map) => {
                                map.push(v);
                            }
                            None => {
                                query_collections.insert(k, vec![v]);
                            }
                        },
                    };
                }
            }
            R::from_path(path, &matched.params().into_iter().collect(), queries)
        }
        Err(_) => R::not_found_route(),
    }
}

fn parse_query() -> impl Iterator<Item = (String, String)> {
    let url = web_sys::Url::new(&yew::utils::document().url().unwrap()).unwrap();

    js_sys::try_iter(&JsValue::from(&url.search_params()))
        .expect("try_iter failed")
        .expect("try_iter failed")
        .into_iter()
        .map(|it| it.unwrap().unchecked_into::<js_sys::Array>().to_vec())
        .map(|it| {
            let mut iter = it.into_iter();
            // unwraps are unreachable
            // there will be at least 2 values here
            // both of them will be strings
            (
                iter.next().unwrap().as_string().unwrap(),
                iter.next().unwrap().as_string().unwrap(),
            )
        })
}
