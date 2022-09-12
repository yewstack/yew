//! A one-time send - receive channel.

use std::cell::UnsafeCell;
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{Poll, Waker};

use thiserror::Error;

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

    // This type is not send or sync.
    _marker: PhantomData<Rc<()>>,
}

/// The receiver of a oneshot channel.
#[derive(Debug)]
pub struct Receiver<T> {
    inner: Rc<UnsafeCell<Inner<T>>>,
}

impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

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
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };
        inner.closed = true;
    }
}

/// The sender of a oneshot channel.
#[derive(Debug)]
pub struct Sender<T> {
    inner: Rc<UnsafeCell<Inner<T>>>,
}

impl<T> Sender<T> {
    /// Send an item to the other side of the channel, consumes the sender.
    pub fn send(self, item: T) -> Result<(), T> {
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

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
        // SAFETY:
        //
        // We can acquire a mutable reference without checking as:
        //
        // - This type is !Sync and !Send.
        // - This function is not used by any other functions and hence uniquely owns the
        // mutable reference.
        // - The mutable reference is dropped at the end of this function.
        let inner = unsafe { &mut *self.inner.get() };

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
    let inner = Rc::new(UnsafeCell::new(Inner {
        rx_waker: None,
        closed: false,
        item: None,

        _marker: PhantomData,
    }));

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
    use tokio::task::LocalSet;
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
        let local_set = LocalSet::new();

        local_set
            .run_until(async {
                let (tx, rx) = channel::<usize>();

                spawn_local(async move {
                    sleep(Duration::from_millis(1)).await;

                    drop(tx);
                });
                rx.await.expect_err("successful to receive.");
            })
            .await;
    }

    #[test]
    async fn oneshot_drops_receiver() {
        let local_set = LocalSet::new();

        local_set
            .run_until(async {
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
            })
            .await;
    }
}
