//! Contains an implementation of CBOR serialization format.

/// A representation of a CBOR data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a Cbor
///# use yew::format::Cbor;
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = Cbor(&data);
///
/// // Converts CBOR string to a data (lazy).
/// let Cbor(data) = dump;
///# }
/// ```
/// This is a binary only format.
#[derive(Debug)]
pub struct Cbor<T>(pub T);

binary_format!(Cbor based on serde_cbor);
text_format_is_an_error!(Cbor);
