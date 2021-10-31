use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::{use_reducer, use_reducer_eq, Reducible, UseReducerHandle};

struct UseStateReducer<T> {
    value: Rc<T>,
}

impl<T> Reducible for UseStateReducer<T> {
    type Action = T;
    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Rc::new(Self {
            value: action.into(),
        })
    }
}

impl<T> PartialEq for UseStateReducer<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.value == rhs.value
    }
}

/// This hook is used to manage state in a function component.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// #[function_component(UseState)]
/// fn state() -> Html {
///     let counter = use_state(|| 0);
///     let onclick = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(*counter + 1))
///     };
///
///
///     html! {
///         <div>
///             <button {onclick}>{ "Increment value" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *counter }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_state<T, F>(init_fn: F) -> UseStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let handle = use_reducer(move || UseStateReducer {
        value: Rc::new(init_fn()),
    });

    UseStateHandle { inner: handle }
}

/// [`use_state`] but only re-renders when `prev_state != next_state`.
///
/// This hook requires the state to implement [`PartialEq`].
pub fn use_state_eq<T, F>(init_fn: F) -> UseStateHandle<T>
where
    T: PartialEq + 'static,
    F: FnOnce() -> T,
{
    let handle = use_reducer_eq(move || UseStateReducer {
        value: Rc::new(init_fn()),
    });

    UseStateHandle { inner: handle }
}

/// State handle for the [`use_state`] hook.
pub struct UseStateHandle<T> {
    inner: UseReducerHandle<UseStateReducer<T>>,
}

impl<T: fmt::Debug> fmt::Debug for UseStateHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseStateHandle")
            .field("value", &format!("{:?}", self.inner.value))
            .finish()
    }
}

impl<T> UseStateHandle<T> {
    /// Replaces the value
    pub fn set(&self, value: T) {
        self.inner.dispatch(value)
    }
}

impl<T> Deref for UseStateHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl<T> Clone for UseStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}
