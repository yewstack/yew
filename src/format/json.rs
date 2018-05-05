//! Contains an implementation of JSON serialization format.

use failure::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use super::{Text, Binary};

/// A representation of a JSON data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Json
/// let dump = Json(&data);
///
/// // Converts JSON string to a data (lazy).
/// let Json(data) = dump;
/// ```
pub struct Json<T>(pub T);

impl<'a, T> Into<Text> for Json<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Text {
        serde_json::to_string(&self.0).map_err(Error::from)
    }
}

impl<T> From<Text> for Json<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Text) -> Self {
        match value {
            Ok(data) => Json(serde_json::from_str(&data).map_err(Error::from)),
            Err(reason) => Json(Err(reason)),
        }
    }
}

impl<'a, T> Into<Binary> for Json<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Binary {
        serde_json::to_vec(&self.0).map_err(Error::from)
    }
}

impl<T> From<Binary> for Json<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Binary) -> Self {
        match value {
            Ok(data) => Json(serde_json::from_slice(&data).map_err(Error::from)),
            Err(reason) => Json(Err(reason)),
        }
    }
}
