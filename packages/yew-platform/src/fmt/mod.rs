//! Asynchronous utilities to work with `String`s.

use std::future::Future;

use futures::future::{self, MaybeDone};
use futures::stream::{FusedStream, Stream};
use futures::StreamExt;
use pin_project::pin_project;

mod buffer;

pub use buffer::{buffer, BufReader, BufWriter};

/// A buffered asynchronous [`String`] [`Stream`](futures::stream::Stream).
///
/// A BufStream combines a BufWriter - BufReader pair and a resolving future that writes to the
/// buffer and polls the future alongside the buffer.
#[derive(Debug)]
#[pin_project]
pub struct BufStream<F>
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
    pub fn new<C>(f: C) -> Self
    where
        C: FnOnce(BufWriter) -> F,
    {
        let (w, r) = buffer();
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
