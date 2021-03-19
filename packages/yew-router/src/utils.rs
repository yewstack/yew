use crate::{components::route::Route, CurrentRoute};
use route_recognizer::Params;
use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use yew::{Children, ChildrenWithProps};

pub fn base_url() -> Option<String> {
    match yew::utils::document().query_selector("base") {
        Ok(Some(base)) => {
            let base = base
                .unchecked_into::<web_sys::Element>()
                .attributes()
                .get_named_item("href")
                .expect("base without href")
                .value();
            if base == "/" {
                None
            } else {
                let base = base.strip_suffix("/").unwrap_or(&base);
                Some(base.to_string())
            }
        }
        _ => None,
    }
}

pub fn build_path_with_base(to: &str) -> String {
    let to = format!("{}{}", base_url().as_deref().unwrap_or(""), to);

    let path = if to == "/" {
        to
    } else {
        to.strip_suffix("/").map(|it| it.to_string()).unwrap_or(to)
    };

    path
}

pub fn get_query_params() -> HashMap<String, String> {
    let url = web_sys::Url::new(&yew::utils::document().url().unwrap()).unwrap();

    let iter = js_sys::try_iter(&JsValue::from(&url.search_params()))
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
        });

    let mut map = HashMap::new();

    for (k, v) in iter {
        map.insert(k, v);
    }

    map
}

pub fn from_route(
    pathname: &str,
    routes: &ChildrenWithProps<Route>,
    not_found_route: Option<&str>,
    router: &route_recognizer::Router<String>,
) -> Option<(Children, CurrentRoute)> {
    let mut selected = None;
    if let Ok(path) = router.recognize(pathname.strip_suffix("/").unwrap_or(pathname)) {
        let children = routes
            .iter()
            .find(|it| build_path_with_base(&it.props.to) == **path.handler())
            .unwrap()
            .props
            .children;
        selected = Some((
            children,
            CurrentRoute::new(path.handler().to_string(), path.params().clone()),
        ));
    }

    match selected {
        Some(selected) => Some(selected),
        None => {
            let not_found_route = not_found_route?;

            let route = routes.iter().find(|it| it.props.to == not_found_route)?;
            Some((
                route.props.children,
                CurrentRoute::new(not_found_route.to_string(), Params::default()),
            ))
        }
    }
}
