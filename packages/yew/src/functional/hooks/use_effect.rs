use crate::functional::{hook, Effect, Hook, HookContext};

struct UseEffectBase<T, F, D>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
{
    runner_with_deps: Option<(T, F)>,
    destructor: Option<D>,
    deps: Option<T>,
    effect_changed_fn: fn(Option<&T>, Option<&T>) -> bool,
}

impl<T, F, D> Effect for UseEffectBase<T, F, D>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
{
    fn rendered(&mut self) {
        if let Some((deps, runner)) = self.runner_with_deps.take() {
            if !(self.effect_changed_fn)(Some(&deps), self.deps.as_ref()) {
                return;
            }

            if let Some(de) = self.destructor.take() {
                de();
            }

            let new_destructor = runner(&deps);

            self.deps = Some(deps);
            self.destructor = Some(new_destructor);
        }
    }
}

impl<T, F, D> Drop for UseEffectBase<T, F, D>
where
    F: FnOnce(&T) -> D + 'static,
    T: 'static,
    D: FnOnce() + 'static,
{
    fn drop(&mut self) {
        if let Some(destructor) = self.destructor.take() {
            destructor()
        }
    }
}

fn use_effect_base<'hook, T, D>(
    runner: impl FnOnce(&T) -> D + 'static,
    deps: T,
    effect_changed_fn: fn(Option<&T>, Option<&T>) -> bool,
) -> impl Hook<'hook, Output = ()>
where
    T: 'static,
    D: FnOnce() + 'static,
{
    struct HookProvider<T, F, D>
    where
        F: FnOnce(&T) -> D + 'static,
        T: 'static,
        D: FnOnce() + 'static,
    {
        runner: F,
        deps: T,
        effect_changed_fn: fn(Option<&T>, Option<&T>) -> bool,
    }

    impl<'hook, T, F, D> Hook<'hook> for HookProvider<T, F, D>
    where
        F: FnOnce(&T) -> D + 'static,
        T: 'static,
        D: FnOnce() + 'static,
    {
        type Output = ();

        fn run(self, ctx: &HookContext) -> Self::Output {
            let Self {
                runner,
                deps,
                effect_changed_fn,
            } = self;

            let state = ctx.next_effect(|| -> UseEffectBase<T, F, D> {
                UseEffectBase {
                    runner_with_deps: None,
                    destructor: None,
                    deps: None,
                    effect_changed_fn,
                }
            });

            state.runner_with_deps = Some((deps, runner));
        }
    }

    HookProvider {
        runner,
        deps,
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
pub fn use_effect<F, D>(f: F)
where
    F: FnOnce() -> D + 'static,
    D: FnOnce() + 'static,
{
    use_effect_base(|_| f(), (), |_, _| true);
}

/// This hook is similar to [`use_effect`] but it accepts dependencies.
///
/// Whenever the dependencies are changed, the effect callback is called again.
/// To detect changes, dependencies must implement `PartialEq`.
/// Note that the destructor also runs when dependencies change.
#[hook]
pub fn use_effect_with_deps<T, F, D>(f: F, deps: T)
where
    T: PartialEq + 'static,
    F: FnOnce(&T) -> D + 'static,
    D: FnOnce() + 'static,
{
    use_effect_base(f, deps, |lhs, rhs| lhs != rhs)
}
