use std::collections::HashSet;
use std::rc::Rc;

use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
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
fn use_reducer_works() {
    struct UseReducerFunction {}
    impl FunctionProvider for UseReducerFunction {
        type TProps = ();
        fn run(_: &Self::TProps) -> Html {
            let counter = use_reducer(|| CounterState { counter: 10 });

            let counter_clone = counter.clone();
            use_effect_with_deps(
                move |_| {
                    counter_clone.dispatch(1);
                    || {}
                },
                (),
            );
            return html! {
                <div>
                    {"The test result is"}
                    <div id="result">{counter.counter}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseReducerComponent = FunctionComponent<UseReducerFunction>;
    yew::start_app_in_element::<UseReducerComponent>(
        gloo_utils::document().get_element_by_id("output").unwrap(),
    );
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
fn use_reducer_eq_works() {
    struct UseReducerFunction {}
    impl FunctionProvider for UseReducerFunction {
        type TProps = ();
        fn run(_: &Self::TProps) -> Html {
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

            return html! {
                <>
                    <div>
                        {"This component has been rendered: "}<span id="result">{render_count}</span>{" Time(s)."}
                    </div>
                    <button onclick={add_content_a} id="add-a">{"Add A to Content"}</button>
                    <button onclick={add_content_b} id="add-b">{"Add B to Content"}</button>
                </>
            };
        }
    }
    type UseReducerComponent = FunctionComponent<UseReducerFunction>;
    yew::start_app_in_element::<UseReducerComponent>(
        document().get_element_by_id("output").unwrap(),
    );

    let result = obtain_result();
    assert_eq!(result.as_str(), "1");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-a")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    let result = obtain_result();
    assert_eq!(result.as_str(), "2");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");

    document()
        .get_element_by_id("add-b")
        .unwrap()
        .unchecked_into::<HtmlElement>()
        .click();

    let result = obtain_result();
    assert_eq!(result.as_str(), "3");
}
