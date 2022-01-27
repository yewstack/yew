use std::cell::RefCell;
use std::rc::Rc;

use crate::functional::{hook, Effect, Hook, HookContext};

struct UseEffectBase<T, F, D, R>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
    R: Fn(Option<&T>, Option<&T>) -> bool + 'static,
{
    runner_with_deps: Option<(Rc<T>, F)>,
    destructor: Option<D>,
    deps: Option<Rc<T>>,
    effect_changed_fn: R,
}

impl<T, F, D, R> Effect for RefCell<UseEffectBase<T, F, D, R>>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
    R: Fn(Option<&T>, Option<&T>) -> bool + 'static,
{
    fn rendered(&self) {
        let mut this = self.borrow_mut();

        if let Some((deps, callback)) = this.runner_with_deps.take() {
            if !(this.effect_changed_fn)(Some(&*deps), this.deps.as_deref()) {
                return;
            }

            if let Some(de) = this.destructor.take() {
                de();
            }

            let new_destructor = callback(&deps);

            this.deps = Some(deps);
            this.destructor = Some(new_destructor);
        }
    }
}

impl<T, F, D, R> Drop for UseEffectBase<T, F, D, R>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
    R: Fn(Option<&T>, Option<&T>) -> bool + 'static,
{
    fn drop(&mut self) {
        if let Some(destructor) = self.destructor.take() {
            destructor()
        }
    }
}

fn use_effect_base<T, D>(
    callback: impl FnOnce(&T) -> D + 'static,
    deps: T,
    effect_changed_fn: impl Fn(Option<&T>, Option<&T>) -> bool + 'static,
) -> impl Hook<Output = ()>
where
    T: 'static,
    D: FnOnce() + 'static,
{
    struct HookProvider<T, F, D, R>
    where
        F: FnOnce(&T) -> D + 'static,
        T: 'static,
        D: FnOnce() + 'static,
        R: Fn(Option<&T>, Option<&T>) -> bool + 'static,
    {
        callback: F,
        deps: Rc<T>,
        effect_changed_fn: R,
    }

    impl<T, F, D, R> Hook for HookProvider<T, F, D, R>
    where
        F: FnOnce(&T) -> D + 'static,
        T: 'static,
        D: FnOnce() + 'static,
        R: Fn(Option<&T>, Option<&T>) -> bool + 'static,
    {
        type Output = ();

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let Self {
                callback,
                deps,
                effect_changed_fn,
            } = self;

            let state = ctx.next_effect(|_| -> RefCell<UseEffectBase<T, F, D, R>> {
                RefCell::new(UseEffectBase {
                    runner_with_deps: None,
                    destructor: None,
                    deps: None,
                    effect_changed_fn,
                })
            });

            state.borrow_mut().runner_with_deps = Some((deps, callback));
        }
    }

    HookProvider {
        callback,
        deps: Rc::new(deps),
        effect_changed_fn,
    }
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
