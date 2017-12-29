use serde::{Serialize, Deserialize};
use serde_json;

pub struct Nothing;

impl Into<Option<String>> for Nothing {
    fn into(self) -> Option<String> {
        None
    }
}

impl From<String> for Nothing {
    fn from(_: String) -> Nothing {
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

