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

#[cfg(test)]
#[cfg(feature = "wasm_test")]
mod tests {
    use super::*;
    use crate::components::route::RouteProps;
    use crate::RouterService;
    use wasm_bindgen_test::wasm_bindgen_test as test;
    use yew::utils::*;
    use yew::virtual_dom::VChild;
    use yew::{html, NodeRef};

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_base_url() {
        assert_eq!(base_url(), None);

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base/">"#);
        assert_eq!(base_url(), Some("/base".to_string()));

        document()
            .head()
            .unwrap()
            .set_inner_html(r#"<base href="/base">"#);
        assert_eq!(base_url(), Some("/base".to_string()));
    }

    #[test]
    fn test_get_query_params() {
        assert_eq!(get_query_params(), HashMap::new());

        let map = {
            let mut map = HashMap::new();
            map.insert("foo".to_string(), "bar".to_string());
            map.insert("value".to_string(), "test".to_string());
            map
        };

        RouterService::push("/?foo=bar&value=test");
        assert_eq!(get_query_params(), map);

        RouterService::push("/path/?foo=bar&value=test");
        assert_eq!(get_query_params(), map);

        RouterService::push("/path?foo=bar&value=test");
        assert_eq!(get_query_params(), map);
    }

    #[test]
    fn from_route_works() {
        let routes = ChildrenWithProps::new(vec![
            VChild::<Route>::new(
                RouteProps {
                    to: "/".to_string(),
                    children: Children::new(vec![html! {<div>{"Hello world"}</div>}]),
                },
                NodeRef::default(),
                None,
            ),
            VChild::<Route>::new(
                RouteProps {
                    to: "/no".to_string(),
                    children: Children::new(vec![html! {<div>{"No"}</div>}]),
                },
                NodeRef::default(),
                None,
            ),
        ]);
        let mut router = route_recognizer::Router::new();
        router.add("/", "/".to_string());
        router.add("/no", "/no".to_string());

        let route = from_route("/", &routes, None, &router).expect("no route matched");
        assert_eq!(route.1.path(), "/");

        let route = from_route("/no", &routes, None, &router).expect("no route matched");
        assert_eq!(route.1.path(), "/no");

        let route = from_route("/no/", &routes, None, &router).expect("no route matched");
        assert_eq!(route.1.path(), "/no");
    }

    #[test]
    fn from_route_404_works() {
        let routes = ChildrenWithProps::new(vec![VChild::<Route>::new(
            RouteProps {
                to: "/404".to_string(),
                children: Children::new(vec![html! {<div>{"404"}</div>}]),
            },
            NodeRef::default(),
            None,
        )]);
        let mut router = route_recognizer::Router::new();
        router.add("/404", "/404".to_string());

        let route = from_route("/no", &routes, Some("/404"), &router).expect("no route matched");
        assert_eq!(route.1.path(), "/404");
    }

    #[test]
    fn from_route_returns_none_on_no_match_without_not_found_specifed() {
        let routes = ChildrenWithProps::new(vec![VChild::<Route>::new(
            RouteProps {
                to: "/404".to_string(),
                children: Children::new(vec![html! {<div>{"404"}</div>}]),
            },
            NodeRef::default(),
            None,
        )]);
        let mut router = route_recognizer::Router::new();
        router.add("/404", "/404".to_string());

        let route = from_route("/no", &routes, None, &router);
        assert!(route.is_none());
    }
}
