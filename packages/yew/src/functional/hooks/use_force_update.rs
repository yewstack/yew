use std::fmt;

use super::{Hook, HookContext};
use crate::functional::ReRender;

/// A handle which can be used to force a re-render of the associated
/// function component.
#[derive(Clone)]
pub struct UseForceUpdateHandle {
    trigger: ReRender,
}

impl fmt::Debug for UseForceUpdateHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseForceUpdate").finish()
    }
}

impl UseForceUpdateHandle {
    /// Trigger an unconditional re-render of the associated function component
    pub fn force_update(&self) {
        (self.trigger)()
    }
}

#[cfg(nightly_yew)]
mod feat_nightly {
    use super::*;

    impl FnOnce<()> for UseForceUpdateHandle {
        type Output = ();

        extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
            self.force_update()
        }
    }

    impl FnMut<()> for UseForceUpdateHandle {
        extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
            self.force_update()
        }
    }

    impl Fn<()> for UseForceUpdateHandle {
        extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
            self.force_update()
        }
    }
}

/// This hook is used to manually force a function component to re-render.
///
/// # Note
///
/// Often, using this hook means that you're doing something wrong.
/// Try to use more specialized hooks, such as [`use_state`] and [`use_reducer`].
/// This hook should only be used when your component depends on external state where you
/// can't subscribe to changes, or as a low-level primitive to enable such a subscription-based
/// approach.
///
/// # Use-case
///
/// Use this hook when wrapping an API that doesn't expose precise subscription events for fetched
/// data. You could then, at some point, invalidate your local cache of the fetched data and trigger
/// a re-render to let the normal render flow of components tell you again which data to fetch, and
/// repopulate the cache accordingly.
///
/// A large externally managed cache, such as a app-wide cache for GraphQL data
/// should not rerender every component whenever new data arrives, but only those where a query
/// changed.
///
/// If the state of your component is not shared, you should need to use this hook.
///
/// # Example
///
/// This example implements a silly, manually updated display of the current time. The component
/// is re-rendered every time the button is clicked. You should usually use a timeout and
/// `use_state` to automatically trigger a re-render every second without having to use this hook.
///
/// ```rust
/// use yew::prelude::*;
///
/// #[function_component]
/// fn ManuallyUpdatedDate() -> Html {
///     let trigger = use_force_update();
///     let onclick = use_state(move || Callback::from(move |_| trigger.force_update()));
///     let last_update = js_sys::Date::new_0().to_utc_string();
///     html! {
///         <div>
///             <button onclick={&*onclick}>{"Update now!"}</button>
///             <p>{"Last updated: "}{last_update}</p>
///         </div>
///     }
/// }
/// ```
///
/// [`use_state`]: super::use_state()
/// [`use_reducer`]: super::use_reducer()
pub fn use_force_update() -> impl Hook<Output = UseForceUpdateHandle> {
    struct UseRerenderHook;

    impl Hook for UseRerenderHook {
        type Output = UseForceUpdateHandle;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            UseForceUpdateHandle {
                trigger: ctx.re_render.clone(),
            }
        }
    }

    UseRerenderHook
}

#[cfg(all(test, nightly_yew))]
mod nightly_test {
    use yew::prelude::*;

    #[function_component]
    fn ManuallyUpdatedDate() -> Html {
        let trigger = use_force_update();
        let _ = move || trigger();
        html! {}
    }
}
