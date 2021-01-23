mod common;

use common::obtain_result;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use wasm_bindgen_test::*;
use yew::{html, App, Html, Properties};
use yew_functional::{
    use_effect_with_deps, use_ref, use_state, FunctionComponent, FunctionProvider,
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn use_effect_destroys_on_component_drop() {
    struct UseEffectFunction {}
    struct UseEffectWrapper {}
    #[derive(Properties, Clone)]
    struct WrapperProps {
        destroy_called: Rc<dyn Fn()>,
    }
    impl PartialEq for WrapperProps {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }
    #[derive(Properties, Clone)]
    struct FunctionProps {
        effect_called: Rc<dyn Fn()>,
        destroy_called: Rc<dyn Fn()>,
    }
    impl PartialEq for FunctionProps {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }
    type UseEffectComponent = FunctionComponent<UseEffectFunction>;
    type UseEffectWrapperComponent = FunctionComponent<UseEffectWrapper>;
    impl FunctionProvider for UseEffectFunction {
        type TProps = FunctionProps;

        fn run(props: &Self::TProps) -> Html {
            let effect_called = props.effect_called.clone();
            let destroy_called = props.destroy_called.clone();
            use_effect_with_deps(
                move |_| {
                    effect_called();
                    move || destroy_called()
                },
                (),
            );
            return html! {};
        }
    }
    impl FunctionProvider for UseEffectWrapper {
        type TProps = WrapperProps;

        fn run(props: &Self::TProps) -> Html {
            let (show, set_show) = use_state(|| true);
            if *show {
                let effect_called: Rc<dyn Fn()> = Rc::new(move || set_show(false));
                return html! {
                    <UseEffectComponent destroy_called=props.destroy_called.clone() effect_called=effect_called />
                };
            } else {
                return html! {
                    <div>{"EMPTY"}</div>
                };
            }
        }
    }
    let app: App<UseEffectWrapperComponent> = yew::App::new();
    let destroy_counter = Rc::new(std::cell::RefCell::new(0));
    let destroy_counter_c = destroy_counter.clone();
    app.mount_with_props(
        yew::utils::document().get_element_by_id("output").unwrap(),
        WrapperProps {
            destroy_called: Rc::new(move || *destroy_counter_c.borrow_mut().deref_mut() += 1),
        },
    );
    assert_eq!(1, *destroy_counter.borrow().deref());
}

#[wasm_bindgen_test]
fn use_effect_works_many_times() {
    struct UseEffectFunction {}
    impl FunctionProvider for UseEffectFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let (counter, set_counter) = use_state(|| 0);
            let counter_clone = counter.clone();

            use_effect_with_deps(
                move |_| {
                    if *counter_clone < 4 {
                        set_counter(*counter_clone + 1);
                    }
                    || {}
                },
                *counter,
            );

            return html! {
                <div>
                    {"The test result is"}
                    <div id="result">{counter}</div>
                    {"\n"}
                </div>
            };
        }
    }

    type UseEffectComponent = FunctionComponent<UseEffectFunction>;
    let app: App<UseEffectComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result = obtain_result();
    assert_eq!(result.as_str(), "4");
}

#[wasm_bindgen_test]
fn use_effect_works_once() {
    struct UseEffectFunction {}
    impl FunctionProvider for UseEffectFunction {
        type TProps = ();

        fn run(_: &Self::TProps) -> Html {
            let (counter, set_counter) = use_state(|| 0);
            let counter_clone = counter.clone();

            use_effect_with_deps(
                move |_| {
                    set_counter(*counter_clone + 1);
                    || panic!("Destructor should not have been called")
                },
                (),
            );

            return html! {
                <div>
                    {"The test result is"}
                    <div id="result">{counter}</div>
                    {"\n"}
                </div>
            };
        }
    }
    type UseEffectComponent = FunctionComponent<UseEffectFunction>;
    let app: App<UseEffectComponent> = yew::App::new();
    app.mount(yew::utils::document().get_element_by_id("output").unwrap());
    let result = obtain_result();
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
            let (_, set_counter) = use_state(|| 0);
            use_effect_with_deps(
                move |dep| {
                    let mut ref_mut = number_ref_c.borrow_mut();
                    let inner_ref_mut = ref_mut.deref_mut();
                    if *inner_ref_mut < 1 {
                        *inner_ref_mut += 1;
                        assert_eq!(dep, &0);
                    } else {
                        assert_eq!(dep, &1);
                    }
                    set_counter(10); // we just need to make sure it does not panic
                    move || {
                        set_counter(11);
                        *number_ref2_c.borrow_mut().deref_mut() += 1;
                    }
                },
                arg,
            );
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
    let result: String = obtain_result();

    assert_eq!(result.as_str(), "11");
}
