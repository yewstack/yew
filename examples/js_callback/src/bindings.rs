use wasm_bindgen::prelude::*;

// https://github.com/rustwasm/wasm-bindgen/issues/3208
#[wasm_bindgen(inline_js = "export function import2(path) { return import(path); }")]
extern "C" {
    // this should be in js-sys but is not. see https://github.com/rustwasm/wasm-bindgen/issues/2865
    // pub fn import(s: &str) -> js_sys::Promise;
    #[wasm_bindgen(js_name = "import2")]
    pub fn import(name: &str) -> js_sys::Promise;

    pub type Window;

    #[wasm_bindgen(method, getter, js_name = "wasmBindgenSnippetsPath")]
    pub fn wasm_bindgen_snippets_path(this: &Window) -> String;
}

#[wasm_bindgen(module = "/js/imp.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn hello() -> String;
}

#[wasm_bindgen]
extern "C" {
    pub type UnimpModule;

    #[wasm_bindgen(method)]
    pub fn bye(this: &UnimpModule) -> String;
}

#[wasm_bindgen(module = "/js/unimp.js")]
extern "C" {
    /// This exists so that wasm bindgen copies js/unimp.js to
    /// dist/snippets/<bin-name>-<hash>/js/uninp.js
    #[wasm_bindgen]
    fn _dummy_fn_so_wasm_bindgen_copies_over_the_file();
}
