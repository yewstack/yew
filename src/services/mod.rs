pub mod timeout;
pub mod interval;
pub mod storage;
pub mod alert;
pub mod console;
pub mod fetch;

use std::time::Duration;
use serde_json;
use serde::{Serialize, Deserialize};
use stdweb::unstable::{TryInto, TryFrom};

pub trait Task {
    fn cancel(&mut self);
}

pub struct Nothing;

impl Into<Option<String>> for Nothing {
    fn into(self) -> Option<String> {
        None
    }
}

impl From<String> for Nothing {
    fn from(value: String) -> Nothing {
        Nothing
    }
}

pub struct Json<T>(pub T);

impl<T> Into<Option<String>> for Json<T>
where
    T: Serialize
{
    fn into(self) -> Option<String> {
        serde_json::to_string(&self.0).ok()
    }
}

impl<T> From<String> for Json<Result<T, ()>>
where
    T: for <'de> Deserialize<'de>
{
    fn from(value: String) -> Self {
        Json(serde_json::from_str(&value).map_err(drop))
    }
}

#[doc(hidden)]
fn to_ms(duration: Duration) -> u32 {
    let ms = duration.subsec_nanos() / 1_000_000;
    ms + duration.as_secs() as u32 * 1000
}
