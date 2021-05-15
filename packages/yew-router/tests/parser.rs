use std::collections::HashMap;
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::utils::*;
use yew_router::prelude::*;
use yew_router::utils::*;
use serde::Serialize;

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

#[derive(Serialize, Clone)]
struct QueryParams {
    foo: String,
    bar: u32
}

#[test]
fn test_get_query_params() {
    assert_eq!(get_query_params(), HashMap::new());

    let query = QueryParams {
        foo: "test".to_string(),
        bar: 69,
    };

    service::push_with_query(
        Routes::Home,
        query.clone(),
    ).unwrap();

    let params = get_query_params();

    assert_eq!(params, {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), "test".to_string());
        map.insert("bar".to_string(), "69".to_string());
        map
    });
}
