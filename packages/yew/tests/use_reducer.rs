#![cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]

use std::collections::HashSet;
use std::rc::Rc;
use std::time::Duration;

use gloo::utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::platform::time::sleep;
use yew::prelude::*;

mod common;

use common::obtain_result;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug)]
struct CounterState {
    counter: i32,
}

impl Reducible for CounterState {
    type Action = i32;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {
            counter: self.counter + action,
        }
        .into()
    }
}

#[wasm_bindgen_test]
async fn use_reducer_works() {
    #[component(UseReducerComponent)]
    fn use_reducer_comp() -> Html {
        let counter = use_reducer(|| CounterState { counter: 10 });

        let counter_clone = counter.clone();
        use_effect_with((), move |_| {
            counter_clone.dispatch(1);
            || {}
        });
        html! {
            <div>
                {"The test result is"}
                <div id="result">{counter.counter}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseReducerComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;
    let result = obtain_result();

    assert_eq!(result.as_str(), "11");
}

#[derive(Debug, Clone, PartialEq)]
struct ContentState {
    content: HashSet<String>,
}

impl Reducible for ContentState {
    type Action = String;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut self_: Self = (*self).clone();
        self_.content.insert(action);
        self_.into()
    }
}

#[wasm_bindgen_test]
async fn use_reducer_eq_works() {
    #[component(UseReducerComponent)]
    fn use_reducer_comp() -> Html {
        let content = use_reducer_eq(|| ContentState {
            content: HashSet::default(),
        });

        let render_count = use_mut_ref(|| 0);

        let render_count = {
            let mut render_count = render_count.borrow_mut();
            *render_count += 1;

            *render_count
        };

        let add_content_a = {
            let content = content.clone();
            Callback::from(move |_| content.dispatch("A".to_string()))
        };

        let add_content_b = Callback::from(move |_| content.dispatch("B".to_string()));

        html! {
            <>
                <div>
                    {"This component has been rendered: "}<span id="result">{render_count}</span>{" Time(s)."}
                </div>
                <button onclick={add_content_a} id="add-a">{"Add A to Content"}</button>
                <button onclick={add_content_b} id="add-b">{"Add B to Content"}</button>
            </>
        }
    }

    yew::Renderer::<UseReducerComponent>::with_root(
        document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "1");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");
}

enum SometimesChangeAction {
    /// If this action is sent, the state will remain the same
    Keep,
    /// If this action is sent, the state will change
    Change,
}

/// A state that does not implement PartialEq
#[derive(Clone)]
struct SometimesChangingState {
    value: i32,
}

impl Reducible for SometimesChangingState {
    type Action = SometimesChangeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        use SometimesChangeAction::*;
        match action {
            Keep => self,
            Change => {
                let mut self_: Self = (*self).clone();
                self_.value += 1;
                self_.into()
            }
        }
    }
}

#[wasm_bindgen_test]
async fn use_reducer_does_not_rerender_when_rc_is_reused() {
    #[component(UseReducerComponent)]
    fn use_reducer_comp() -> Html {
        let state = use_reducer(|| SometimesChangingState { value: 0 });
        let render_count = use_mut_ref(|| 0);

        let render_count = {
            let mut render_count = render_count.borrow_mut();
            *render_count += 1;

            *render_count
        };

        let keep_state = {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(SometimesChangeAction::Keep))
        };

        let change_state = Callback::from(move |_| state.dispatch(SometimesChangeAction::Change));

        html! {
            <>
                <div>
                    {"This component has been rendered: "}<span id="result">{render_count}</span>{" Time(s)."}
                </div>
                <button onclick={keep_state} id="keep-state">{"Keep State"}</button>
                <button onclick={change_state} id="change-state">{"Change State"}</button>
            </>
        }
    }

    yew::Renderer::<UseReducerComponent>::with_root(
        document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "1");

    document()
        .get_element_by_id("change-state")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("keep-state")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();
    sleep(Duration::ZERO).await;

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");
}
