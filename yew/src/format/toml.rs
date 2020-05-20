//! Contains an implementation of TOML serialization format.

/// A representation of a TOML data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a Toml
///# use yew::format::Toml;
///# fn dont_execute() {
///# let data: String = unimplemented!();
/// let dump = Toml(&data);
///
/// // Converts TOML string to a data (lazy).
/// let Toml(data) = dump;
/// }
/// ```
#[derive(Debug)]
pub struct Toml<T>(pub T);

text_format!(Toml based on toml);

binary_format!(Toml based on toml);
