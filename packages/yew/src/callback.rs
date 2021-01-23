//! This module contains data types for interacting with `Scope`s.
//!
//! ## Relevant examples
//! - [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
//! - [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because if you call one from the `update` loop
/// of a `Component` (even from JS) it will delay a message until next.
/// Callbacks should be used from JS callbacks or `setTimeout` calls.
/// </aside>
/// An `Rc` wrapper is used to make it cloneable.
pub enum Callback<IN> {
    /// A callback which can be called multiple times
    Callback(Rc<dyn Fn(IN)>),
    /// A callback which can only be called once. The callback will panic if it is
    /// called more than once.
    CallbackOnce(Rc<CallbackOnce<IN>>),
}

type CallbackOnce<IN> = RefCell<Option<Box<dyn FnOnce(IN)>>>;

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback::Callback(Rc::new(func))
    }
}

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        match self {
            Callback::Callback(cb) => Callback::Callback(cb.clone()),
            Callback::CallbackOnce(cb) => Callback::CallbackOnce(cb.clone()),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN> PartialEq for Callback<IN> {
    fn eq(&self, other: &Callback<IN>) -> bool {
        match (&self, &other) {
            (Callback::Callback(cb), Callback::Callback(other_cb)) => Rc::ptr_eq(cb, other_cb),
            (Callback::CallbackOnce(cb), Callback::CallbackOnce(other_cb)) => {
                Rc::ptr_eq(cb, other_cb)
            }
            _ => false,
        }
    }
}

impl<IN> fmt::Debug for Callback<IN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Callback::Callback(_) => "Callback<_>",
            Callback::CallbackOnce(_) => "CallbackOnce<_>",
        };

        f.write_str(data)
    }
}

impl<IN> Callback<IN> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: IN) {
        match self {
            Callback::Callback(cb) => cb(value),
            Callback::CallbackOnce(rc) => {
                let cb = rc.replace(None);
                let f = cb.expect("callback in CallbackOnce has already been used");
                f(value)
            }
        };
    }

    /// Creates a callback from an `FnOnce`. The programmer is responsible for ensuring
    /// that the callback is only called once. If it is called more than once, the callback
    /// will panic.
    pub fn once<F>(func: F) -> Self
    where
        F: FnOnce(IN) + 'static,
    {
        Callback::CallbackOnce(Rc::new(RefCell::new(Some(Box::new(func)))))
    }

    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<Callback>`.
    pub fn noop() -> Self {
        Self::from(|_| {})
    }
}

impl<IN> Default for Callback<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static> Callback<IN> {
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
