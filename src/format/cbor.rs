//! Contains an implementation of CBOR serialization format.

use serde_cbor;

/// A representation of a CBOR data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Cbor
/// let dump = Cbor(&data);
///
/// // Converts CBOR string to a data (lazy).
/// let Cbor(data) = dump;
/// ```
#[derive(Debug)]
pub struct Cbor<T>(pub T);

binary_format!(Cbor based on serde_cbor);
