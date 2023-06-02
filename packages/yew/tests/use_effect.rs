#![cfg(target_arch = "wasm32")]

mod common;

use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::time::Duration;

use common::obtain_result;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;
use yew::prelude::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn use_effect_destroys_on_component_drop() {
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

    #[function_component(UseEffectComponent)]
    fn use_effect_comp(props: &FunctionProps) -> Html {
        let effect_called = props.effect_called.clone();
        let destroy_called = props.destroy_called.clone();
        use_effect_with((), move |_| {
            effect_called();
            #[allow(clippy::redundant_closure)] // Otherwise there is a build error
            move || destroy_called()
        });
        html! {}
    }

    #[function_component(UseEffectWrapperComponent)]
    fn use_effect_wrapper_comp(props: &WrapperProps) -> Html {
        let show = use_state(|| true);
        if *show {
            let effect_called: Rc<dyn Fn()> = { Rc::new(move || show.set(false)) };
            html! {
                <UseEffectComponent destroy_called={props.destroy_called.clone()} {effect_called} />
            }
        } else {
            html! {
                <div>{ "EMPTY" }</div>
            }
        }
    }

    let destroy_counter = Rc::new(std::cell::RefCell::new(0));
    let destroy_counter_c = destroy_counter.clone();
    yew::Renderer::<UseEffectWrapperComponent>::with_root_and_props(
        gloo::utils::document().get_element_by_id("output").unwrap(),
        WrapperProps {
            destroy_called: Rc::new(move || *destroy_counter_c.borrow_mut().deref_mut() += 1),
        },
    )
    .render();

    sleep(Duration::ZERO).await;

    assert_eq!(1, *destroy_counter.borrow().deref());
}

#[wasm_bindgen_test]
async fn use_effect_works_many_times() {
    #[function_component(UseEffectComponent)]
    fn use_effect_comp() -> Html {
        let counter = use_state(|| 0);
        let counter_clone = counter.clone();

        use_effect_with(*counter, move |_| {
            if *counter_clone < 4 {
                counter_clone.set(*counter_clone + 1);
            }
            || {}
        });

        html! {
            <div>
                { "The test result is" }
                <div id="result">{ *counter }</div>
                { "\n" }
            </div>
        }
    }

    yew::Renderer::<UseEffectComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;
    let result = obtain_result();
    assert_eq!(result.as_str(), "4");
}

#[wasm_bindgen_test]
async fn use_effect_works_once() {
    #[function_component(UseEffectComponent)]
    fn use_effect_comp() -> Html {
        let counter = use_state(|| 0);
        let counter_clone = counter.clone();

        use_effect_with((), move |_| {
            counter_clone.set(*counter_clone + 1);
            || panic!("Destructor should not have been called")
        });

        html! {
            <div>
                { "The test result is" }
                <div id="result">{ *counter }</div>
                { "\n" }
            </div>
        }
    }

    yew::Renderer::<UseEffectComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();
    sleep(Duration::ZERO).await;

    let result = obtain_result();

    assert_eq!(result.as_str(), "1");
}

#[wasm_bindgen_test]
async fn use_effect_refires_on_dependency_change() {
    #[function_component(UseEffectComponent)]
    fn use_effect_comp() -> Html {
        let number_ref = use_mut_ref(|| 0);
        let number_ref_c = number_ref.clone();
        let number_ref2 = use_mut_ref(|| 0);
        let number_ref2_c = number_ref2.clone();
        let arg = *number_ref.borrow_mut().deref_mut();
        let counter = use_state(|| 0);
        use_effect_with(arg, move |dep| {
            let mut ref_mut = number_ref_c.borrow_mut();
            let inner_ref_mut = ref_mut.deref_mut();
            if *inner_ref_mut < 1 {
                *inner_ref_mut += 1;
                assert_eq!(dep, &0);
            } else {
                assert_eq!(dep, &1);
            }
            counter.set(10); // we just need to make sure it does not panic
            move || {
                counter.set(11);
                *number_ref2_c.borrow_mut().deref_mut() += 1;
            }
        });
        html! {
            <div>
                {"The test result is"}
                <div id="result">{*number_ref.borrow_mut().deref_mut()}{*number_ref2.borrow_mut().deref_mut()}</div>
                {"\n"}
            </div>
        }
    }

    yew::Renderer::<UseEffectComponent>::with_root(
        gloo::utils::document().get_element_by_id("output").unwrap(),
    )
    .render();

    sleep(Duration::ZERO).await;
    let result: String = obtain_result();

    assert_eq!(result.as_str(), "11");
}
