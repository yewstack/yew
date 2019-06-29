//! Contains an implementation of JSON serialization format.

/// A representation of a JSON data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```rust
/// // Converts (lazy) data to a Json
/// let dump = Json(&data);
///
/// // Converts JSON string to a data (lazy).
/// let Json(data) = dump;
/// ```
pub struct Json<T>(pub T);

text_format!(Json based on serde_json);

binary_format!(Json based on serde_json);
