//! This module contains data types for interacting with `Scope`s.
//!
//! ## Relevant examples
//! - [Counter](https://github.com/yewstack/yew/tree/master/examples/counter)
//! - [Timer](https://github.com/yewstack/yew/tree/master/examples/timer)

use std::fmt;
use std::rc::Rc;

use crate::html::ImplicitClone;

macro_rules! generate_callback_impls {
    ($callback:ident, $in_ty:ty, $out_var:ident => $out_val:expr) => {
        impl<IN, OUT, F: Fn($in_ty) -> OUT + 'static> From<F> for $callback<IN, OUT> {
            fn from(func: F) -> Self {
                $callback { cb: Rc::new(func) }
            }
        }

        impl<IN, OUT> Clone for $callback<IN, OUT> {
            fn clone(&self) -> Self {
                Self {
                    cb: self.cb.clone(),
                }
            }
        }

        // We are okay with comparisons from different compilation units to result in false
        // not-equal results. This should only lead in the worst-case to some unneeded re-renders.
        #[allow(ambiguous_wide_pointer_comparisons)]
        impl<IN, OUT> PartialEq for $callback<IN, OUT> {
            fn eq(&self, other: &$callback<IN, OUT>) -> bool {
                let ($callback { cb }, $callback { cb: rhs_cb }) = (self, other);
                Rc::ptr_eq(cb, rhs_cb)
            }
        }

        impl<IN, OUT> fmt::Debug for $callback<IN, OUT> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "$callback<_>")
            }
        }

        impl<IN, OUT> $callback<IN, OUT> {
            /// This method calls the callback's function.
            pub fn emit(&self, value: $in_ty) -> OUT {
                (*self.cb)(value)
            }
        }

        impl<IN> $callback<IN> {
            /// Creates a "no-op" callback which can be used when it is not suitable to use an
            /// `Option<$callback>`.
            pub fn noop() -> Self {
                Self::from(|_: $in_ty| ())
            }
        }

        impl<IN> Default for $callback<IN> {
            fn default() -> Self {
                Self::noop()
            }
        }

        impl<IN: 'static, OUT: 'static> $callback<IN, OUT> {
            /// Creates a new [`Callback`] from another callback and a function.
            ///
            /// That when emitted will call that function and will emit the original callback
            pub fn reform<F, T>(&self, func: F) -> Callback<T, OUT>
            where
                F: Fn(T) -> IN + 'static,
            {
                let this = self.clone();
                let func = move |input: T| {
                    #[allow(unused_mut)]
                    let mut $out_var = func(input);
                    this.emit($out_val)
                };
                func.into()
            }

            /// Creates a new [`CallbackRef`] from another callback and a function.
            ///
            /// That when emitted will call that function and will emit the original callback
            pub fn reform_ref<F, T>(&self, func: F) -> CallbackRef<T, OUT>
            where
                F: Fn(&T) -> $in_ty + 'static,
            {
                let this = self.clone();
                let func = move |input: &T| {
                    #[allow(unused_mut)]
                    let mut $out_var = func(input);
                    this.emit($out_val)
                };
                func.into()
            }

            /// Creates a new [`CallbackRefMut`] from another callback and a function.
            ///
            /// That when emitted will call that function and will emit the original callback
            pub fn reform_ref_mut<F, T>(&self, func: F) -> CallbackRefMut<T, OUT>
            where
                F: Fn(&mut T) -> $in_ty + 'static,
            {
                let this = self.clone();
                let func = move |input: &mut T| {
                    #[allow(unused_mut)]
                    let mut $out_var = func(input);
                    this.emit($out_val)
                };
                func.into()
            }

            /// Creates a new [`Callback`] from another callback and a function.
            ///
            /// When emitted will call the function and, only if it returns `Some(value)`, will emit
            /// `value` to the original callback.
            pub fn filter_reform<F, T>(&self, func: F) -> Callback<T, Option<OUT>>
            where
                F: Fn(T) -> Option<IN> + 'static,
            {
                let this = self.clone();
                let func = move |input: T| {
                    func(input).map(
                        #[allow(unused_mut)]
                        |mut $out_var| this.emit($out_val),
                    )
                };
                func.into()
            }

            /// Creates a new [`CallbackRef`] from another callback and a function.
            ///
            /// When emitted will call the function and, only if it returns `Some(value)`, will emit
            /// `value` to the original callback.
            pub fn filter_reform_ref<F, T>(&self, func: F) -> CallbackRef<T, Option<OUT>>
            where
                F: Fn(&T) -> Option<$in_ty> + 'static,
            {
                let this = self.clone();
                let func = move |input: &T| {
                    func(input).map(
                        #[allow(unused_mut)]
                        |mut $out_var| this.emit($out_val),
                    )
                };
                func.into()
            }

            /// Creates a new [`CallbackRefMut`] from another callback and a function.
            ///
            /// When emitted will call the function and, only if it returns `Some(value)`, will emit
            /// `value` to the original callback.
            pub fn filter_reform_ref_mut<F, T>(&self, func: F) -> CallbackRefMut<T, Option<OUT>>
            where
                F: Fn(&mut T) -> Option<$in_ty> + 'static,
            {
                let this = self.clone();
                let func = move |input: &mut T| {
                    func(input).map(
                        #[allow(unused_mut)]
                        |mut $out_var| this.emit($out_val),
                    )
                };
                func.into()
            }
        }

        impl<IN, OUT> ImplicitClone for $callback<IN, OUT> {}
    };
}

