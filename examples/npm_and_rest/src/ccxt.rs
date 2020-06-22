use js_sys::{Array, Reflect};
use web_sys::console;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

pub struct CcxtService(&'static JsValue);

#[wasm_bindgen]
extern "C" {
    static ccxt: JsValue;
}

impl Default for CcxtService {
    fn default() -> CcxtService {
        let lib: &JsValue = &ccxt;
        CcxtService(lib)
    }
}

impl CcxtService {
    pub fn exchanges(&mut self) -> Vec<String> {
        let v = {
            let exchanges = Reflect::get(&self.0, &JsValue::from_str("exchanges")).unwrap();
            console::log_1(&exchanges);
            exchanges
        };
        let v: Vec<String> = Array::from(&v)
            .to_vec()
            .into_iter()
            .map(|v| v.as_string().expect("can't extract exchanges"))
            .collect();
        v
    }
}
