use std::time::Duration;

use yew::platform::time::sleep;
use yew::prelude::*;
use yew_router::prelude::*;

mod utils;
use utils::*;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
wasm_bindgen_test_configure!(run_in_browser);

#[derive(Routable, Debug, Clone, PartialEq)]
enum AppRoute {
    #[at("/")]
    Root,
    #[at("/search/:query")]
    Search { query: String },
}

#[function_component]
fn Comp() -> Html {
    let switch = move |routes: AppRoute| match routes {
        AppRoute::Root => html! {
            <>
                <h1>{ "Root" }</h1>
                <Link<AppRoute> to={AppRoute::Search { query: "a/b".to_string() }}>
                    {"Click me"}
                </Link<AppRoute>>
            </>
        },
        AppRoute::Search { query } => html! {
            <p id="q">{ query }</p>
        },
    };
    html! {
        <Switch<AppRoute> render={switch} />
    }
}

#[function_component(Root)]
fn root() -> Html {
    html! {
        <BrowserRouter>
            <Comp />
        </BrowserRouter>
    }
}

#[test]
async fn url_encoded_roundtrip() {
    yew::Renderer::<Root>::with_root(gloo::utils::document().get_element_by_id("output").unwrap())
        .render();
    sleep(Duration::ZERO).await;
    click("a");
    sleep(Duration::ZERO).await;
    let res = obtain_result_by_id("q");
    assert_eq!(res, "a/b");

    assert_eq!(
        gloo::utils::window().location().pathname().unwrap(),
        "/search/a%2Fb"
    )
}
