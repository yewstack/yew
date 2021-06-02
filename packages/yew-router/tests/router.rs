use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;
use yew_functional::function_component;
use yew_router::prelude::*;

mod utils;
use utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Clone, PartialEq, Routable)]
enum Routes {
    #[at("/")]
    Home,
    #[at("/no/:id")]
    No {
        id: u32,
        #[bind(query)]
        query_param: String,
    },
    #[at("/404")]
    NotFound,
}

#[derive(Properties, PartialEq, Clone)]
struct NoProps {
    id: u32,
    query_param: String,
}

#[function_component(No)]
fn no(props: &NoProps) -> Html {
    let route = props.id.to_string();

    html! {
        <>
            <div id="result-params">{ route }</div>
            <div id="result-query">{ props.query_param.clone() }</div>
        </>
    }
}

#[function_component(Comp)]
fn component() -> Html {
    let switch = Router::render(|routes| {
        let onclick = Callback::from(|_| {
            yew_router::push_route(Routes::No {
                id: 2,
                query_param: "bar".to_string(),
            })
        });

        match routes {
            Routes::Home => html! {
                <>
                    <div id="result">{"Home"}</div>
                    <a onclick=onclick>{"click me"}</a>
                </>
            },
            Routes::No { id, query_param } => html! { <No id=id query_param=query_param /> },
            Routes::NotFound => html! { <div id="result">{"404"}</div> },
        }
    });

    html! {
        <Router<Routes> render=switch>
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
    yew::start_app_in_element::<Comp>(yew::utils::document().get_element_by_id("output").unwrap());

    assert_eq!("Home", obtain_result_by_id("result"));

    click("a");
    assert_eq!("2", obtain_result_by_id("result-params"));
    assert_eq!("bar", obtain_result_by_id("result-query"));
}
