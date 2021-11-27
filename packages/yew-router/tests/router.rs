use serde::{Deserialize, Serialize};
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::functional::function_component;
use yew::prelude::*;
use yew_router::prelude::*;

mod utils;
use utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Serialize, Deserialize)]
struct Query {
    foo: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/no/:id")]
    No { id: u32 },
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq, Clone)]
struct NoProps {
    id: u32,
}

#[function_component(No)]
fn no(props: &NoProps) -> Html {
    let route = props.id.to_string();

    let location = use_location().unwrap();

    html! {
        <>
            <div id="result-params">{ route }</div>
            <div id="result-query">{ location.query::<Query>().unwrap().foo }</div>
        </>
    }
}

#[function_component(Comp)]
fn component() -> Html {
    let history = use_history().unwrap();

    let switch = Switch::render(move |routes| {
        let history_clone = history.clone();
        let replace_route = Callback::from(move |_| {
            history_clone
                .replace_with_query(
                    Routes::No { id: 2 },
                    Query {
                        foo: "bar".to_string(),
                    },
                )
                .unwrap();
        });

        let history_clone = history.clone();
        let push_route = Callback::from(move |_| {
            history_clone
                .push_with_query(
                    Routes::No { id: 3 },
                    Query {
                        foo: "baz".to_string(),
                    },
                )
                .unwrap();
        });

        match routes {
            Routes::Home => html! {
                <>
                    <div id="result">{"Home"}</div>
                    <button onclick={replace_route}>{"replace a route"}</button>
                </>
            },
            Routes::No { id } => html! {
                <>
                    <No id={*id} />
                    <button onclick={push_route}>{"push a route"}</button>
                </>
            },
            Routes::NotFound => html! { <div id="result">{"404"}</div> },
        }
    });

    html! {
        <Switch<Routes> render={switch} />
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
    yew::start_app_in_element::<Root>(gloo_utils::document().get_element_by_id("output").unwrap());

    assert_eq!("Home", obtain_result_by_id("result"));

    let initial_length = history_length();

    click("button"); // replacing the current route
    assert_eq!("2", obtain_result_by_id("result-params"));
    assert_eq!("bar", obtain_result_by_id("result-query"));
    assert_eq!(initial_length, history_length());

    click("button"); // pushing a new route
    assert_eq!("3", obtain_result_by_id("result-params"));
    assert_eq!("baz", obtain_result_by_id("result-query"));
    assert_eq!(initial_length + 1, history_length());
}
