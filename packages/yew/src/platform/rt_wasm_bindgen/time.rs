use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use futures::stream;
use futures::stream::Stream;
use gloo::timers::callback::Timeout;
use wasm_bindgen::UnwrapThrowExt;

#[inline(always)]
pub(crate) fn sleep(dur: Duration) -> impl Future<Output = ()> {
    pub struct Sleep {
        inner: Option<Timeout>,
        dur_millis: Option<u128>,
    }

    impl Future for Sleep {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // set_timeout can only accept maximum of i32, so we wrap around if it gets longer.
            static I32_MAX_U128: u128 = 2_147_483_647;
            static I32_MAX_U32: u32 = 2_147_483_647;

            match self.dur_millis.as_mut() {
                Some(m) if *m > I32_MAX_U128 => {
                    *m -= I32_MAX_U128;
                    let waker = cx.waker().clone();
                    self.inner = Some(Timeout::new(I32_MAX_U32, move || {
                        waker.wake();
                    }));
                    Poll::Pending
                }
                Some(m) => {
                    let waker = cx.waker().clone();
                    self.inner = Some(Timeout::new((*m).try_into().unwrap_throw(), move || {
                        waker.wake();
                    }));
                    self.dur_millis = None;
                    Poll::Pending
                }
                None => Poll::Ready(()),
            }
        }
    }

    Sleep {
        inner: None,
        dur_millis: Some(dur.as_millis()),
    }
}

pub(crate) fn interval(dur: Duration) -> impl Stream<Item = ()> {
    stream::unfold((), move |_: ()| async move {
        sleep(dur).await;

        Some(((), ()))
    })
}
