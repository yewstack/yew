use std::pin::Pin;
use std::task::{Context, Poll};

use futures::channel::mpsc as imp;
use futures::stream::Stream;
use pin_project::pin_project;

use crate::platform::sync::mpsc::SendError;

#[derive(Debug)]
pub(crate) struct UnboundedSender<T> {
    inner: imp::UnboundedSender<T>,
}

#[derive(Debug)]
#[pin_project]
pub(crate) struct UnboundedReceiver<T> {
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

pub(crate) fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let (inner_tx, inner_rx) = imp::unbounded();

    let tx = UnboundedSender { inner: inner_tx };
    let rx = UnboundedReceiver { inner: inner_rx };

    (tx, rx)
}

impl<T> UnboundedSender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.inner
            .unbounded_send(value)
            .map_err(|e| SendError(e.into_inner()))
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.inner.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
