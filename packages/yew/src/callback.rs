//! This module contains data types for interacting with `Scope`s.
//!
//! ## Relevant examples
//! - [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
//! - [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)

use std::fmt;
use std::rc::Rc;

use crate::html::ImplicitClone;

/// Universal callback wrapper.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct Callback<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(IN) -> OUT>,
}

impl<IN, OUT, F: Fn(IN) -> OUT + 'static> From<F> for Callback<IN, OUT> {
    fn from(func: F) -> Self {
        Callback { cb: Rc::new(func) }
    }
}

impl<IN, OUT> Clone for Callback<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, OUT> PartialEq for Callback<IN, OUT> {
    fn eq(&self, other: &Callback<IN, OUT>) -> bool {
        let (Callback { cb }, Callback { cb: rhs_cb }) = (self, other);
        Rc::ptr_eq(cb, rhs_cb)
    }
}

impl<IN, OUT> fmt::Debug for Callback<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Callback<_>")
    }
}

impl<IN, OUT> Callback<IN, OUT> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: IN) -> OUT {
        (*self.cb)(value)
    }
}

impl<IN> Callback<IN> {
    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<Callback>`.
    pub fn noop() -> Self {
        Self::from(|_| ())
    }
}

impl<IN> Default for Callback<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static, OUT: 'static> Callback<IN, OUT> {
    /// Creates a new callback from another callback and a function
    /// That when emitted will call that function and will emit the original callback
    pub fn reform<F, T>(&self, func: F) -> Callback<T, OUT>
    where
        F: Fn(T) -> IN + 'static,
    {
        let this = self.clone();
        let func = move |input| {
            let output = func(input);
            this.emit(output)
        };
        Callback::from(func)
    }

    /// Creates a new callback from another callback and a function.
    /// When emitted will call the function and, only if it returns `Some(value)`, will emit
    /// `value` to the original callback.
    pub fn filter_reform<F, T>(&self, func: F) -> Callback<T, Option<OUT>>
    where
        F: Fn(T) -> Option<IN> + 'static,
    {
        let this = self.clone();
        let func = move |input| func(input).map(|output| this.emit(output));
        Callback::from(func)
    }
}

impl<IN, OUT> ImplicitClone for Callback<IN, OUT> {}

/// Universal callback wrapper with reference in argument.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct CallbackRef<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(&IN) -> OUT>,
}

impl<IN, OUT, F: Fn(&IN) -> OUT + 'static> From<F> for CallbackRef<IN, OUT> {
    fn from(func: F) -> Self {
        CallbackRef { cb: Rc::new(func) }
    }
}

impl<IN, OUT> Clone for CallbackRef<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, OUT> PartialEq for CallbackRef<IN, OUT> {
    fn eq(&self, other: &CallbackRef<IN, OUT>) -> bool {
        let (CallbackRef { cb }, CallbackRef { cb: rhs_cb }) = (self, other);
        Rc::ptr_eq(cb, rhs_cb)
    }
}

impl<IN, OUT> fmt::Debug for CallbackRef<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CallbackRef<_>")
    }
}

impl<IN, OUT> CallbackRef<IN, OUT> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: &IN) -> OUT {
        (*self.cb)(value)
    }
}

impl<IN> CallbackRef<IN> {
    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<CallbackRef>`.
    pub fn noop() -> Self {
        Self::from(|_: &_| ())
    }
}

impl<IN> Default for CallbackRef<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static, OUT: 'static> CallbackRef<IN, OUT> {
    /// Creates a new callback from another callback and a function
    /// That when emitted will call that function and will emit the original callback
    pub fn reform<F, T>(&self, func: F) -> CallbackRef<T, OUT>
    where
        F: Fn(&T) -> &IN + 'static,
    {
        let this = self.clone();
        let func = move |input: &_| {
            let output = func(input);
            this.emit(output)
        };
        CallbackRef::from(func)
    }

    /// Creates a new callback from another callback and a function.
    /// When emitted will call the function and, only if it returns `Some(value)`, will emit
    /// `value` to the original callback.
    pub fn filter_reform<F, T>(&self, func: F) -> CallbackRef<T, Option<OUT>>
    where
        F: Fn(&T) -> Option<&IN> + 'static,
    {
        let this = self.clone();
        let func = move |input: &_| func(input).map(|output| this.emit(output));
        CallbackRef::from(func)
    }
}

