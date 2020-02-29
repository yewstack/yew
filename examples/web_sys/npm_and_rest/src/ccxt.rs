use js_sys::{Array, Reflect};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::console;

#[derive(Default)]
pub struct CcxtService(Option<&'static JsValue>);

#[wasm_bindgen]
extern "C" {
    static ccxt: JsValue;
}

impl CcxtService {
    pub fn new() -> Self {
        let lib: &JsValue = &ccxt;
        CcxtService(Some(lib))
    }

    pub fn exchanges(&mut self) -> Vec<String> {
        let lib = self.0.as_ref().expect("ccxt library object lost");
        let v = {
            let exchanges = Reflect::get(lib, &JsValue::from_str("exchanges")).unwrap();
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
