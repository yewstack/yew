#[cfg(feature = "std_web")]
use stdweb::{unstable::TryInto, Value};
#[cfg(feature = "web_sys")]
use ::{
    js_sys::{Array, Reflect},
    wasm_bindgen::{prelude::wasm_bindgen, JsValue},
    web_sys::console,
};

#[derive(Default)]
pub struct CcxtService(
    #[cfg(feature = "std_web")] Option<Value>,
    #[cfg(feature = "web_sys")] Option<&'static JsValue>,
);

#[cfg(feature = "web_sys")]
#[wasm_bindgen]
extern "C" {
    static ccxt: JsValue;
}

impl CcxtService {
    pub fn new() -> Self {
        #[cfg(feature = "std_web")]
        let lib = js! {
            return ccxt;
        };
        #[cfg(feature = "web_sys")]
        let lib: &JsValue = &ccxt;
        CcxtService(Some(lib))
    }

    pub fn exchanges(&mut self) -> Vec<String> {
        let lib = self.0.as_ref().expect("ccxt library object lost");
        #[cfg(feature = "std_web")]
        let v: Value = js! {
            var ccxt = @{lib};
            console.log(ccxt.exchanges);
            return ccxt.exchanges;
        };
        #[cfg(feature = "web_sys")]
        let v = {
            let exchanges = Reflect::get(lib, &JsValue::from_str("exchanges")).unwrap();
            console::log_1(&exchanges);
            exchanges
        };
        #[cfg(feature = "std_web")]
        let v: Vec<String> = v.try_into().expect("can't extract exchanges");
        #[cfg(feature = "web_sys")]
        let v: Vec<String> = Array::from(&v)
            .to_vec()
            .into_iter()
            .map(|v| v.as_string().expect("can't extract exchanges"))
            .collect();
        v
    }
}