impl<IN, OUT> ImplicitClone for CallbackRef<IN, OUT> {}

/// Universal callback wrapper with mutable reference in argument.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct CallbackRefMut<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(&mut IN) -> OUT>,
}

impl<IN, OUT, F: Fn(&mut IN) -> OUT + 'static> From<F> for CallbackRefMut<IN, OUT> {
    fn from(func: F) -> Self {
        CallbackRefMut { cb: Rc::new(func) }
    }
}

impl<IN, OUT> Clone for CallbackRefMut<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            cb: self.cb.clone(),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN, OUT> PartialEq for CallbackRefMut<IN, OUT> {
    fn eq(&self, other: &CallbackRefMut<IN, OUT>) -> bool {
        let (CallbackRefMut { cb }, CallbackRefMut { cb: rhs_cb }) = (self, other);
        Rc::ptr_eq(cb, rhs_cb)
    }
}

impl<IN, OUT> fmt::Debug for CallbackRefMut<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CallbackRefMut<_>")
    }
}

impl<IN, OUT> CallbackRefMut<IN, OUT> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: &mut IN) -> OUT {
        (*self.cb)(value)
    }
}

impl<IN> CallbackRefMut<IN> {
    /// Creates a "no-op" callback which can be used when it is not suitable to use an
    /// `Option<CallbackRefMut>`.
    pub fn noop() -> Self {
        Self::from(|_: &mut _| ())
    }
}

impl<IN> Default for CallbackRefMut<IN> {
    fn default() -> Self {
        Self::noop()
    }
}

impl<IN: 'static, OUT: 'static> CallbackRefMut<IN, OUT> {
    /// Creates a new callback from another callback and a function
    /// That when emitted will call that function and will emit the original callback
    pub fn reform<F, T>(&self, func: F) -> CallbackRefMut<T, OUT>
    where
        F: Fn(&mut T) -> &mut IN + 'static,
    {
        let this = self.clone();
        let func = move |input: &mut _| {
            let output = func(input);
            this.emit(output)
        };
        CallbackRefMut::from(func)
    }

    /// Creates a new callback from another callback and a function.
    /// When emitted will call the function and, only if it returns `Some(value)`, will emit
    /// `value` to the original callback.
    pub fn filter_reform<F, T>(&self, func: F) -> CallbackRefMut<T, Option<OUT>>
    where
        F: Fn(&mut T) -> Option<&mut IN> + 'static,
    {
        let this = self.clone();
        let func = move |input: &mut _| func(input).map(|output| this.emit(output));
        CallbackRefMut::from(func)
    }
}

impl<IN, OUT> ImplicitClone for CallbackRefMut<IN, OUT> {}

#[cfg(test)]
mod test {
    use std::sync::Mutex;

    use super::*;

    /// emit the callback with the provided value
    fn emit<T, I, R: 'static + Clone, F, OUT>(values: I, f: F) -> Vec<R>
    where
        I: IntoIterator<Item = T>,
        F: FnOnce(Callback<R, ()>) -> Callback<T, OUT>,
    {
        let result = Rc::new(Mutex::new(Vec::new()));
        let cb_result = result.clone();
        let cb = f(Callback::<R, ()>::from(move |v| {
            cb_result.lock().unwrap().push(v);
        }));
        for value in values {
            cb.emit(value);
        }
        let x = result.lock().unwrap().clone();
        x
    }

    #[test]
    fn test_callback() {
        assert_eq!(*emit([true, false], |cb| cb), vec![true, false]);
    }

    #[test]
    fn test_reform() {
        assert_eq!(
            *emit([true, false], |cb| cb.reform(|v: bool| !v)),
            vec![false, true]
        );
    }

    #[test]
    fn test_filter_reform() {
        assert_eq!(
            *emit([1, 2, 3], |cb| cb.filter_reform(|v| match v {
                1 => Some(true),
                2 => Some(false),
                _ => None,
            })),
            vec![true, false]
        );
    }
}
