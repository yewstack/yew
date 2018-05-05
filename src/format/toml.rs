//! Contains an implementation of TOML serialization format.

use failure::Error;
use serde::{Deserialize, Serialize};
use toml as format;
use super::{Text, Binary};

/// A representation of a TOML data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Toml
/// let dump = Toml(&data);
///
/// // Converts TOML string to a data (lazy).
/// let Toml(data) = dump;
/// ```
pub struct Toml<T>(pub T);

impl<'a, T> Into<Text> for Toml<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Text {
        format::to_string(&self.0).map_err(Error::from)
    }
}

impl<T> From<Text> for Toml<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Text) -> Self {
        match value {
            Ok(data) => Toml(format::from_str(&data).map_err(Error::from)),
            Err(reason) => Toml(Err(reason)),
        }
    }
}

impl<'a, T> Into<Binary> for Toml<&'a T>
where
    T: Serialize,
{
    fn into(self) -> Binary {
        format::to_vec(&self.0).map_err(Error::from)
    }
}

impl<T> From<Binary> for Toml<Result<T, Error>>
where
    T: for<'de> Deserialize<'de>,
{
    fn from(value: Binary) -> Self {
        match value {
            Ok(data) => Toml(format::from_slice(&data).map_err(Error::from)),
            Err(reason) => Toml(Err(reason)),
        }
    }
}
