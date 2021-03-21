use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;
use yew_functional::function_component;
use yew_router::prelude::*;

mod utils;
use utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(No)]
fn no() -> Html {
    html! {
        <>
            <div id="result-params">{RouterService::current_route().parmas().get("id").unwrap()}</div>
            <div id="result-query">{RouterService::current_route().query().get("foo").unwrap()}</div>
            <Link route="/yes">{"click me"}</Link>
        </>
    }
}

#[function_component(Comp)]
fn component() -> Html {
    html! {
        <Router not_found_route="/404">
            <Route to="/">
                <div id="result">{"Home"}</div>
                <Link route="/no/2/?foo=bar">{"click me"}</Link>
            </Route>
            <Route to="/no/:id">
                <No />
            </Route>
            <Route to="/404">
                <div id="result">{"404"}</div>
            </Route>
        </Router>
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
    click("a");
    assert_eq!("404", obtain_result_by_id("result"));
}
