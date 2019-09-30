//! Contains an implementation of MessagePack serialization format.

use rmp_serde;

/// A representation of a MessagePack data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a MsgPack
///
///# use yew::format::MsgPack;
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = MsgPack(&data);
///
/// // Converts MessagePack string to a data (lazy).
/// let MsgPack(data) = dump;
///# }
/// ```
pub struct MsgPack<T>(pub T);

binary_format!(MsgPack based on rmp_serde);
