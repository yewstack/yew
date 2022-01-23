use crate::functional::{hook, use_hook};
use std::rc::Rc;

type UseEffectRunnerFn<T, D> = Box<dyn FnOnce(&T) -> D>;

struct UseEffectBase<T, D> {
    runner_with_deps: Option<(Rc<T>, UseEffectRunnerFn<T, D>)>,
    destructor: Option<Box<D>>,
    deps: Option<Rc<T>>,
}

#[hook]
fn use_effect_base<T, D, R>(callback: impl FnOnce(&T) -> D + 'static, deps: T, effect_changed_fn: R)
where
    T: 'static,
    D: FnOnce() + 'static,
    R: FnOnce(Option<&T>, Option<&T>) -> bool + 'static,
{
    let deps = Rc::new(deps);

    use_hook(
        move || {
            let destructor: Option<Box<D>> = None;
            UseEffectBase {
                runner_with_deps: None,
                destructor,
                deps: None,
            }
        },
        move |state, updater| {
            state.runner_with_deps = Some((deps, Box::new(callback)));

            updater.post_render(move |state: &mut UseEffectBase<T, D>| {
                if let Some((deps, callback)) = state.runner_with_deps.take() {
                    if !effect_changed_fn(Some(&*deps), state.deps.as_deref()) {
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
#[hook]
pub fn use_effect<D>(callback: impl FnOnce() -> D + 'static)
where
    D: FnOnce() + 'static,
{
    use_effect_base(|_| callback(), (), |_, _| true);
}

/// This hook is similar to [`use_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the effect callback is called again.
/// To detect changes, dependencies must implement `PartialEq`.
/// Note that the destructor also runs when dependencies change.
#[hook]
pub fn use_effect_with_deps<T, D>(callback: impl FnOnce(&T) -> D + 'static, deps: T)
where
    T: PartialEq + 'static,
    D: FnOnce() + 'static,
{
    use_effect_base(callback, deps, |lhs, rhs| lhs != rhs)
}