/// Universal callback wrapper.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct Callback<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(IN) -> OUT>,
}

generate_callback_impls!(Callback, IN, output => output);

/// Universal callback wrapper with reference in argument.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct CallbackRef<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(&IN) -> OUT>,
}

generate_callback_impls!(CallbackRef, &IN, output => #[allow(clippy::needless_borrow)] &output);

/// Universal callback wrapper with mutable reference in argument.
///
/// An `Rc` wrapper is used to make it cloneable.
pub struct CallbackRefMut<IN, OUT = ()> {
    /// A callback which can be called multiple times
    pub(crate) cb: Rc<dyn Fn(&mut IN) -> OUT>,
}

generate_callback_impls!(CallbackRefMut, &mut IN, output => &mut output);

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

    #[test]
    fn test_ref() {
        let callback: CallbackRef<usize, usize> = CallbackRef::from(|x: &usize| *x);
        assert_eq!(callback.emit(&42), 42);
    }

    #[test]
    fn test_ref_mut() {
        let callback: CallbackRefMut<usize, ()> = CallbackRefMut::from(|x: &mut usize| *x = 42);
        let mut value: usize = 0;
        callback.emit(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_reform_ref() {
        let callback: Callback<usize, usize> = Callback::from(|x: usize| x + 1);
        let reformed: CallbackRef<usize, usize> = callback.reform_ref(|x: &usize| *x + 2);
        assert_eq!(reformed.emit(&42), 45);
    }

    #[test]
    fn test_reform_ref_mut() {
        let callback: CallbackRefMut<usize, ()> = CallbackRefMut::from(|x: &mut usize| *x += 1);
        let reformed: CallbackRefMut<usize, ()> = callback.reform_ref_mut(|x: &mut usize| {
            *x += 2;
            x
        });
        let mut value: usize = 42;
        reformed.emit(&mut value);
        assert_eq!(value, 45);
    }

    #[test]
    fn test_filter_reform_ref() {
        let callback: Callback<usize, usize> = Callback::from(|x: usize| x + 1);
        let reformed: CallbackRef<usize, Option<usize>> =
            callback.filter_reform_ref(|x: &usize| Some(*x + 2));
        assert_eq!(reformed.emit(&42), Some(45));
    }

    #[test]
    fn test_filter_reform_ref_mut() {
        let callback: CallbackRefMut<usize, ()> = CallbackRefMut::from(|x: &mut usize| *x += 1);
        let reformed: CallbackRefMut<usize, Option<()>> =
            callback.filter_reform_ref_mut(|x: &mut usize| {
                *x += 2;
                Some(x)
            });
        let mut value: usize = 42;
        reformed.emit(&mut value).expect("is some");
        assert_eq!(value, 45);
    }
}
