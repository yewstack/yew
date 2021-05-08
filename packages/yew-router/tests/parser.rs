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

impl Default for Routes {
    fn default() -> Self {
        Self::NotFound
    }
}

#[test]
fn test_find_base_url() {
    document().head().unwrap().set_inner_html(r#""#);

    assert_eq!(find_base_url(), None);

    document()
        .head()
        .unwrap()
        .set_inner_html(r#"<base href="/base/">"#);
    assert_eq!(find_base_url(), Some("/base".to_string()));

    document()
        .head()
        .unwrap()
        .set_inner_html(r#"<base href="/base">"#);
    assert_eq!(find_base_url(), Some("/base".to_string()));
}
