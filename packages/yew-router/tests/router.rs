use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;
use yew_functional::function_component;
use yew_router::prelude::*;

mod utils;
use std::collections::HashMap;
use utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/no/:id")]
    No { id: u32 },
    #[at("/404")]
    NotFound,
}

#[function_component(No)]
fn no() -> Html {
    let route = match RouterService::current_route().route::<Routes>() {
        Routes::No { id } => format!("{}", id),
        _ => String::new(),
    };

    html! {
        <>
            <div id="result-params">{route}</div>
            <div id="result-query">{RouterService::current_route().query().get("foo").unwrap()}</div>
        </>
    }
}

#[function_component(Comp)]
fn component() -> Html {
    let onclick = Callback::from(|_| {
        RouterService::push(
            Routes::No { id: 2 },
            Some({
                let mut map = HashMap::new();
                map.insert("foo", "bar".to_string());
                map
            }),
        )
    });
    html! {
        <Router<Routes> not_found_route="/404">
            <Route to=Routes::HOME>
                <div id="result">{"Home"}</div>
                <a onclick=onclick>{"click me"}</a>
            </Route>
            <Route to=Routes::NO>
                <No />
            </Route>
            <Route to=Routes::NOT_FOUND>
                <div id="result">{"404"}</div>
            </Route>
        </Router<Routes>>
    }
}

// all the tests are in place because document state isn't being reset between tests
// different routes at the time of execution are set and it causes weird behavior (tests
// failing randomly)
// this test tests
// - routing
// - parameters in the path
// - query parameters
// - 404 redirects
#[test]
fn router_works() {
    let app = App::<Comp>::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());

    assert_eq!("Home", obtain_result_by_id("result"));

    click("a");
    assert_eq!("2", obtain_result_by_id("result-params"));
    assert_eq!("bar", obtain_result_by_id("result-query"));
}
