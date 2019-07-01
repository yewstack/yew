use std::env;

pub fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").unwrap_or_default();
    if target_arch == "wasm32" && cargo_web != "1" {
        println!("cargo:rustc-cfg=feature=\"wasm_bindgen_test\"");
    }
}
