use crate::{get_current_scope, use_hook};
use yew::context::ContextHandle;

/// Hook for consuming context values in function components.
/// The context of the type passed as `T` is returned. If there is no such context in scope, `None` is returned.
/// A component which calls `use_context` will re-render when the data of the context changes.
///
/// More information about contexts and how to define and consume them can be found on [Yew Docs](https://yew.rs).
///
/// # Example
/// ```rust
/// # use yew_functional::{function_component, use_context};
/// # use yew::prelude::*;
/// # use std::rc::Rc;
///
/// # #[derive(Clone, Debug, PartialEq)]
/// # struct ThemeContext {
/// #    foreground: String,
/// #    background: String,
/// # }
/// #[function_component(ThemedButton)]
/// pub fn themed_button() -> Html {
///     let theme = use_context::<Rc<ThemeContext>>().expect("no ctx found");
///
///     html! {
///         <button style=format!("background: {}; color: {}", theme.background, theme.foreground)>
///             { "Click me" }
///         </button>
///     }
/// }
/// ```
pub fn use_context<T: Clone + PartialEq + 'static>() -> Option<T> {
    struct UseContextState<T2: Clone + PartialEq + 'static> {
        initialized: bool,
        context: Option<(T2, ContextHandle<T2>)>,
    }

    let scope = get_current_scope()
        .expect("No current Scope. `use_context` can only be called inside function components");

    use_hook(
        move || UseContextState {
            initialized: false,
            context: None,
        },
        |state: &mut UseContextState<T>, updater| {
            if !state.initialized {
                state.initialized = true;
                let callback = move |ctx: T| {
                    updater.callback(|state: &mut UseContextState<T>| {
                        if let Some(context) = &mut state.context {
                            context.0 = ctx;
                        }
                        true
                    });
                };
                state.context = scope.context::<T>(callback.into());
            }

            Some(state.context.as_ref()?.0.clone())
        },
        |state| {
            state.context = None;
        },
    )
}
