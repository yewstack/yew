//! This module contains types for I/O functionality.

use std::cell::RefCell;
use std::fmt::{self, Write};
use std::future::Future;
use std::rc::Rc;
use std::task::{Poll, Waker};

use futures::future::{self, FusedFuture, MaybeDone};
use futures::stream::{FusedStream, Stream};
use pin_project::pin_project;

pub(crate) static DEFAULT_BUF_SIZE: usize = 1024;

struct BufStreamInner {
    buf: String,
    waker: Option<Waker>,
    done: bool,
}

pub(crate) struct Writer {
    inner: Rc<RefCell<BufStreamInner>>,
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        if inner.buf.is_empty() {
            inner.buf.reserve(DEFAULT_BUF_SIZE);
        }

        let result = inner.buf.write_str(s);

        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }

        result
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        if inner.buf.is_empty() {
            inner.buf.reserve(DEFAULT_BUF_SIZE);
        }

        let result = inner.buf.write_char(c);

        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }

        result
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        if inner.buf.is_empty() {
            inner.buf.reserve(DEFAULT_BUF_SIZE);
        }

        let result = inner.buf.write_fmt(args);

        if let Some(waker) = inner.waker.take() {
            waker.wake();
        }

        result
    }
}

#[pin_project]
pub(crate) struct BufStream<F>
where
    F: Future<Output = ()>,
{
    #[pin]
    resolver: Option<MaybeDone<F>>,
    inner: Rc<RefCell<BufStreamInner>>,
}

impl<F> BufStream<F>
where
    F: Future<Output = ()>,
{
    pub fn new<C>(f: C) -> Self
    where
        C: FnOnce(Writer) -> F,
    {
        let inner = {
            Rc::new(RefCell::new(BufStreamInner {
                buf: String::new(),
                waker: None,
                done: false,
            }))
        };

        let resolver = {
            let inner = inner.clone();
            let w = Writer { inner };

            future::maybe_done(f(w))
        };

        Self {
            resolver: Some(resolver),
            inner,
        }
    }

    pub fn new_with_resolver<C>(f: C) -> (BufStream<F>, impl Future<Output = ()>)
    where
        C: FnOnce(Writer) -> F,
    {
        let inner = {
            Rc::new(RefCell::new(BufStreamInner {
                buf: String::new(),
                waker: None,
                done: false,
            }))
        };

        let resolver = {
            let inner = inner.clone();
            let w = {
                let inner = inner.clone();

                Writer { inner }
            };

            async move {
                f(w).await;
                inner.borrow_mut().done = true;
            }
        };

        (
            Self {
                resolver: None,
                inner,
            },
            resolver,
        )
    }
}

impl<F> Stream for BufStream<F>
where
    F: Future<Output = ()>,
{
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();

        match this.resolver.as_pin_mut() {
            Some(mut resolver) => {
                let _ = resolver.as_mut().poll(cx);

                let mut inner = this.inner.borrow_mut();

                match (inner.buf.is_empty(), resolver.is_terminated()) {
                    (true, true) => Poll::Ready(None),
                    (true, false) => Poll::Pending,
                    (false, _) => Poll::Ready(Some(inner.buf.split_off(0))),
                }
            }
            None => {
                let mut inner = this.inner.borrow_mut();

                if !inner.buf.is_empty() {
                    return Poll::Ready(Some(inner.buf.split_off(0)));
                }

                if inner.done {
                    return Poll::Ready(None);
                }

                inner.waker = Some(cx.waker().clone());

                Poll::Pending
            }
        }
    }
}

impl<F> FusedStream for BufStream<F>
where
    F: Future<Output = ()>,
{
    fn is_terminated(&self) -> bool {
        let inner = self.inner.borrow();

        match self.resolver.as_ref() {
            Some(resolver) => inner.buf.is_empty() && resolver.is_terminated(),
            None => inner.buf.is_empty() && inner.done,
        }
    }
}
