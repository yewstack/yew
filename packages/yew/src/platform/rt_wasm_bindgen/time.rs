use std::future::Future;
use std::time::Duration;

use futures::stream::Stream;

pub(crate) fn sleep(dur: Duration) -> impl Future<Output = ()> {
    gloo::timers::future::sleep(dur)
}

pub(crate) fn interval(dur: Duration) -> impl Stream<Item = ()> {
    let millis = u32::try_from(dur.as_millis())
        .expect_throw("failed to cast the duration into a u32 with Duration::as_millis.");

    gloo::timers::future::IntervalStream::new(millis)
}
