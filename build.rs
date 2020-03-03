use std::env;

pub fn main() {
    let using_web_sys = cfg!(feature = "web_sys");
    let using_std_web = cfg!(feature = "std_web");
    if using_web_sys && using_std_web {
        panic!("Yew does not allow the `web_sys` and `std_web` cargo features to be used simultaneously");
    } else if !using_web_sys && !using_std_web {
        panic!("Yew requires selecting either the `web_sys` or `std_web` cargo feature");
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let using_wasi = target_os == "wasi";

    let using_cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").is_ok();
    if using_cargo_web && using_web_sys {
        panic!("cargo-web is not compatible with web-sys");
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let using_wasm_bindgen = target_arch == "wasm32" && !using_cargo_web && !using_wasi;

    let using_clippy = env::var("CLIPPY_ARGS").is_ok();
    let running_doc_tests = cfg!(feature = "doc_test");

    if !using_wasm_bindgen && using_web_sys && !running_doc_tests && !using_clippy {
        let target = env::var("TARGET").unwrap_or_default();
        panic!(
            "Selected target `{}` is not compatible with web-sys",
            target
        );
    }
}
