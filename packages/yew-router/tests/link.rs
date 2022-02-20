use gloo::timers::future::sleep;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::functional::function_component;
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
        Self { q: q.to_string(), lang: None  }
    }

    fn new_with_lang(q: &str, lang: &str) -> Self {
        Self { q: q.to_string(), lang: Some(lang.to_string()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/posts")]
    Posts,
    #[at("/search")]
    Search,
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <BrowserRouter>
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
        </BrowserRouter>
    }
}

#[test]
async fn link_works() {
    let div = gloo::utils::document().create_element("div").unwrap();
    let _ = gloo::utils::body().append_child(&div);
    yew::start_app_in_element::<Root>(div);

    sleep(Duration::ZERO).await;

    assert_eq!("/posts", link_href("ul > li.posts > a"));
    assert_eq!("/posts?page=2", link_href("ul > li.posts-page-2 > a"));

    assert_eq!("/search", link_href("ul > li.search > a"));
    assert_eq!("/search?q=Rust", link_href("ul > li.search-q > a"));
    assert_eq!("/search?q=Rust&lang=en_US", link_href("ul > li.search-q-lang > a"));
}
