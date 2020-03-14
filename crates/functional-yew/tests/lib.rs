#[cfg(test)]
extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;
use std::ops::DerefMut;
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);


extern crate yew;

use yew::{Html, html, App, Properties};
use functional_yew::{FunctionProvider, use_state, FunctionComponent, use_ref, use_effect, use_effect1, use_reducer2};

#[wasm_bindgen_test]
fn use_state_works() {
    struct UseStateFunction {}
    impl FunctionProvider for UseStateFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let (counter, set_counter) = use_state(|| 0);
            if *counter < 5 {
                set_counter(*counter + 1)
            }
            return html! {
                <div>
                    {"Test Output: "}
                    <div id="result">{*counter}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseComponent = FunctionComponent<UseStateFunction>;
    // yew::initialize();
    let app: App<UseComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();
    assert_eq!(result.as_str(), "5");
}

#[wasm_bindgen_test]
fn props_are_passed() {
    struct PropsPassedFunction {}
    #[derive(Properties, Clone, PartialEq)]
    struct PropsPassedFunctionProps {
        value: String
    }
    impl FunctionProvider for PropsPassedFunction {
        type TProps = PropsPassedFunctionProps;

        fn run(props: &Self::TProps) -> Html {
            assert_eq!(&props.value, "props");
            return html! {
                <div id="result">
                    {"done"}
                </div>
            };
        }
    }
    type PropsComponent = FunctionComponent<PropsPassedFunction>;
    let app: App<PropsComponent> = yew::App::new();
    app.mount_with_props(yew::utils::document().get_element_by_id("output").unwrap(),
                         PropsPassedFunctionProps { value: "props".to_string() });
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();
    assert_eq!(result.as_str(), "done");
}

#[wasm_bindgen_test]
fn use_ref_works() {
    struct UseRefFunction {}
    impl FunctionProvider for UseRefFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let ref_example = use_ref(|| 0);
            *ref_example.borrow_mut().deref_mut() += 1;
            let (counter, set_counter) = use_state(|| 0);
            if *counter < 5 {
                set_counter(*counter + 1)
            }
            return html! {
                <div>
                    {"The test output is: "}
                    <div id="result">{*ref_example.borrow_mut().deref_mut() > 4}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseRefComponent = FunctionComponent<UseRefFunction>;
    let app: App<UseRefComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();
    assert_eq!(result.as_str(), "true");
}

#[wasm_bindgen_test]
fn use_effect_works() {
    struct UseEffectFunction {}
    impl FunctionProvider for UseEffectFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let number_ref = use_ref(|| 0);
            let number_ref_c = number_ref.clone();
            use_effect(|| {
                if *number_ref_c.borrow_mut().deref_mut() == 1 {
                    panic!("This effect should have been called once only")
                }
                *number_ref_c.borrow_mut().deref_mut() += 1;
                || {
                    panic!("Destructor should not have been called")
                }
            });
            return html! {
                <div>
                    {"The test result is"}
                    <div id="result">{*number_ref.borrow_mut().deref_mut()}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseEffectComponent = FunctionComponent<UseEffectFunction>;
    let app: App<UseEffectComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();
    assert_eq!(result.as_str(), "1");
}

#[wasm_bindgen_test]
fn use_effect_refires_on_dependency_change() {
    struct UseEffectFunction {}
    impl FunctionProvider for UseEffectFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let number_ref = use_ref(|| 0);
            let number_ref_c = number_ref.clone();
            let number_ref2 = use_ref(|| 0);
            let number_ref2_c = number_ref2.clone();
            let arg = *number_ref.borrow_mut().deref_mut();
            use_effect1(move |dep| {
                let mut ref_mut = number_ref_c.borrow_mut();
                let inner_ref_mut = ref_mut.deref_mut();
                if *inner_ref_mut < 1 {
                    *inner_ref_mut += 1;
                    assert_eq!(dep, &0);
                } else {
                    assert_eq!(dep, &1);
                }
                move || {
                    *number_ref2_c.borrow_mut().deref_mut() += 1;
                }
            }, arg);
            return html! {
                <div>
                    {"The test result is"}
                    <div id="result">{*number_ref.borrow_mut().deref_mut()}{*number_ref2.borrow_mut().deref_mut()}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseEffectComponent = FunctionComponent<UseEffectFunction>;
    let app: App<UseEffectComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();

    assert_eq!(result.as_str(), "11");
}

#[wasm_bindgen_test]
fn use_reducer_works() {
    struct UseReducerFunction {}
    impl FunctionProvider for UseReducerFunction {
        type TProps = ();

        fn run(props: &Self::TProps) -> Html {
            struct CounterState {
                counter: i32,
            }
            let (counter, dispatch) = use_reducer2(
                |prev: std::rc::Rc<CounterState>, action: i32| CounterState {
                    counter: prev.counter + action,
                },
                0,
                |initial: i32| CounterState {
                    counter: initial + 10,
                },
            );
            use_effect1(|_| {
                dispatch(1);
                || {}
            }, 0);
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
    let app: App<UseReducerComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result: String = yew::utils::document().get_element_by_id("result").unwrap().inner_html();

    assert_eq!(result.as_str(), "11");
}