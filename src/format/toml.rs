//! Contains an implementation of TOML serialization format.

use toml;

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

impl_format!(Toml based on toml);
