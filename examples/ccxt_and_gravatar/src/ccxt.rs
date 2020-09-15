use js_sys::{Array, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    static ccxt: JsValue;
}

fn raw_exchanges() -> Array {
    let v = Reflect::get(&ccxt, &JsValue::from_str("exchanges")).unwrap();
    Array::from(&v)
}

pub fn iter_exchanges() -> impl Iterator<Item = String> + 'static {
    raw_exchanges()
        .to_vec()
        .into_iter()
        .filter_map(|v| v.as_string())
}
