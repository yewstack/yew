//! Service to load files using `FileReader`.

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
        pub use std_web::*;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
        pub use self::web_sys::*;
    }
}
