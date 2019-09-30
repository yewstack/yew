//! Contains an implementation of MessagePack serialization format.

use rmp_serde;

/// A representation of a MessagePack data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a MsgPack
/// let dump = MsgPack(&data);
///
/// // Converts MessagePack string to a data (lazy).
/// let MsgPack(data) = dump;
/// ```
#[derive(Debug)]
pub struct MsgPack<T>(pub T);

binary_format!(MsgPack based on rmp_serde);
