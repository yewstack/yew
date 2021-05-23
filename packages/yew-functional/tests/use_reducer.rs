mod common;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::{html, Html};
use yew_functional::{
    use_effect_with_deps, use_reducer_with_init, FunctionComponent, FunctionProvider,
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn use_reducer_works() {
    struct UseReducerFunction {}
    impl FunctionProvider for UseReducerFunction {
        type TProps = ();
        fn run(_: &Self::TProps) -> Html {
            struct CounterState {
                counter: i32,
            }
            let counter = use_reducer_with_init(
                |prev: std::rc::Rc<CounterState>, action: i32| CounterState {
                    counter: prev.counter + action,
                },
                0,
                |initial: i32| CounterState {
                    counter: initial + 10,
                },
            );

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
        yew::utils::document().get_element_by_id("output").unwrap(),
    );
    let result = obtain_result();

    assert_eq!(result.as_str(), "11");
}
