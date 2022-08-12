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

enum BufStreamInner {
    Combined {
        buf: String,
    },
    Detached {
        buf: String,
        waker: Option<Waker>,
        done: bool,
    },
}

impl BufStreamInner {
    #[inline]
    const fn new_detached() -> Self {
        Self::Detached {
            buf: String::new(),
            waker: None,
            done: false,
        }
    }

    #[inline]
    const fn new_combined() -> Self {
        Self::Combined { buf: String::new() }
    }

    #[inline]
    fn buf(&self) -> &String {
        match self {
            Self::Combined { ref buf } => buf,
            Self::Detached { ref buf, .. } => buf,
        }
    }

    #[inline]
    fn buf_mut(&mut self) -> &mut String {
        match self {
            Self::Combined { ref mut buf } => buf,
            Self::Detached { ref mut buf, .. } => buf,
        }
    }

    fn wake(&self) {
        match self {
            Self::Combined { .. } => {}
            Self::Detached { ref waker, .. } => {
                if let Some(m) = waker {
                    m.wake_by_ref();
                }
            }
        }
    }

    fn set_waker(&mut self, waker: Waker) {
        match self {
            Self::Combined { .. } => {}
            Self::Detached {
                waker: ref mut current_waker,
                ..
            } => {
                *current_waker = Some(waker);
            }
        }
    }

    fn finish(&mut self) {
        match self {
            Self::Combined { .. } => {}
            Self::Detached {
                ref waker,
                ref mut done,
                ..
            } => {
                *done = true;
                if let Some(m) = waker {
                    m.wake_by_ref();
                }
            }
        }
    }

    fn is_finished(&self) -> Option<bool> {
        match self {
            Self::Combined { .. } => None,
            Self::Detached { ref buf, done, .. } => Some(buf.is_empty() && *done),
        }
    }
}

pub(crate) struct BufWriter {
    inner: Rc<RefCell<BufStreamInner>>,
}

impl Write for BufWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        let mut inner = self.inner.borrow_mut();
        inner.wake();

        let buf = inner.buf_mut();
        if buf.is_empty() {
            buf.reserve(DEFAULT_BUF_SIZE);
        }

        buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();
        inner.wake();

        let buf = inner.buf_mut();
        if buf.is_empty() {
            buf.reserve(DEFAULT_BUF_SIZE);
        }

        buf.write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        let buf = inner.buf_mut();
        if buf.is_empty() {
            buf.reserve(DEFAULT_BUF_SIZE);
        }

        buf.write_fmt(args)
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
        C: FnOnce(BufWriter) -> F,
    {
        let inner = { Rc::new(RefCell::new(BufStreamInner::new_combined())) };

        let resolver = {
            let inner = inner.clone();
            let w = BufWriter { inner };

            future::maybe_done(f(w))
        };

        Self {
            resolver: Some(resolver),
            inner,
        }
    }

    pub fn new_with_resolver<C>(f: C) -> (BufStream<F>, impl Future<Output = ()>)
    where
        C: FnOnce(BufWriter) -> F,
    {
        let inner = { Rc::new(RefCell::new(BufStreamInner::new_detached())) };

        let resolver = {
            let inner = inner.clone();
            let w = {
                let inner = inner.clone();
                BufWriter { inner }
            };

            async move {
                f(w).await;
                inner.borrow_mut().finish();
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

                match (inner.buf().is_empty(), resolver.is_terminated()) {
                    (true, true) => Poll::Ready(None),
                    (true, false) => Poll::Pending,
                    (false, _) => Poll::Ready(Some(inner.buf_mut().split_off(0))),
                }
            }
            None => {
                let mut inner = this.inner.borrow_mut();

                if !inner.buf().is_empty() {
                    return Poll::Ready(Some(inner.buf_mut().split_off(0)));
                }

                if Some(true) == inner.is_finished() {
                    return Poll::Ready(None);
                }

                inner.set_waker(cx.waker().clone());

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
            Some(resolver) => inner.buf().is_empty() && resolver.is_terminated(),
            None => inner.is_finished().unwrap_or_default(),
        }
    }
}
