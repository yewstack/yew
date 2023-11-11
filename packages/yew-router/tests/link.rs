#![cfg(not(target_os = "wasi"))]

use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

use gloo::utils::window;
use js_sys::{JsString, Object, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::functional::function_component;
use yew::platform::time::sleep;
use yew::prelude::*;
use yew_router::prelude::*;

mod utils;
use utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct PageParam {
    page: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct SearchParams {
    q: String,
    lang: Option<String>,
}

impl SearchParams {
    fn new(q: &str) -> Self {
        Self {
            q: q.to_string(),
            lang: None,
        }
    }

    fn new_with_lang(q: &str, lang: &str) -> Self {
        Self {
            q: q.to_string(),
            lang: Some(lang.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/posts")]
    Posts,
    #[at("/search")]
    Search,
}

#[derive(PartialEq, Properties)]
struct NavigationMenuProps {
    #[prop_or(None)]
    assertion: Option<fn(&Navigator, &Location)>,
}

#[function_component(NavigationMenu)]
fn navigation_menu(props: &NavigationMenuProps) -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    if let Some(assertion) = props.assertion {
        assertion(&navigator, &location);
    }

    html! {
        <ul>
            <li class="posts">
                <Link<Routes> to={Routes::Posts}>
                    { "Posts without parameters" }
                </Link<Routes>>
            </li>
            <li class="posts-page-2">
                <Link<Routes, PageParam> to={Routes::Posts} query={Some(PageParam { page: 2 })}>
                    { "Posts of 2nd page" }
                </Link<Routes, PageParam>>
            </li>
            <li class="search">
                <Link<Routes> to={Routes::Search}>
                    { "Search withfout parameters" }
                </Link<Routes>>
            </li>
            <li class="search-q">
                <Link<Routes, SearchParams> to={Routes::Search} query={Some(SearchParams::new("Rust"))}>
                    { "Search with keyword parameter" }
                </Link<Routes, SearchParams>>
            </li>
            <li class="search-q-lang">
                <Link<Routes, SearchParams> to={Routes::Search} query={Some(SearchParams::new_with_lang("Rust", "en_US"))}>
                    { "Search with keyword and language parameters" }
                </Link<Routes, SearchParams>>
            </li>
        </ul>
    }
}

#[function_component(RootForBrowserRouter)]
fn root_for_browser_router() -> Html {
    html! {
        <BrowserRouter>
            <NavigationMenu />
        </BrowserRouter>
    }
}

async fn link_in_browser_router() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "browser-router");
    let _ = gloo::utils::body().append_child(&div);
    let handle = yew::Renderer::<RootForBrowserRouter>::with_root(div).render();

    sleep(Duration::ZERO).await;

    assert_eq!("/posts", link_href("#browser-router ul > li.posts > a"));
    assert_eq!(
        "/posts?page=2",
        link_href("#browser-router ul > li.posts-page-2 > a")
    );

    assert_eq!("/search", link_href("#browser-router ul > li.search > a"));
    assert_eq!(
        "/search?q=Rust",
        link_href("#browser-router ul > li.search-q > a")
    );
    assert_eq!(
        "/search?q=Rust&lang=en_US",
        link_href("#browser-router ul > li.search-q-lang > a")
    );

    handle.destroy();
}

#[derive(PartialEq, Properties)]
struct BasenameProps {
    basename: Option<String>,
    assertion: fn(&Navigator, &Location),
}

#[function_component(RootForBasename)]
fn root_for_basename(props: &BasenameProps) -> Html {
    html! {
        <BrowserRouter basename={props.basename.clone()}>
            <NavigationMenu assertion={props.assertion}/>
        </BrowserRouter>
    }
}

async fn link_with_basename(correct_initial_path: bool) {
    if correct_initial_path {
        let cookie = Object::new();
        Reflect::set(&cookie, &JsString::from("foo"), &JsString::from("bar")).unwrap();
        window()
            .history()
            .unwrap()
            .replace_state_with_url(&cookie, "", Some("/base/"))
            .unwrap();
    }

    static RENDERS: AtomicU8 = AtomicU8::new(0);
    RENDERS.store(0, Ordering::Relaxed);

    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "with-basename");
    let _ = gloo::utils::body().append_child(&div);

    let mut handle = yew::Renderer::<RootForBasename>::with_root_and_props(
        div,
        BasenameProps {
            basename: Some("/base/".to_owned()),
            assertion: |navigator, location| {
                RENDERS.fetch_add(1, Ordering::Relaxed);
                assert_eq!(navigator.basename(), Some("/base"));
                assert_eq!(location.path(), "/base/");
            },
        },
    )
    .render();

    sleep(Duration::ZERO).await;

    if correct_initial_path {
        // If the initial path was correct, the router shouldn't have mutated the history.
        assert_eq!(
            Reflect::get(
                &window().history().unwrap().state().unwrap(),
                &JsString::from("foo")
            )
            .unwrap()
            .as_string()
            .as_deref(),
            Some("bar")
        );
    }

    assert_eq!(
        "/base/",
        gloo::utils::window().location().pathname().unwrap()
    );

    assert_eq!("/base/posts", link_href("#with-basename ul > li.posts > a"));
    assert_eq!(
        "/base/posts?page=2",
        link_href("#with-basename ul > li.posts-page-2 > a")
    );

    assert_eq!(
        "/base/search",
        link_href("#with-basename ul > li.search > a")
    );
    assert_eq!(
        "/base/search?q=Rust",
        link_href("#with-basename ul > li.search-q > a")
    );
    assert_eq!(
        "/base/search?q=Rust&lang=en_US",
        link_href("#with-basename ul > li.search-q-lang > a")
    );

    // Some(a) -> Some(b)
    handle.update(BasenameProps {
        basename: Some("/bayes/".to_owned()),
        assertion: |navigator, location| {
            RENDERS.fetch_add(1, Ordering::Relaxed);
            assert_eq!(navigator.basename(), Some("/bayes"));
            assert_eq!(location.path(), "/bayes/");
        },
    });

    sleep(Duration::ZERO).await;

    assert_eq!(
        "/bayes/",
        gloo::utils::window().location().pathname().unwrap()
    );

    assert_eq!(
        "/bayes/posts",
        link_href("#with-basename ul > li.posts > a")
    );

    // Some -> None
    handle.update(BasenameProps {
        basename: None,
        assertion: |navigator, location| {
            RENDERS.fetch_add(1, Ordering::Relaxed);
            assert_eq!(navigator.basename(), None);
            assert_eq!(location.path(), "/");
        },
    });

    sleep(Duration::ZERO).await;

    assert_eq!("/", gloo::utils::window().location().pathname().unwrap());

    assert_eq!("/posts", link_href("#with-basename ul > li.posts > a"));

    // None -> Some
    handle.update(BasenameProps {
        basename: Some("/bass/".to_string()),
        assertion: |navigator, location| {
            RENDERS.fetch_add(1, Ordering::Relaxed);
            assert_eq!(navigator.basename(), Some("/bass"));
            assert_eq!(location.path(), "/bass/");
        },
    });

    sleep(Duration::ZERO).await;

    assert_eq!(
        "/bass/",
        gloo::utils::window().location().pathname().unwrap()
    );

    assert_eq!("/bass/posts", link_href("#with-basename ul > li.posts > a"));

    handle.destroy();

    // 1 initial, 1 rerender after initial, 3 props changes
    assert_eq!(RENDERS.load(Ordering::Relaxed), 5);
}

#[function_component(RootForHashRouter)]
fn root_for_hash_router() -> Html {
    html! {
        <HashRouter>
            <NavigationMenu />
        </HashRouter>
    }
}

async fn link_in_hash_router() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "hash-router");
    let _ = gloo::utils::body().append_child(&div);
    let handle = yew::Renderer::<RootForHashRouter>::with_root(div).render();

    sleep(Duration::ZERO).await;

    assert_eq!("#/posts", link_href("#hash-router ul > li.posts > a"));
    assert_eq!(
        "#/posts?page=2",
        link_href("#hash-router ul > li.posts-page-2 > a")
    );

    assert_eq!("#/search", link_href("#hash-router ul > li.search > a"));
    assert_eq!(
        "#/search?q=Rust",
        link_href("#hash-router ul > li.search-q > a")
    );
    assert_eq!(
        "#/search?q=Rust&lang=en_US",
        link_href("#hash-router ul > li.search-q-lang > a")
    );

    handle.destroy();
}

// These cannot be run in concurrently because they all read/write the URL.
#[test]
async fn sequential_tests() {
    link_in_hash_router().await;
    link_in_browser_router().await;
    link_with_basename(false).await;
    link_with_basename(true).await;
}
