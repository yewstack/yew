// TODO: remove the cfg after wasm-bindgen-test stops emitting the function unconditionally
#![cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew_router::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

#[test]
fn router_always_404() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum AppRoute {
        #[at("/")]
        Home,
        #[at("/{id}")]
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
        #[at("/category/{name}/")]
        Category { name: String },
        #[at("/{id}")]
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

#[test]
fn router_url_encoding() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum AppRoute {
        #[at("/")]
        Root,
        #[at("/search/{query}")]
        Search { query: String },
    }

    assert_eq!(
        yew_router::__macro::decode_for_url("/search/a%2Fb/").unwrap(),
        "/search/a/b/"
    );

    assert_eq!(
        Some(AppRoute::Search {
            query: "a/b".to_string()
        }),
        AppRoute::recognize("/search/a%2Fb/")
    );
}

#[test]
fn router_wildcard_encoding() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum AppRoute {
        #[at("/")]
        Root,
        #[at("/file/{*path}")]
        File { path: String },
        #[at("/user/{id}")]
        User { id: String },
    }

    let route = AppRoute::File {
        path: "docs/guides/getting-started.md".to_string(),
    };
    assert_eq!(route.to_path(), "/file/docs/guides/getting-started.md");

    assert_eq!(
        Some(AppRoute::File {
            path: "docs/guides/getting-started.md".to_string()
        }),
        AppRoute::recognize("/file/docs/guides/getting-started.md")
    );

    let route_special = AppRoute::File {
        path: "docs/my file (1)/notes.txt".to_string(),
    };
    assert_eq!(
        route_special.to_path(),
        "/file/docs/my%20file%20%281%29/notes.txt"
    );

    assert_eq!(
        Some(AppRoute::File {
            path: "docs/my file (1)/notes.txt".to_string()
        }),
        AppRoute::recognize("/file/docs/my%20file%20%281%29/notes.txt")
    );

    let user_route = AppRoute::User {
        id: "a/b".to_string(),
    };
    assert_eq!(user_route.to_path(), "/user/a%2Fb");
}

#[test]
fn router_nested() {
    #[derive(Routable, Debug, Clone, PartialEq)]
    enum MainRoute {
        #[at("/")]
        Home,
        #[at("/settings")]
        SettingsRoot,
        #[at("/settings/{*_rest}")]
        Settings { _rest: String },
        #[at("/404")]
        #[not_found]
        NotFound,
    }

    #[derive(Routable, Debug, Clone, PartialEq)]
    enum SettingsRoute {
        #[at("/settings")]
        Profile,
        #[at("/settings/friends")]
        Friends,
        #[at("/settings/theme")]
        Theme,
        #[at("/settings/404")]
        #[not_found]
        NotFound,
    }

    // Static /settings matches the root variant
    assert_eq!(
        Some(MainRoute::SettingsRoot),
        MainRoute::recognize("/settings")
    );

    // Trailing slash also matches root via strip_slash_suffix
    assert_eq!(
        Some(MainRoute::SettingsRoot),
        MainRoute::recognize("/settings/")
    );

    // Subpaths match the wildcard variant
    assert_eq!(
        Some(MainRoute::Settings {
            _rest: "friends".to_string()
        }),
        MainRoute::recognize("/settings/friends")
    );
    assert_eq!(
        Some(MainRoute::Settings {
            _rest: "theme".to_string()
        }),
        MainRoute::recognize("/settings/theme")
    );

    // Unknown subpath still matches wildcard on the outer router
    assert_eq!(
        Some(MainRoute::Settings {
            _rest: "unknown".to_string()
        }),
        MainRoute::recognize("/settings/unknown")
    );

    // Inner router resolves its own routes
    assert_eq!(
        Some(SettingsRoute::Profile),
        SettingsRoute::recognize("/settings")
    );
    assert_eq!(
        Some(SettingsRoute::Friends),
        SettingsRoute::recognize("/settings/friends")
    );
    assert_eq!(
        Some(SettingsRoute::Theme),
        SettingsRoute::recognize("/settings/theme")
    );

    // Inner router falls back to not_found for unknown subpaths
    assert_eq!(
        Some(SettingsRoute::NotFound),
        SettingsRoute::recognize("/settings/unknown")
    );

    // Unrelated paths hit the outer not_found
    assert_eq!(
        Some(MainRoute::NotFound),
        MainRoute::recognize("/other/path")
    );
}
