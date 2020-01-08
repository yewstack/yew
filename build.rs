use std::env;

pub fn main() {
    if cfg!(all(feature = "web_sys", feature = "std_web")) {
        panic!("the `web_sys` and `std_web` cargo features cannot be used simultaneously")
    } else if cfg!(not(any(feature = "web_sys", feature = "std_web"))) {
        panic!("please select either the `web_sys` or `std_web` cargo feature")
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").unwrap_or_default();
    if target_arch == "wasm32" && cargo_web != "1" {
        println!("cargo:rustc-cfg=feature=\"wasm_bindgen_test\"");
    }
}
