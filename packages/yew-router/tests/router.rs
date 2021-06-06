#![allow(clippy::blacklisted_name)]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;
use yew_functional::function_component;
use yew_router::{prelude::*, push_route};

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
        #[bind(query_arg)]
        foo: String,
    },
    #[at("/complex/:a/:b/:c")]
    Complex {
        b: u32,
        #[bind(query_arg)]
        q1: u32,
        #[bind(hash_arg)]
        h1: u32,
        a: u32,
        c: u32,
        #[bind(query_arg)]
        q2: u32,
        #[bind(hash_arg)]
        h2: u32,
    },
    #[at("/404")]
    NotFound,
}

impl Default for Routes {
    fn default() -> Self {
        Self::NotFound
    }
}

#[derive(Properties, PartialEq, Clone)]
struct NoProps {
    id: u32,
    foo: String,
}

#[function_component(No)]
fn no(props: &NoProps) -> Html {
    let route = props.id.to_string();

    html! {
        <>
            <div id="result-params">{ route }</div>
            <div id="result-query">{ &props.foo }</div>
        </>
    }
}

#[function_component(Comp)]
fn component() -> Html {
    let switch = Router::render(|routes| {
        let onclick = Callback::from(|_| {
            push_route(Routes::No {
                id: 2,
                foo: "bar".into(),
            })
        });
        match routes {
            Routes::Home => html! {
                <>
                    <div id="result">{"Home"}</div>
                    <a onclick=onclick>{"click me"}</a>
                </>
            },
            Routes::No { id, foo } => html! { <No id=*id foo=foo.clone() /> },
            Routes::Complex {
                a,
                b,
                c,
                q1,
                q2,
                h1,
                h2,
            } => html! { <div id="result">{a} {b} {c} {q1} {q2} {h1} {h2}</div> },
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

    push_route(Routes::Home);
}

#[test]
fn complex_args() {
    yew::start_app_in_element::<Comp>(yew::utils::document().get_element_by_id("output").unwrap());

    assert_eq!("Home", obtain_result_by_id("result"));

    push_route(Routes::Complex {
        a: 1,
        b: 2,
        c: 3,
        q1: 4,
        q2: 5,
        h1: 6,
        h2: 7,
    });
    assert_eq!("1234567", obtain_result_by_id("result"));

    push_route(Routes::Home);
}
