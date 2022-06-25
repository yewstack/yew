//! A multi-producer, single-receiver channel;

use std::error::Error;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;
use pin_project::pin_project;

use crate::platform::imp::sync::mpsc as imp;

/// The channel has closed when attempting sending.
#[derive(Debug)]
pub struct SendError<T>(pub T);

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "channel closed")
    }
}

impl<T: fmt::Debug> Error for SendError<T> {}

/// An unbounded sender for a multi-producer, single receiver channel.
#[derive(Debug)]
pub struct UnboundedSender<T> {
    inner: imp::UnboundedSender<T>,
}

/// An unbounded receiver for a multi-producer, single receiver channel.
#[derive(Debug)]
#[pin_project]
pub struct UnboundedReceiver<T> {
    #[pin]
    inner: imp::UnboundedReceiver<T>,
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// Creates an unbounded channel.
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (inner_tx, inner_rx) = imp::unbounded();

    let tx = UnboundedSender { inner: inner_tx };
    let rx = UnboundedReceiver { inner: inner_rx };

    (tx, rx)
}

impl<T> UnboundedSender<T> {
    /// Send the value to the receiver.
    #[inline]
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.inner.send(value)
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;

    #[inline]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
