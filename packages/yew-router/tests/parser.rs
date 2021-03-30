use std::collections::HashMap;
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::prelude::*;
use yew::utils::*;
use yew::virtual_dom::VChild;
use yew_router::components::route::RouteProps;
use yew_router::prelude::*;
use yew_router::utils::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/no")]
    No,
    #[at("/404")]
    NotFound,
}

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

    RouterService::push(
        Routes::Home,
        Some({
            let mut map = HashMap::new();
            map.insert("foo", "bar".to_string());
            map.insert("value", "test".to_string());
            map
        }),
    );

    assert_eq!(get_query_params(), {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "bar".to_string());
        map.insert("value".to_string(), "test".to_string());
        map
    });
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

    let route = from_route::<Routes>("/", &routes, None, &router).expect("no route matched");
    assert_eq!(*route.1.route::<Routes>(), Routes::Home);

    let route = from_route::<Routes>("/no", &routes, None, &router).expect("no route matched");
    assert_eq!(*route.1.route::<Routes>(), Routes::No);

    let route = from_route::<Routes>("/no/", &routes, None, &router).expect("no route matched");
    assert_eq!(*route.1.route::<Routes>(), Routes::No);
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

    let route =
        from_route::<Routes>("/no", &routes, Some("/404"), &router).expect("no route matched");
    assert_eq!(*route.1.route::<Routes>(), Routes::NotFound);
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

    let route = from_route::<Routes>("/no", &routes, None, &router);
    assert!(route.is_none());
}
