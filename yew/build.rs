use std::env;

pub fn main() {
    let using_web_sys = cfg!(feature = "web_sys");
    let using_std_web = cfg!(feature = "std_web");
    let using_ssr = cfg!(feature = "static_render");

    match (using_web_sys, using_std_web, using_ssr) {
        // Valid, using web_sys}
        (true, false, false) => {
            let using_cargo_web = env::var("COMPILING_UNDER_CARGO_WEB").is_ok();
            if using_cargo_web {
                panic!("cargo-web is not compatible with web-sys");
            }
        }

        // Valid, using std_web
        (false, true, false) => {}

        // Valid, using ssr
        (false, false, true) => {}

        // Invalid, using all?
        (true, true, true) | (true, true, false) | (true, false, true) | (false, true, true) => {
            panic!("Plase choose one of `web_sys`, `std_web`, or `static-render` cargo features");
        }

        (false, false, false) => {
            panic!("Yew requires selecting either the `web_sys`, `std_web`, or `static_render` cargo feature");
        }
    }
}
