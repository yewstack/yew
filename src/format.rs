//! Utility module to convert data to types and back by
//! specific formats like: JSON, BSON, TOML, YAML, XML.
//!
//! All types here are lazy and it's necessary to
//! use `Into` and `From` traits to get (convert) the data.

use failure::Error;
use serde::{Serialize, Deserialize};
use serde_json;

/// A representation of a value which can be stored.
pub type Storable = Option<String>;

/// A representation of a value which can be restored.
pub type Restorable = Result<String, Error>;

/// A representation of an empty data. Nothing stored. Nothing restored.
pub struct Nothing;

impl Into<Storable> for Nothing {
    fn into(self) -> Storable {
        None
    }
}

impl From<Restorable> for Nothing {
    fn from(_: Restorable) -> Nothing {
        Nothing
    }
}

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

impl<'a, T> Into<Storable> for Json<&'a T>
where
    T: Serialize
{
    fn into(self) -> Storable {
        serde_json::to_string(&self.0).ok()
    }
}

impl<T> From<Restorable> for Json<Result<T, Error>>
where
    T: for <'de> Deserialize<'de>
{
    fn from(value: Restorable) -> Self {
        match value {
            Ok(data) => {
                Json(serde_json::from_str(&data).map_err(Error::from))
            }
            Err(reason) => {
                Json(Err(reason))
            }
        }
    }
}

