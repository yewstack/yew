use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm-bindgen-test")]
use wasm_bindgen_test::wasm_bindgen_test as test;
use yew::format::{Binary, Json, Text};

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
