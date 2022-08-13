//! Asynchronous utilities to work with `String`s.

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
    #[inline]
    const fn new() -> Self {
        Self {
            buf: String::new(),
            state: BufStreamState::Ready,
        }
    }

    #[inline]
    fn wake(&self) {
        if let BufStreamState::Pending(ref waker) = self.state {
            waker.wake_by_ref();
        }
    }

    #[inline]
    fn try_reserve(&mut self, capacity: usize) {
        if self.buf.is_empty() {
            self.buf.reserve(capacity);
        }
    }
}

pub(crate) struct BufWriter {
    inner: Rc<RefCell<BufStreamInner>>,
    capacity: usize,
}

impl BufWriter {
    #[inline]
    pub const fn capacity(&self) -> usize {
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
        inner.try_reserve(self.capacity);

        inner.buf.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        inner.wake();
        inner.try_reserve(self.capacity);

        inner.buf.write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        let mut inner = self.inner.borrow_mut();

        inner.wake();
        inner.try_reserve(self.capacity);

        inner.buf.write_fmt(args)
    }
}

impl Drop for BufWriter {
    fn drop(&mut self) {
        let mut inner = self.inner.borrow_mut();

        inner.wake();
        inner.state = BufStreamState::Done;
    }
}

/// Creates an asynchronous buffer that operates over String.
pub(crate) fn buffer(capacity: usize) -> (BufWriter, BufReader) {
    let inner = Rc::new(RefCell::new(BufStreamInner::new()));

    let w = {
        let inner = inner.clone();
        BufWriter { inner, capacity }
    };

    let r = BufReader { inner };

    (w, r)
}

pub(crate) struct BufReader {
    inner: Rc<RefCell<BufStreamInner>>,
}

impl Stream for BufReader {
    type Item = String;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut inner = self.inner.borrow_mut();

        if !inner.buf.is_empty() {
            let buf = std::mem::take(&mut inner.buf);
            return Poll::Ready(Some(buf));
        }

        if let BufStreamState::Done = inner.state {
            return Poll::Ready(None);
        }

        inner.state = BufStreamState::Pending(cx.waker().clone());
        Poll::Pending
    }
}

impl FusedStream for BufReader {
    fn is_terminated(&self) -> bool {
        let inner = self.inner.borrow();

        matches!(
            (&inner.state, inner.buf.is_empty()),
            (BufStreamState::Done, true)
        )
    }
}

/// A buffered asynchronous string Stream.
///
/// This combines a BufWriter - BufReader pair and a resolving future.
/// The resoloving future will be polled as the stream is polled.
#[pin_project]
pub(crate) struct BufStream<F>
where
    F: Future<Output = ()>,
{
    #[pin]
    resolver: MaybeDone<F>,
    inner: BufReader,
}

impl<F> BufStream<F>
where
    F: Future<Output = ()>,
{
    /// Creates a `BufStream`.
    pub fn new<C>(capacity: usize, f: C) -> Self
    where
        C: FnOnce(BufWriter) -> F,
        F: Future<Output = ()>,
    {
        let (w, r) = buffer(capacity);
        let resolver = future::maybe_done(f(w));

        BufStream { inner: r, resolver }
    }
}

impl<F> Stream for BufStream<F>
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

impl<F> FusedStream for BufStream<F>
where
    F: Future<Output = ()>,
{
    #[inline]
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
