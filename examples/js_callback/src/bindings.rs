use wasm_bindgen::prelude::*;

// this should be in js-sys but is not. see https://github.com/rustwasm/wasm-bindgen/issues/2865
#[wasm_bindgen]
extern "C" {
    pub fn import(s: &str) -> js_sys::Promise;
}

#[wasm_bindgen(module = "/js/imp.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn hello() -> String;
}

#[wasm_bindgen(module = "/js/unimp.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn bye() -> String;
}
