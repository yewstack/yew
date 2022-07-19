use std::future::Future;
use std::time::Duration;

use futures::stream::{Stream, StreamExt};
use tokio_stream::wrappers::IntervalStream;

#[inline(always)]
pub(crate) fn sleep(dur: Duration) -> impl Future<Output = ()> {
    tokio::time::sleep(dur)
}

pub(crate) fn interval(dur: Duration) -> impl Stream<Item = ()> {
    IntervalStream::new(tokio::time::interval(dur)).then(|_| async {})
}
