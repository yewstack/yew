use std::time::Duration;

use futures::stream::LocalBoxStream;

use super::panic_no_runtime;

pub(crate) async fn sleep(_dur: Duration) {
    panic_no_runtime();
}

pub(crate) fn interval(_dur: Duration) -> LocalBoxStream<'static, ()> {
    panic_no_runtime();
}
