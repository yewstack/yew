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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{Binary, Text};
    use serde::{Deserialize, Serialize};
    #[cfg(feature = "wasm_test")]
    use wasm_bindgen_test::wasm_bindgen_test as test;

    #[test]
    fn json_format() {
        #[derive(Serialize, Deserialize)]
        struct Data {
            value: u8,
        }

        let Json(data): Json<Result<Data, _>> = Json::from(Ok(r#"{"value": 123}"#.to_string()));
        let data = data.unwrap();
        assert_eq!(data.value, 123);

        let _stored: Text = Json(&data).into();
        let _stored: Binary = Json(&data).into();
    }
}
