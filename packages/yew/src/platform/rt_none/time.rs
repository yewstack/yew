use std::time::Duration;

use futures::stream::LocalBoxStream;

use super::NO_RUNTIME_NOTICE;

pub(crate) async fn sleep(_dur: Duration) {
    panic!("{}", NO_RUNTIME_NOTICE);
}

pub(crate) fn interval(_dur: Duration) -> LocalBoxStream<'static, ()> {
    panic!("{}", NO_RUNTIME_NOTICE);
}
