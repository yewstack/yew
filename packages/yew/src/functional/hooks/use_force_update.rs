use super::{Hook, HookContext};
use crate::functional::ReRender;
use std::fmt;

/// A handle which can be used to force a re-render of the associated
/// function component.
#[derive(Clone)]
pub struct UseForceUpdate {
    trigger: ReRender,
}

impl fmt::Debug for UseForceUpdate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseForceUpdate").finish()
    }
}

impl UseForceUpdate {
    /// Trigger an unconditional re-render of the associated function component
    pub fn force_update(&self) {
        (self.trigger)()
    }
}

// #![feature(fn_traits)] // required nightly feature to make UseForceUpdate callable directly
// impl Fn<()> for UseForceUpdate {
//     extern "rust-call" fn call(&self, _args: ()) {
//         self.force_update()
//     }
// }

/// This hook is used to manually force a function component to re-render.
///
/// Try to use more specialized hooks, such as [`use_state`] and [`use_reducer`].
/// This hook should only be used when your component depends on external state where you
/// can't subscribe to changes, or as a low-level primitive to enable such a subscription-based
/// approach.
///
/// For example, a large externally managed cache, such as a app-wide cache for GraphQL data
/// should not rerender every component whenever new data arrives, but only those where a query
/// changed.
///
/// If the state of your component is not shared, you should need to use this hook.
///
/// # Example
///
/// This example implements a silly, manually updated display of the current time. The component
/// is rerendered every time the button is clicked. You should usually use a timeout and `use_state`
/// to automatically trigger a re-render every second without having to use this hook.
///
/// ```rust
/// # use yew::prelude::*;
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
pub fn use_force_update() -> impl Hook<Output = UseForceUpdate> {
    struct UseRerenderHook;

    impl Hook for UseRerenderHook {
        type Output = UseForceUpdate;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            UseForceUpdate {
                trigger: ctx.re_render.clone(),
            }
        }
    }

    UseRerenderHook
}
