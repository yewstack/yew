use crate::functional::use_hook;
use std::rc::Rc;

struct UseEffect<Destructor> {
    runner: Option<Box<dyn FnOnce() -> Destructor>>,
    destructor: Option<Box<Destructor>>,
}

/// This hook is used for hooking into the component's lifecycle.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseEffect)]
/// fn effect() -> Html {
///     let counter = use_state(|| 0);
///
///     let counter_one = counter.clone();
///     use_effect(move || {
///         // Make a call to DOM API after component is rendered
///         gloo_utils::document().set_title(&format!("You clicked {} times", *counter_one));
///
///         // Perform the cleanup
///         || gloo_utils::document().set_title(&format!("You clicked 0 times"))
///     });
///
///     let onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(*counter + 1))
///     };
///
///     html! {
///         <button {onclick}>{ format!("Increment to {}", *counter) }</button>
///     }
/// }
/// ```
pub fn use_effect<Destructor>(callback: impl FnOnce() -> Destructor + 'static)
where
    Destructor: FnOnce() + 'static,
{
    use_hook(
        move || {
            let effect: UseEffect<Destructor> = UseEffect {
                runner: None,
                destructor: None,
            };
            effect
        },
        |state, updater| {
            state.runner = Some(Box::new(callback) as Box<dyn FnOnce() -> Destructor>);

            // Run on every render
            updater.post_render(move |state: &mut UseEffect<Destructor>| {
                if let Some(callback) = state.runner.take() {
                    if let Some(de) = state.destructor.take() {
                        de();
                    }

                    let new_destructor = callback();
                    state.destructor.replace(Box::new(new_destructor));
                }
                false
            });
        },
        |hook| {
            if let Some(destructor) = hook.destructor.take() {
                destructor()
            }
        },
    )
}

type UseEffectDepsRunnerFn<Dependents, Destructor> = Box<dyn FnOnce(&Dependents) -> Destructor>;

struct UseEffectDeps<Destructor, Dependents> {
    runner_with_deps: Option<(
        Rc<Dependents>,
        UseEffectDepsRunnerFn<Dependents, Destructor>,
    )>,
    destructor: Option<Box<Destructor>>,
    deps: Option<Rc<Dependents>>,
}

/// This hook is similar to [`use_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the effect callback is called again.
/// To detect changes, dependencies must implement `PartialEq`.
/// Note that the destructor also runs when dependencies change.
pub fn use_effect_with_deps<Callback, Destructor, Dependents>(callback: Callback, deps: Dependents)
where
    Callback: FnOnce(&Dependents) -> Destructor + 'static,
    Destructor: FnOnce() + 'static,
    Dependents: PartialEq + 'static,
{
    let deps = Rc::new(deps);

    use_hook(
        move || {
            let destructor: Option<Box<Destructor>> = None;
            UseEffectDeps {
                runner_with_deps: None,
                destructor,
                deps: None,
            }
        },
        move |state, updater| {
            state.runner_with_deps = Some((deps, Box::new(callback)));

            updater.post_render(move |state: &mut UseEffectDeps<Destructor, Dependents>| {
                if let Some((deps, callback)) = state.runner_with_deps.take() {
                    if Some(&deps) == state.deps.as_ref() {
                        return false;
                    }

                    if let Some(de) = state.destructor.take() {
                        de();
                    }

                    let new_destructor = callback(&deps);

                    state.deps = Some(deps);
                    state.destructor = Some(Box::new(new_destructor));
                }
                false
            });
        },
        |hook| {
            if let Some(destructor) = hook.destructor.take() {
                destructor()
            }
        },
    );
}
