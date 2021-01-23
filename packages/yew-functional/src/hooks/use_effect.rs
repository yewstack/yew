use super::{use_hook, Hook};
use std::{borrow::Borrow, rc::Rc};

/// This hook is used for hooking into the component's lifecycle.
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_effect, use_state};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseEffect)]
/// fn effect() -> Html {
///     let (counter, set_counter) = use_state(|| 0);
///
///     let counter_one = counter.clone();
///     use_effect(move || {
///         // Make a call to DOM API after component is rendered
///         yew::utils::document().set_title(&format!("You clicked {} times", counter_one));
///
///         // Perform the cleanup
///         || yew::utils::document().set_title(&format!("You clicked 0 times"))
///     });
///
///     let onclick = {
///         let counter = Rc::clone(&counter);
///         Callback::from(move |_| set_counter(*counter + 1))
///     };
///
///     html! {
///         <button onclick=onclick>{ format!("Increment to {}", counter) }</button>
///     }
/// }
/// ```
pub fn use_effect<F, Destructor>(callback: F)
where
    F: FnOnce() -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
{
    struct UseEffectState<Destructor> {
        destructor: Option<Box<Destructor>>,
    }
    impl<T: FnOnce() + 'static> Hook for UseEffectState<T> {
        fn tear_down(&mut self) {
            if let Some(destructor) = self.destructor.take() {
                destructor()
            }
        }
    }

    let callback = Box::new(callback);

    use_hook(
        |_: &mut UseEffectState<Destructor>, hook_callback| {
            hook_callback(
                move |state: &mut UseEffectState<Destructor>| {
                    if let Some(de) = state.destructor.take() {
                        de();
                    }
                    let new_destructor = callback();
                    state.destructor.replace(Box::new(new_destructor));
                    false
                },
                true, // run post render
            );
        },
        || UseEffectState { destructor: None },
    );
}

/// This hook is similar to [`use_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the effect callback is called again.
/// To detect changes, dependencies must implement `PartialEq`.
/// Note that the destructor also runs when dependencies change.
pub fn use_effect_with_deps<F, Destructor, Dependents>(callback: F, deps: Dependents)
where
    F: FnOnce(&Dependents) -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
    Dependents: PartialEq + 'static,
{
    struct UseEffectState<Dependents, Destructor> {
        deps: Rc<Dependents>,
        destructor: Option<Box<Destructor>>,
    }
    impl<Dependents, Destructor: FnOnce() + 'static> Hook for UseEffectState<Dependents, Destructor> {
        fn tear_down(&mut self) {
            if let Some(destructor) = self.destructor.take() {
                destructor()
            }
        }
    }

    let deps = Rc::new(deps);
    let deps_c = deps.clone();

    use_hook(
        move |_state: &mut UseEffectState<Dependents, Destructor>, hook_callback| {
            hook_callback(
                move |state: &mut UseEffectState<Dependents, Destructor>| {
                    if state.deps != deps {
                        if let Some(de) = state.destructor.take() {
                            de();
                        }
                        let new_destructor = callback(deps.borrow());
                        state.deps = deps;
                        state.destructor.replace(Box::new(new_destructor));
                    } else if state.destructor.is_none() {
                        state
                            .destructor
                            .replace(Box::new(callback(state.deps.borrow())));
                    }
                    false
                },
                true, // run post render
            );
        },
        || UseEffectState {
            deps: deps_c,
            destructor: None,
        },
    );
}
