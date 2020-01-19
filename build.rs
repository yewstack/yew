use std::env;

pub fn main() {
    if cfg!(all(feature = "web_sys", feature = "std_web")) {
        panic!("Yew does not allow the `web_sys` and `std_web` cargo features to be used simultaneously");
    } else if cfg!(not(any(feature = "web_sys", feature = "std_web"))) {
        panic!("Yew requires selecting either the `web_sys` or `std_web` cargo feature");
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let using_wasi = target_os == "wasi";

    let cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").unwrap_or_default();
    let using_cargo_web = cargo_web == "1";
    if using_cargo_web && cfg!(feature = "web_sys") {
        panic!("cargo-web is not compatible with web-sys");
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let using_wasm_bindgen = target_arch == "wasm32" && !using_cargo_web && !using_wasi;
    if !using_wasm_bindgen && cfg!(all(feature = "web_sys", not(feature = "doc_test"))) {
        let target = env::var("TARGET").unwrap_or_default();
        panic!(
            "Selected target `{}` is not compatible with web-sys",
            target
        );
    }
}
