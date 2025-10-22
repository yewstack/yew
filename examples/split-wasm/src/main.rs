use std::cell::Cell;

use wasm_bindgen::prelude::wasm_bindgen;

// You can use a global variable from javascript, or a static
// and even thread local variable without any changes.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = "globalFoo")]
    static GLOBAL_FOO: u32;
}
thread_local! {
    static COUNTER: Cell<usize> = const { Cell::new(0) };
}

mod yew;

pub fn main() {
    yew::main();
}
