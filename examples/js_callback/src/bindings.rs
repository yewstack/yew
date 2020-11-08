use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/get-payload-script.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getPayload")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "getPayloadLater")]
    pub fn get_payload_later(payload_callback: JsValue);
}
