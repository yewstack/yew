//! A multi-producer single-receiver channel.

use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{Poll, Waker};

use futures::sink::Sink;
use futures::stream::{FusedStream, Stream};
use thiserror::Error;

/// Error returned by [`try_next`](UnboundedReceiver::try_next).
#[derive(Error, Debug)]
#[error("queue is empty")]
pub struct TryRecvError {
    _marker: PhantomData<()>,
}

/// Error returned by [`send_now`](UnboundedSender::send_now).
#[derive(Error, Debug)]
#[error("failed to send")]
pub struct SendError<T> {
    /// The send value.
    pub inner: T,
}

/// Error returned by [`UnboundedSender`] when used as a [`Sink`](futures::sink::Sink).
#[derive(Error, Debug)]
#[error("failed to send")]
pub struct TrySendError {
    _marker: PhantomData<()>,
}

#[derive(Debug)]
struct Inner<T> {
    rx_waker: Option<Waker>,
    closed: bool,
    sender_ctr: usize,
    items: VecDeque<T>,
}

impl<T> Inner<T> {
    /// Creates a unchecked mutable reference from an immutable reference.
    ///
    /// SAFETY: You can only use this when:
    ///
    /// 1. The mutable reference is released at the end of a function call.
    /// 2. No parent function has acquired the mutable reference.
    /// 3. The caller is not an async function / the mutable reference is released before an await
    /// statement.
    #[inline]
    unsafe fn get_mut_unchecked(&self) -> *mut Self {
        self as *const Self as *mut Self
    }

    fn close(&mut self) {
        self.closed = true;

        if let Some(ref m) = self.rx_waker {
            m.wake_by_ref();
        }
    }
}

/// The receiver of an unbounded mpsc channel.
#[derive(Debug)]
pub struct UnboundedReceiver<T> {
    inner: Rc<Inner<T>>,
}

impl<T> UnboundedReceiver<T> {
    /// Try to read the next value from the channel.
    ///
    /// This function will return:
    /// - `Ok(Some(T))` if a value is ready.
    /// - `Ok(None)` if the channel has become closed.
    /// - `Err(TryRecvError)` if the channel is not closed and the channel is empty.
    pub fn try_next(&self) -> std::result::Result<Option<T>, TryRecvError> {
        // SAFETY: This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();

            match ((*inner).items.pop_front(), (*inner).closed) {
                (Some(m), _) => Ok(Some(m)),
                (None, false) => Ok(None),
                (None, true) => Err(TryRecvError {
                    _marker: PhantomData,
                }),
            }
        }
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // SAFETY: This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();

            match ((*inner).items.pop_front(), (*inner).closed) {
                (Some(m), _) => Poll::Ready(Some(m)),
                (None, false) => {
                    (*inner).rx_waker = Some(cx.waker().clone());
                    Poll::Pending
                }
                (None, true) => Poll::Ready(None),
            }
        }
    }
}

impl<T> FusedStream for UnboundedReceiver<T> {
    fn is_terminated(&self) -> bool {
        self.inner.items.is_empty() && self.inner.closed
    }
}

impl<T> Drop for UnboundedReceiver<T> {
    fn drop(&mut self) {
        // SAFETY: This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();
            (*inner).close();
        }
    }
}

/// The sender of an unbounded mpsc channel.
#[derive(Debug)]
pub struct UnboundedSender<T> {
    inner: Rc<Inner<T>>,
}

impl<T> UnboundedSender<T> {
    /// Sends a value to the unbounded receiver.
    pub fn send_now(&self, item: T) -> Result<(), SendError<T>> {
        // SAFETY: This function is not used by any function that have already acquired a mutable
        // reference.
        unsafe {
            let inner = &mut *self.inner.get_mut_unchecked();

            if (*inner).closed {
                return Err(SendError { inner: item });
            }

            (*inner).items.push_back(item);

            if let Some(ref m) = (*inner).rx_waker {
                m.wake_by_ref();
            }
        }

        Ok(())
    }

