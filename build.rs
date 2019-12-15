use std::env;

pub fn main() {
    if cfg!(all(feature = "web_sys", feature = "stdweb")) {
        panic!("don't use `web_sys` and `stdweb` simultaneously")
    }
    else if cfg!(not(any(feature = "web_sys", feature = "stdweb"))) {
        panic!("please select either `web_sys` or `stdweb`")
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").unwrap_or_default();
    if target_arch == "wasm32" && cargo_web != "1" {
        println!("cargo:rustc-cfg=feature=\"wasm_bindgen_test\"");
    }
}
