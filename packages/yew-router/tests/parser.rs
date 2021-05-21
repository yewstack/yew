use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::utils::*;
use yew_router::parse_query;
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
    document().head().unwrap().set_inner_html(r#""#);

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

#[derive(Serialize, Clone)]
struct QueryParams {
    foo: String,
    bar: u32,
}

#[test]
fn test_get_query_params() {
    assert_eq!(
        parse_query::<HashMap<String, String>>().unwrap(),
        HashMap::new()
    );

    let query = QueryParams {
        foo: "test".to_string(),
        bar: 69,
    };

    yew_router::push_route_with_query(Routes::Home, query).unwrap();

    let params: HashMap<String, String> = parse_query().unwrap();

    assert_eq!(params, {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "test".to_string());
        map.insert("bar".to_string(), "69".to_string());
        map
    });
}

#[test]
fn test_build_base_url() {
    document().head().unwrap().set_inner_html(r#""#);

    assert_eq!(build_path_with_base("/posts"), "/posts");

    document()
        .head()
        .unwrap()
        .set_inner_html(r#"<base href="/router">"#);

    assert_eq!(build_path_with_base("/posts/"), "/router/posts");

    document()
        .head()
        .unwrap()
        .set_inner_html(r#"<base href="/">"#);

    assert_eq!(build_path_with_base("/posts"), "/posts");
}
