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

        if let Some(m) = self.rx_waker.take() {
            m.wake();
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
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        match (inner.items.pop_front(), inner.closed) {
            (Some(m), _) => Ok(Some(m)),
            (None, false) => Ok(None),
            (None, true) => Err(TryRecvError {
                _marker: PhantomData,
            }),
        }
    }
}

impl<T> Stream for UnboundedReceiver<T> {
    type Item = T;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        match (inner.items.pop_front(), inner.closed) {
            (Some(m), _) => Poll::Ready(Some(m)),
            (None, false) => {
                inner.rx_waker = Some(cx.waker().clone());
                Poll::Pending
            }
            (None, true) => Poll::Ready(None),
        }
    }
}

impl<T> FusedStream for UnboundedReceiver<T> {
    fn is_terminated(&self) -> bool {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        inner.items.is_empty() && inner.closed
    }
}

impl<T> Drop for UnboundedReceiver<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        inner.close();
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
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        if inner.closed {
            return Err(SendError { inner: item });
        }

        inner.items.push_back(item);

        if let Some(m) = inner.rx_waker.take() {
            m.wake();
        }

        Ok(())
    }

    /// Closes the channel.
    pub fn close_now(&self) {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        inner.close();
    }
}

impl<T> Clone for UnboundedSender<T> {
    fn clone(&self) -> Self {
        {
            let inner = unsafe { &mut *self.inner.get_mut_unchecked() };
            inner.sender_ctr += 1;
        }

        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Drop for UnboundedSender<T> {
    fn drop(&mut self) {
        let sender_ctr = {
            let inner = unsafe { &mut *self.inner.get_mut_unchecked() };
            inner.sender_ctr -= 1;

            inner.sender_ctr
        };

        if sender_ctr == 0 {
            self.close_now();
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
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        match inner.closed {
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

    use super::*;
    use crate::platform::time::sleep;
    use crate::platform::{spawn_local, LocalRuntime};

    #[test]
    fn mpsc_works() {
        let rt = LocalRuntime::new().expect("failed to create runtime.");

        rt.block_on(async {
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
        });
    }

    #[test]
    fn mpsc_drops_receiver() {
        let rt = LocalRuntime::new().expect("failed to create runtime.");

        rt.block_on(async {
            let (tx, rx) = unbounded::<usize>();
            drop(rx);

            (&tx).send(0).await.expect_err("should fail to send.");
        });
    }

    #[test]
    fn mpsc_multi_sender() {
        let rt = LocalRuntime::new().expect("failed to create runtime.");

        rt.block_on(async {
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
        });
    }

    #[test]
    fn mpsc_drops_sender() {
        let rt = LocalRuntime::new().expect("failed to create runtime.");

        rt.block_on(async {
            let (tx, mut rx) = unbounded::<usize>();
            drop(tx);

            assert_eq!(rx.next().await, None);
        });
    }
}
