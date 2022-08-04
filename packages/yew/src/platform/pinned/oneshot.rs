//! A one-time send - receive channel.

use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{Poll, Waker};

use thiserror::Error;

/// Error returned by [`send`](Sender::send).
#[derive(Debug, Error)]
#[error("channel has been closed.")]
pub struct SendError<T> {
    /// The inner value.
    pub inner: T,
}

/// Error returned by awaiting the [`Receiver`].
#[derive(Debug, Error)]
#[error("channel has been closed.")]
pub struct RecvError {
    _marker: PhantomData<()>,
}

#[derive(Debug)]
struct Inner<T> {
    rx_waker: Option<Waker>,
    closed: bool,
    item: Option<T>,
}

impl<T> Inner<T> {
    /// Creates a unchecked mutable reference from a mutable reference.
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
}

/// The receiver of a oneshot channel.
#[derive(Debug)]
pub struct Receiver<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        // Implementation Note:
        //
        // It might be neater to use a match pattern here.
        // However, this will slow down the polling process by 10%.

        if let Some(m) = inner.item.take() {
            return Poll::Ready(Ok(m));
        }

        if inner.closed {
            return Poll::Ready(Err(RecvError {
                _marker: PhantomData,
            }));
        }

        inner.rx_waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        inner.closed = true;
    }
}

/// The sender of a oneshot channel.
#[derive(Debug)]
pub struct Sender<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Sender<T> {
    /// Send an item to the other side of the channel, consumes the sender.
    pub fn send(self, item: T) -> Result<(), T> {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        if inner.closed {
            return Err(item);
        }

        inner.item = Some(item);

        if let Some(ref m) = inner.rx_waker {
            m.wake_by_ref();
        }

        Ok(())
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let inner = unsafe { &mut *self.inner.get_mut_unchecked() };

        inner.closed = true;

        if inner.item.is_none() {
            if let Some(ref m) = inner.rx_waker {
                m.wake_by_ref();
            }
        }
    }
}

/// Creates a oneshot channel.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(Inner {
        rx_waker: None,
        closed: false,
        item: None,
    });

    (
        Sender {
            inner: inner.clone(),
        },
        Receiver { inner },
    )
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "tokio")]
#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use tokio::sync::Barrier;
    use tokio::test;

    use super::*;
    use crate::platform::spawn_local;
    use crate::platform::time::sleep;

    #[test]
    async fn oneshot_works() {
        let (tx, rx) = channel();

        tx.send(0).expect("failed to send.");

        assert_eq!(rx.await.expect("failed to receive."), 0);
    }

    #[test]
    async fn oneshot_drops_sender() {
        let (tx, rx) = channel::<usize>();

        spawn_local(async move {
            sleep(Duration::from_millis(1)).await;

            drop(tx);
        });
        rx.await.expect_err("successful to receive.");
    }

    #[test]
    async fn oneshot_drops_receiver() {
        let (tx, rx) = channel::<usize>();

        let bar = Arc::new(Barrier::new(2));

        {
            let bar = bar.clone();
            spawn_local(async move {
                sleep(Duration::from_millis(1)).await;

                drop(rx);

                bar.wait().await;
            });
        }

        bar.wait().await;

        tx.send(0).expect_err("successful to send.");
    }
}
