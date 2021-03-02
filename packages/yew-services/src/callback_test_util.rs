#![cfg(test)]

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use yew::callback::*;

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

pub struct CallbackFuture<T>(Rc<RefCell<CallbackHandle<T>>>);

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
    pub fn ready(&self) -> Option<T> {
        self.0.borrow_mut().output.take()
    }

    fn finish(&self, output: T) {
        self.0.borrow_mut().output = Some(output);
        if let Some(waker) = self.0.borrow_mut().waker.take() {
            waker.wake();
        }
    }
}
