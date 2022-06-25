use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::stream::Stream;

use crate::platform::sync::mpsc::SendError;

#[derive(Debug)]
pub(crate) struct UnboundedSender<T> {
    inner: PhantomData<T>,
}

#[derive(Debug)]
pub(crate) struct UnboundedReceiver<T> {
    inner: PhantomData<T>,
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        Self { inner: PhantomData }
    }
}

pub(crate) fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    panic!(
        r#"No runtime configured for this platform, features that requires task spawning can't be used.
                Either compile with `target_arch = "wasm32", or enable the `tokio` feature."#
    );
}

impl<T> UnboundedSender<T> {
    pub fn send(&self, _value: T) -> Result<(), SendError<T>> {
        unimplemented!();
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unimplemented!();
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        unimplemented!();
    }
}
