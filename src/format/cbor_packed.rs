//! Contains an implementation of CBOR serialization format that uses a
//! packed representation.

use serde_cbor;

/// A packed representation of a CBOR data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a Cbor
///# use yew::format::CborPacked;
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = CborPacked(&data);
///
/// // Converts CBOR string to a data (lazy).
/// let CborPacked(data) = dump;
///# }
/// ```
/// This is a binary only format.
#[derive(Debug)]
pub struct CborPacked<T>(pub T);

binary_format!(
    CborPacked,
    serde_cbor::ser::to_vec_packed,
    serde_cbor::from_slice
);
text_format_is_an_error!(CborPacked);
