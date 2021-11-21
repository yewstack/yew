use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew_router::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn router_always_404() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum AppRoute {
        #[at("/")]
        Home,
        #[at("/:id")]
        Article { id: u64 },
        #[at("/404")]
        #[not_found]
        NotFound,
    }

    assert_eq!(
        Some(AppRoute::NotFound),
        AppRoute::recognize("/not/matched/route")
    );
    assert_eq!(
        Some(AppRoute::NotFound),
        AppRoute::recognize("/not-matched-route")
    );
}

#[test]
fn router_trailing_slash() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum AppRoute {
        #[at("/")]
        Home,
        #[at("/category/:name/")]
        Category { name: String },
        #[at("/:id")]
        Article { id: u64 },
        #[at("/404")]
        #[not_found]
        NotFound,
    }

    assert_eq!(
        Some(AppRoute::Category {
            name: "cooking-recipes".to_string()
        }),
        AppRoute::recognize("/category/cooking-recipes/")
    );
}
