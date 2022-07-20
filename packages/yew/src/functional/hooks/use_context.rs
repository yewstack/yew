use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::callback::Callback;
use crate::context::ContextHandle;
use crate::functional::{Hook, HookContext};

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
pub fn use_context<T: Clone + PartialEq + 'static>() -> impl Hook<Output = Option<T>> {
    struct HookProvider<T: Clone + PartialEq + 'static> {
        _marker: PhantomData<T>,
    }

    struct UseContext<T: Clone + PartialEq + 'static> {
        _handle: Option<ContextHandle<T>>,
        value: Rc<RefCell<Option<T>>>,
    }

    impl<T> Hook for HookProvider<T>
    where
        T: Clone + PartialEq + 'static,
    {
        type Output = Option<T>;

        fn run(self, ctx: &mut HookContext) -> Self::Output {
            let scope = ctx.scope.clone();

            let state = ctx.next_state(move |re_render| -> UseContext<T> {
                let value_cell: Rc<RefCell<Option<T>>> = Rc::default();

                let (init_value, handle) = {
                    let value_cell = value_cell.clone();

                    scope.context(Callback::from(move |m| {
                        *(value_cell.borrow_mut()) = Some(m);
                        re_render()
                    }))
                }
                .map(|(value, handle)| (Some(value), Some(handle)))
                .unwrap_or((None, None));

                *(value_cell.borrow_mut()) = init_value;

                UseContext {
                    _handle: handle,
                    value: value_cell,
                }
            });

            let value = state.value.borrow();
            value.clone()
        }
    }

    HookProvider {
        _marker: PhantomData,
    }
}
