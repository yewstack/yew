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
/// // Or another way to convert JSON string to data:
/// let data = dump.into_inner();
/// ```
#[derive(Debug)]
pub struct Json<T>(pub T);

text_format!(Json based on serde_json);

binary_format!(Json based on serde_json);

impl<T> Json<T> {
    /// Consumes the JSON wrapper and returns the wrapped item.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

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

    #[test]
    fn test_into_inner() {
        #[derive(Serialize, Deserialize)]
        struct Data {
            value: u8,
        }

        let data: Json<Result<Data, _>> = Json::from(Ok(r#"{"value": 123}"#.to_string()));
        let data = data.into_inner().unwrap();
        assert_eq!(data.value, 123);
    }
}
