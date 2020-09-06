//! This module contains data types for interacting with `Scope`s.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Flags that modify default callback behaviour
#[derive(Eq, PartialEq, Clone, Copy, std::hash::Hash, Debug)]
pub struct Flags(u8);

impl std::ops::BitAnd for Flags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Flags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::BitOr for Flags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Flags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Apply no Flags to callback. This is different from constructing a callback with None flags as it
/// overrides the defaults for the callback type.
pub const NO_FLAGS: Flags = Flags(0);
/// Defines the event listener as passive.
/// Yew sets sane defaults depending on the type of the listener.
/// See [addEventListener](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEvent).
pub const PASSIVE: Flags = Flags(1);
/// Causes the event handler to not fire until the next animation frame
/// Implies `PASSIVE`.
// TODO: this flag can apply to Agents and Components as well
pub const DEFER: Flags = Flags(1 << 1 | PASSIVE.0);
/// Causes the event handler to not fire until the next animation frame and be called with the last
/// fired event.
/// Implies `PASSIVE` and `DEFER`.
// TODO: this flag can apply to Agents and Components as well
pub const DEBOUNCE: Flags = Flags(1 << 2 | DEFER.0);
/// Defines event listener to also listen to events in the child tree that bubbled up to the target
/// element
#[cfg(feature = "web_sys")]
pub const HANDLE_BUBBLED: Flags = Flags(1 << 3);

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because if you call one from the `update` loop
/// of a `Component` (even from JS) it will delay a message until next.
/// Callbacks should be used from JS callbacks or `setTimeout` calls.
/// </aside>
/// An `Rc` wrapper is used to make it cloneable.
pub enum Callback<IN> {
    /// A callback which can be called multiple times with optional modifier flags
    Callback {
        /// A callback which can be called multiple times
        cb: Rc<dyn Fn(IN)>,

        /// Sets flags for event listening. A combination of any Flags.
        /// If None, the default flags for the callback event source are used.
        ///
        /// Currently only used with `feature = "web_sys"`.
        flags: Option<Flags>,
    },

    /// A callback which can only be called once. The callback will panic if it is
    /// called more than once.
    Once(Rc<Once<IN>>),
}

type Once<IN> = RefCell<Option<Box<dyn FnOnce(IN)>>>;

impl<IN, F: Fn(IN) + 'static> From<F> for Callback<IN> {
    fn from(func: F) -> Self {
        Callback::Callback {
            cb: Rc::new(func),
            flags: None,
        }
    }
}

impl<IN> Clone for Callback<IN> {
    fn clone(&self) -> Self {
        match self {
            Callback::Callback { cb, flags } => Callback::Callback {
                cb: cb.clone(),
                flags: *flags,
            },
            Callback::Once(cb) => Callback::Once(cb.clone()),
        }
    }
}

#[allow(clippy::vtable_address_comparisons)]
impl<IN> PartialEq for Callback<IN> {
    fn eq(&self, other: &Callback<IN>) -> bool {
        match (&self, &other) {
            (Callback::Once(cb), Callback::Once(other_cb)) => Rc::ptr_eq(cb, other_cb),
            (
                Callback::Callback { cb, flags },
                Callback::Callback {
                    cb: rhs_cb,
                    flags: rhs_flags,
                },
            ) => Rc::ptr_eq(cb, rhs_cb) && flags == rhs_flags,
            _ => false,
        }
    }
}

impl<IN> fmt::Debug for Callback<IN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Callback::Callback { .. } => "Callback<_>",
            Callback::Once(_) => "Once<_>",
        };

        f.write_str(data)
    }
}

impl<IN> Callback<IN> {
    /// This method calls the callback's function.
    pub fn emit(&self, value: IN) {
        match self {
            Callback::Callback { cb, .. } => cb(value),
            Callback::Once(rc) => {
                let cb = rc.replace(None);
                let f = cb.expect("callback contains `FnOnce` which has already been used");
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
        Callback::Once(Rc::new(RefCell::new(Some(Box::new(func)))))
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

#[cfg(test)]
pub(crate) mod test_util {
    use super::*;
    use std::cell::RefCell;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    struct CallbackHandle<T> {
        waker: Option<Waker>,
        output: Option<T>,
    }

    impl<T> Default for CallbackHandle<T> {
        fn default() -> Self {
            CallbackHandle {
                waker: None,
                output: None,
            }
        }
    }

    pub(crate) struct CallbackFuture<T>(Rc<RefCell<CallbackHandle<T>>>);

    impl<T> Clone for CallbackFuture<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Default for CallbackFuture<T> {
        fn default() -> Self {
            Self(Rc::default())
        }
    }

    impl<T: 'static> Into<Callback<T>> for CallbackFuture<T> {
        fn into(self) -> Callback<T> {
            Callback::from(move |r| self.finish(r))
        }
    }

    impl<T> Future for CallbackFuture<T> {
        type Output = T;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(output) = self.ready() {
                Poll::Ready(output)
            } else {
                let handle = &self.0;
                handle.borrow_mut().waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }

    impl<T> CallbackFuture<T> {
        fn ready(&self) -> Option<T> {
            self.0.borrow_mut().output.take()
        }

        fn finish(&self, output: T) {
            self.0.borrow_mut().output = Some(output);
            if let Some(waker) = self.0.borrow_mut().waker.take() {
                waker.wake();
            }
        }
    }
}
