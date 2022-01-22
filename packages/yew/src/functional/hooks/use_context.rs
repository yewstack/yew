use crate::callback::Callback;
use crate::context::ContextHandle;
use crate::functional::{hook, use_component_scope, use_memo, use_state};

/// Hook for consuming context values in function components.
/// The context of the type passed as `T` is returned. If there is no such context in scope, `None` is returned.
/// A component which calls `use_context` will re-render when the data of the context changes.
///
/// More information about contexts and how to define and consume them can be found on [Yew Docs](https://yew.rs).
///
/// # Example
/// ```rust
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
///         <button style={format!("background: {}; color: {}", theme.background, theme.foreground)}>
///             { "Click me" }
///         </button>
///     }
/// }
/// ```
#[hook]
pub fn use_context<T: Clone + PartialEq + 'static>() -> Option<T> {
    struct State<T: Clone + PartialEq + 'static> {
        context: Option<(T, ContextHandle<T>)>,
    }

    impl<T> PartialEq for State<T>
    where
        T: Clone + PartialEq + 'static,
    {
        fn eq(&self, rhs: &Self) -> bool {
            self.context.as_ref().map(|m| &m.0) == rhs.context.as_ref().map(|m| &m.0)
        }
    }

    let scope = use_component_scope();

    let val = use_state(|| -> Option<T> { None });
    let state = {
        let val_dispatcher = val.setter();
        use_memo(
            move |_| State {
                context: scope.context::<T>(Callback::from(move |m| {
                    val_dispatcher.clone().set(Some(m));
                })),
            },
            (),
        )
    };

    // we fallback to initial value if it was not overriden.
    (*val)
        .clone()
        .or_else(move || state.context.as_ref().map(|m| m.0.clone()))
}
