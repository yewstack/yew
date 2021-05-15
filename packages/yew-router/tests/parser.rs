use std::collections::HashMap;
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::utils::*;
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

    service::push(
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
