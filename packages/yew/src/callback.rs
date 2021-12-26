//! This module contains data types for interacting with `Scope`s.
//!
//! ## Relevant examples
//! - [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
//! - [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)

use crate::html::ImplicitClone;
use std::fmt;
use std::rc::Rc;

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because if you call one from the `update` loop
/// of a `Component` (even from JS) it will delay a message until next.
/// Callbacks should be used from JS callbacks or `setTimeout` calls.
/// </aside>
/// An `Rc` wrapper is used to make it cloneable.
pub struct Callback<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(IN) -> OUT>,

    /// Setting `passive` to [Some] explicitly makes the event listener passive or not.
    /// Yew sets sane defaults depending on the type of the listener.
    /// See
    /// [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener).
    pub(crate) passive: Option<bool>,
}

impl<IN, OUT, F: Fn(IN) -> OUT + 'static> From<F> for Callback<IN, OUT> {
    fn from(func: F) -> Self {
        Callback {
            cb: Rc::new(func),
            passive: None,
        }
    }
}

impl<IN, OUT> Clone for Callback<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
            passive: self.passive,
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, OUT> PartialEq for Callback<IN, OUT> {
    fn eq(&self, other: &Callback<IN, OUT>) -> bool {
        match (&self, &other) {
            (
                Callback { cb, passive },
                Callback {
                    cb: rhs_cb,
                    passive: rhs_passive,
                },
            ) => Rc::ptr_eq(cb, rhs_cb) && passive == rhs_passive,
        }
    }
}

impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Callback { .. } => "Callback<_>",
        };

        f.write_str(data)
    }
}

impl<IN, OUT> Callback<IN, OUT> {
    /// Creates a new callback.
    pub fn new(cb: impl Fn(IN) -> OUT + 'static, passive: Option<bool>) -> Self {
        Self {
            cb: Rc::new(cb),
            passive,
        }
    }

    /// This method calls the callback's function.
    pub fn emit(&self, value: IN) -> OUT {
        (*self.cb)(value)
    }
}

impl<IN> Callback<IN> {
    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<Callback>`.
    pub fn noop() -> Self {
        Self::new(|_| (), None)
    }
}
impl<IN> Default for Callback<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static, OUT: 'static> Callback<IN, OUT> {
    /// Changes the input type of the callback to another.
    /// Works like the `map` method but in the opposite direction.
    pub fn reform<F, T>(&self, func: F) -> Callback<T>
    where
        F: Fn(T) -> IN + 'static,
    {
        let this = self.clone();
        let func = move |input| {
            let output = func(input);
            this.emit(output);
        };
        Callback::from(func)
    }
}

impl<IN, OUT> ImplicitClone for Callback<IN, OUT> {}
