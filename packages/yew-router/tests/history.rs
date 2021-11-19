use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew_router::prelude::*;
use yew_router::AnyRoute;

use serde::{Deserialize, Serialize};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Query {
    a: String,
    b: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct State {
    i: String,
    ii: u64,
}

#[test]
async fn history_works() {
    let history = BrowserHistory::new();
    assert_eq!(history.location().pathname(), "/");

    history.push(AnyRoute::new("/path-a"));
    assert_eq!(history.location().pathname(), "/path-a");

    history.replace(AnyRoute::new("/path-b"));
    assert_eq!(history.location().pathname(), "/path-b");

    history.back();
    TimeoutFuture::new(100).await;
    assert_eq!(history.location().pathname(), "/");

    history.forward();
    TimeoutFuture::new(100).await;
    assert_eq!(history.location().pathname(), "/path-b");

    history
        .push_with_query(
            AnyRoute::new("/path"),
            Query {
                a: "something".to_string(),
                b: 123,
            },
        )
        .unwrap();

    assert_eq!(history.location().pathname(), "/path");
    assert_eq!(history.location().search(), "?a=something&b=123");
    assert_eq!(
        history.location().query::<Query>().unwrap(),
        Query {
            a: "something".to_string(),
            b: 123,
        }
    );

    history
        .push_with_state(
            AnyRoute::new("/path-c"),
            State {
                i: "something".to_string(),
                ii: 123,
            },
        )
        .unwrap();

    assert_eq!(history.location().pathname(), "/path-c");
    assert_eq!(
        history.state::<State>().unwrap(),
        State {
            i: "something".to_string(),
            ii: 123,
        }
    );
}
