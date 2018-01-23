#[macro_use]
extern crate serde_derive;
extern crate yew;

use yew::format::{Json, Storable};

#[test]
fn json_format() {
    #[derive(Serialize, Deserialize)]
    struct Data {
        value: u8,
    }

    let Json(data): Json<Result<Data, _>> = Json::from(Ok(r#"{"value": 123}"#.into()));
    let data = data.unwrap();
    assert_eq!(data.value, 123);

    let _stored: Storable = Json(&data).into();
}
