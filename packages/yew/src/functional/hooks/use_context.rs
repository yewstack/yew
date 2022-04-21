use crate::callback::Callback;
use crate::context::ContextHandle;
use crate::functional::{hook, use_component_scope, use_memo, use_state};

/// Hook for consuming context values in function components.
/// The context of the type passed as `T` is returned. If there is no such context in scope, `None`
/// is returned. A component which calls `use_context` will re-render when the data of the context
/// changes.
///
/// More information about contexts and how to define and consume them can be found on [Yew Docs](https://yew.rs).
///
/// # Example
///
/// ```rust
/// use yew::{ContextProvider, function_component, html, use_context, use_state, Html};
///
///
/// /// App theme
/// #[derive(Clone, Debug, PartialEq)]
/// struct Theme {
///     foreground: String,
///     background: String,
/// }
///
/// /// Main component
/// #[function_component]
/// pub fn App() -> Html {
///     let ctx = use_state(|| Theme {
///         foreground: "#000000".to_owned(),
///         background: "#eeeeee".to_owned(),
///     });
///
///     html! {
///         // `ctx` is type `Rc<UseStateHandle<Theme>>` while we need `Theme`
///         // so we deref it.
///         // It derefs to `&Theme`, hence the clone
///         <ContextProvider<Theme> context={(*ctx).clone()}>
///             // Every child here and their children will have access to this context.
///             <Toolbar />
///         </ContextProvider<Theme>>
///     }
/// }
///
/// /// The toolbar.
/// /// This component has access to the context
/// #[function_component]
/// pub fn Toolbar() -> Html {
///     html! {
///         <div>
///             <ThemedButton />
///         </div>
///     }
/// }
///
/// /// Button placed in `Toolbar`.
/// /// As this component is a child of `ThemeContextProvider` in the component tree, it also has access to the context.
/// #[function_component]
/// pub fn ThemedButton() -> Html {
///     let theme = use_context::<Theme>().expect("no ctx found");
///
///     html! {
///         <button style={format!("background: {}; color: {};", theme.background, theme.foreground)}>
///             { "Click me!" }
///         </button>
///     }
/// }
/// ```
#[hook]
pub fn use_context<T: Clone + PartialEq + 'static>() -> Option<T> {
    struct UseContext<T: Clone + PartialEq + 'static> {
        context: Option<(T, ContextHandle<T>)>,
    }

    let scope = use_component_scope();

    let val = use_state(|| -> Option<T> { None });
    let state = {
        let val_dispatcher = val.setter();
        use_memo(
            move |_| UseContext {
                context: scope.context::<T>(Callback::from(move |m| {
                    val_dispatcher.clone().set(Some(m));
                })),
            },
            (),
        )
    };

    // we fallback to initial value if it was not updated.
    (*val)
        .clone()
        .or_else(move || state.context.as_ref().map(|m| m.0.clone()))
}