    /// Closes the channel.
    pub fn close_now(&self) {
        // SAFETY: This function is not used by any other functions that have acquired a mutable
        // reference and hence uniquely owns the mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();
            (*inner).close();
        }
    }
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        let self_ = Self {
            inner: self.inner.clone(),
        };

        // SAFETY: This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();
            (*inner).sender_ctr += 1;
        }

        self_
    }
}

impl<T> Drop for UnboundedSender<T> {
    fn drop(&mut self) {
        // SAFETY: This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        unsafe {
            let inner = self.inner.get_mut_unchecked();

            let sender_ctr = {
                (*inner).sender_ctr -= 1;
                (*inner).sender_ctr
            };

            if sender_ctr == 0 {
                (*inner).close();
            }
        }
    }
}

impl<T> Sink<T> for &'_ UnboundedSender<T> {
    type Error = TrySendError;

    fn start_send(self: std::pin::Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.send_now(item).map_err(|_| TrySendError {
            _marker: PhantomData,
        })
    }

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        match self.inner.closed {
            false => Poll::Ready(Ok(())),
            true => Poll::Ready(Err(TrySendError {
                _marker: PhantomData,
            })),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.close_now();

        Poll::Ready(Ok(()))
    }
}

/// Creates an unbounded channel.
///
/// # Note
///
/// This channel has an infinite buffer and can run out of memory if the channel is not actively
/// drained.
pub fn unbounded<T>() -> (UnboundedSender<T>, UnboundedReceiver<T>) {
    let inner = Rc::new(Inner {
        rx_waker: None,
        closed: false,

        sender_ctr: 1,
        items: VecDeque::new(),
    });

    (
        UnboundedSender {
            inner: inner.clone(),
        },
        UnboundedReceiver { inner },
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::sink::SinkExt;
    use futures::stream::StreamExt;
    use tokio::task::LocalSet;
    use tokio::test;

    use super::*;
    use crate::platform::spawn_local;
    use crate::platform::time::sleep;

    #[test]
    async fn mpsc_works() {
        let local_set = LocalSet::new();

        local_set
            .run_until(async {
                let (tx, mut rx) = unbounded::<usize>();

                spawn_local(async move {
                    for i in 0..10 {
                        (&tx).send(i).await.expect("failed to send.");
                        sleep(Duration::from_millis(1)).await;
                    }
                });

                for i in 0..10 {
                    let received = rx.next().await.expect("failed to receive");

                    assert_eq!(i, received);
                }

                assert_eq!(rx.next().await, None);
            })
            .await;
    }

    #[test]
    async fn mpsc_drops_receiver() {
        let (tx, rx) = unbounded::<usize>();
        drop(rx);

        (&tx).send(0).await.expect_err("should fail to send.");
    }

    #[test]
    async fn mpsc_multi_sender() {
        let local_set = LocalSet::new();

        local_set
            .run_until(async {
                let (tx, mut rx) = unbounded::<usize>();

                spawn_local(async move {
                    let tx2 = tx.clone();

                    for i in 0..10 {
                        if i % 2 == 0 {
                            (&tx).send(i).await.expect("failed to send.");
                        } else {
                            (&tx2).send(i).await.expect("failed to send.");
                        }

                        sleep(Duration::from_millis(1)).await;
                    }

                    drop(tx2);

                    for i in 10..20 {
                        (&tx).send(i).await.expect("failed to send.");

                        sleep(Duration::from_millis(1)).await;
                    }
                });

                for i in 0..20 {
                    let received = rx.next().await.expect("failed to receive");

                    assert_eq!(i, received);
                }

                assert_eq!(rx.next().await, None);
            })
            .await;
    }

    #[test]
    async fn mpsc_drops_sender() {
        let (tx, mut rx) = unbounded::<usize>();
        drop(tx);

        assert_eq!(rx.next().await, None);
    }
}
