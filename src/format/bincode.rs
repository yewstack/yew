//! Contains an implementation of CBOR serialization format.

use bincode;

/// A representation of a CBOR data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Cbor
/// let dump = Bincode(&data);
///
/// // Converts CBOR string to a data (lazy).
/// let Bincode(data) = dump;
/// ```
pub struct Bincode<T>(pub T);

impl<T: serde::Serialize> Into<super::Binary> for Bincode<T> {
    fn into(self) -> super::Binary {
        bincode::serialize(&self.0).map_err(failure::Error::from)
    }
}

impl<T> From<super::Binary> for Bincode<Result<T, failure::Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    fn from(value: super::Binary) -> Self {
        match value {
            Ok(data) => Bincode(bincode::deserialize(&data).map_err(failure::Error::from)),
            Err(reason) => Bincode(Err(reason)),
        }
    }
}

unimplemented_text_format!(Bincode based on bincode);
