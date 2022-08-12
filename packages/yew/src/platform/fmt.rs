//! This module contains types for I/O functionality.

use std::cell::RefCell;
use std::fmt::{self, Write};
use std::future::Future;
use std::rc::Rc;
use std::task::{Poll, Waker};

use futures::future::{self, MaybeDone};
use futures::stream::{FusedStream, Stream};
use futures::StreamExt;
use pin_project::pin_project;

pub(crate) static DEFAULT_BUF_SIZE: usize = 1024;

enum BufStreamState {
    Ready,
    Pending(Waker),
    Done,
}

struct BufStreamInner {
    buf: String,
    state: BufStreamState,
}

impl BufStreamInner {
    const fn new() -> Self {
        Self {
            buf: String::new(),
            state: BufStreamState::Ready,
        }
    }

    fn wake(&self) {
        if let BufStreamState::Pending(ref waker) = self.state {
            waker.wake_by_ref();
        }
    }

    fn finish(&mut self) {
        self.wake();
        self.state = BufStreamState::Done;
    }
}

pub(crate) struct BufWriter {
    inner: Rc<RefCell<BufStreamInner>>,
    capacity: usize,
}

impl BufWriter {
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl Write for BufWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }

        let mut inner = self.inner.borrow_mut();
        inner.wake();

        if inner.buf.is_empty() {
            inner.buf.reserve(self.capacity);
        }

        inner.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();
        inner.wake();

        if inner.buf.is_empty() {
            inner.buf.reserve(self.capacity);
        }

        inner.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        if inner.buf.is_empty() {
            inner.buf.reserve(self.capacity);
        }

        inner.buf.write_fmt(args)
    }
}

pub(crate) struct BufStream {
    inner: Rc<RefCell<BufStreamInner>>,
}

impl BufStream {
    pub fn new<C, F>(capacity: usize, f: C) -> (BufStream, impl Future<Output = ()>)
    where
        C: FnOnce(BufWriter) -> F,
        F: Future<Output = ()>,
    {
        let inner = Rc::new(RefCell::new(BufStreamInner::new()));

        let resolver = {
            let inner = inner.clone();
            let w = {
                let inner = inner.clone();
                BufWriter { inner, capacity }
            };

            async move {
                f(w).await;
                inner.borrow_mut().finish();
            }
        };

        (Self { inner }, resolver)
    }
}

impl Stream for BufStream {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut inner = self.inner.borrow_mut();

        if !inner.buf.is_empty() {
            let mut buf = String::new();
            std::mem::swap(&mut buf, &mut inner.buf);

            return Poll::Ready(Some(buf));
        }

        if let BufStreamState::Done = inner.state {
            return Poll::Ready(None);
        }

        inner.state = BufStreamState::Pending(cx.waker().clone());
        Poll::Pending
    }
}

impl FusedStream for BufStream {
    fn is_terminated(&self) -> bool {
        let inner = self.inner.borrow();

        matches!(
            (&inner.state, inner.buf.is_empty()),
            (BufStreamState::Done, true)
        )
    }
}

#[pin_project]
pub(crate) struct ResolvedBufStream<F>
where
    F: Future<Output = ()>,
{
    #[pin]
    resolver: MaybeDone<F>,
    inner: BufStream,
}

impl<F> ResolvedBufStream<F>
where
    F: Future<Output = ()>,
{
    pub fn new<C>(capacity: usize, f: C) -> ResolvedBufStream<impl Future<Output = ()>>
    where
        C: FnOnce(BufWriter) -> F,
        F: Future<Output = ()>,
    {
        let (inner, resolver) = BufStream::new(capacity, f);

        ResolvedBufStream {
            inner,
            resolver: future::maybe_done(resolver),
        }
    }
}

impl<F> Stream for ResolvedBufStream<F>
where
    F: Future<Output = ()>,
{
    type Item = String;

    #[inline]
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        let _ = this.resolver.poll(cx);

        this.inner.poll_next_unpin(cx)
    }
}

impl<F> FusedStream for ResolvedBufStream<F>
where
    F: Future<Output = ()>,
{
    #[inline]
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
