//! Contains an implementation of Bincode serialization format.

/// A representation of a Bincode data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Bincode
///# use yew::format::Bincode;
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = Bincode(&data);
///
/// // Converts Bincode to a data (lazy).
/// let Bincode(data) = dump;
///# }
/// ```
/// This is a binary only format.
#[derive(Debug)]
pub struct Bincode<T>(pub T);

binary_format!(Bincode, bincode::serialize, bincode::deserialize);
text_format_is_an_error!(Bincode);
