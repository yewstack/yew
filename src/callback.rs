//! This module contains structs to interact with `Scope`s.

use std::fmt;
use std::rc::Rc;

/// Universal callback wrapper.
/// <aside class="warning">
/// Use callbacks carefully, because if you call it from `update` loop
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

    /// Creates a no-op callback which can be used when it is not suitable to use an
    /// `Option<Callback>`.
    pub fn noop() -> Self {
        Self::from(|_| {})
    }
}

impl<IN: 'static> Callback<IN> {
    /// Changes input type of the callback to another.
    /// Works like common `map` method but in an opposite direction.
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
        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if let Some(output) = self.ready() {
                Poll::Ready(output)
            } else {
                self.0.borrow_mut().waker = Some(cx.waker().clone());
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
