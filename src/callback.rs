//! This module contains structs to interact with `Scope`s.

use std::fmt;
use std::rc::Rc;

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because it you call it from `update` loop
/// of `Components` (even from JS) it will delay a message until next.
/// Callbacks should be used from JS callbacks or `setTimeout` calls.
/// </aside>
/// `Rc` wrapper used to make it clonable.
pub struct Callback<IN>(Rc<dyn Fn(IN)>);

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        Callback(self.0.clone())
    }
}

impl<IN> PartialEq for Callback<IN> {
    fn eq(&self, other: &Callback<IN>) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<IN> fmt::Debug for Callback<IN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Callback<_>")
    }
}

impl<IN> Callback<IN> {
    /// This method calls the actual callback.
    pub fn emit(&self, value: IN) {
        (self.0)(value);
    }
}

impl<IN: 'static> Callback<IN> {
    /// Changes input type of the callback to another.
    /// Works like common `map` method but in an opposite direction.
    pub fn reform<F, T>(self, func: F) -> Callback<T>
    where
        F: Fn(T) -> IN + 'static,
    {
        let func = move |input| {
            let output = func(input);
            self.clone().emit(output);
        };
        Callback::from(func)
    }
}
