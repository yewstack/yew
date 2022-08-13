use std::cell::Cell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::stream;
use futures::stream::Stream;
use gloo::timers::callback::Timeout;

#[inline(always)]
pub(crate) fn sleep(dur: Duration) -> impl Future<Output = ()> {
    pub struct Sleep {
        inner: Option<Timeout>,
        dur_left: Option<u128>,
        timeout_registered: Rc<Cell<bool>>,
    }

    impl Future for Sleep {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            static I32_MAX_U128: u128 = 2_147_483_647;
            static I32_MAX_U32: u32 = 2_147_483_647;

            // If polling before the registered timeout is reached, return Pending.
            if self.timeout_registered.get() {
                return Poll::Pending;
            }

            // set_timeout can only accept maximum of i32, so we wrap around if it gets longer.
            let next_timeout = match self.dur_left.map(|m| (m, u32::try_from(m))) {
                Some((m_u128, Err(_))) => {
                    self.dur_left = Some(m_u128 - I32_MAX_U128);
                    I32_MAX_U32
                }
                Some((m_u128, _)) if m_u128 > I32_MAX_U128 => {
                    self.dur_left = Some(m_u128 - I32_MAX_U128);
                    I32_MAX_U32
                }
                Some((_, Ok(m_u32))) => {
                    self.dur_left = None;
                    m_u32
                }
                None => return Poll::Ready(()),
            };

            let waker = cx.waker().clone();
            self.timeout_registered.set(true);
            let timeout_registered = self.timeout_registered.clone();

            self.inner = Some(Timeout::new(next_timeout, move || {
                timeout_registered.set(false);
                waker.wake();
            }));

            Poll::Pending
        }
    }

    Sleep {
        inner: None,
        dur_left: Some(dur.as_millis()),
        timeout_registered: Cell::new(false).into(),
    }
}

pub(crate) fn interval(dur: Duration) -> impl Stream<Item = ()> {
    stream::unfold((), move |_: ()| async move {
        sleep(dur).await;

        Some(((), ()))
    })
}
