pub mod timeout;
pub mod interval;
pub mod storage;
pub mod alert;

use std::time::Duration;

pub trait Task {
    fn cancel(&mut self);
}

fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_nanos() / 1_000_000;
    ms + duration.as_secs() as u32 * 1000
}


