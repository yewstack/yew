//! Contains an implementation of JSON serialization format.

/// A representation of a JSON data. Use it as wrapper to
/// set a format you want to use for conversion:
///
/// ```
/// // Converts (lazy) data to a Json
/// use yew::format::Json;
/// let data: String = r#"{lorem: "ipsum"}"#.to_string();
/// let dump = Json(&data);
///
/// // Converts JSON string to a data (lazy).
/// let Json(data) = dump;
/// ```
#[derive(Debug)]
pub struct Json<T>(pub T);

text_format!(Json based on serde_json);

binary_format!(Json based on serde_json);
