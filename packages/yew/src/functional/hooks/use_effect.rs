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
    let callback = Box::new(callback);
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

type UseEffectDepsRunnerFn<Dependents, Destructor> =
    Box<dyn FnOnce() -> (Destructor, Rc<Dependents>)>;

struct UseEffectDeps<Destructor, Dependents> {
    runner: Option<UseEffectDepsRunnerFn<Dependents, Destructor>>,
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
                runner: None,
                destructor,
                deps: None,
            }
        },
        move |state, updater| {
            if state.deps.as_ref() != Some(&deps) {
                let runner = move || (callback(&deps), deps);

                state.runner =
                    Some(Box::new(runner) as UseEffectDepsRunnerFn<Dependents, Destructor>);
            }

            updater.post_render(move |state: &mut UseEffectDeps<Destructor, Dependents>| {
                if let Some(callback) = state.runner.take() {
                    if let Some(de) = state.destructor.take() {
                        de();
                    }

                    let (new_destructor, deps) = callback();

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
