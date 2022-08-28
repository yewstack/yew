use std::time::Duration;

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

#[function_component(NavigationMenu)]
fn navigation_menu() -> Html {
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

#[test]
async fn link_in_browser_router() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "browser-router");
    let _ = gloo::utils::body().append_child(&div);
    yew::Renderer::<RootForBrowserRouter>::with_root(div).render();

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
}

#[function_component(RootForBasename)]
fn root_for_basename() -> Html {
    html! {
        <BrowserRouter basename="/base/">
            <NavigationMenu />
        </BrowserRouter>
    }
}

#[test]
async fn link_with_basename() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "with-basename");
    let _ = gloo::utils::body().append_child(&div);
    yew::Renderer::<RootForBasename>::with_root(div).render();

    sleep(Duration::ZERO).await;

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
}

#[function_component(RootForHashRouter)]
fn root_for_hash_router() -> Html {
    html! {
        <HashRouter>
            <NavigationMenu />
        </HashRouter>
    }
}

#[test]
async fn link_in_hash_router() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = div.set_attribute("id", "hash-router");
    let _ = gloo::utils::body().append_child(&div);
    yew::Renderer::<RootForHashRouter>::with_root(div).render();

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
}
