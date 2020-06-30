//! This module contains a service implementation to use browser's console.

use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use wasm_bindgen::JsValue;
        use web_sys::console;
    }
}

/// A service to use methods of a
/// [Console](https://developer.mozilla.org/en-US/docs/Web/API/Console).
#[derive(Default, Debug)]
pub struct ConsoleService {}

impl ConsoleService {
    /// [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
    /// method implementation.
    /// This method outputs the provided message to the console.
    pub fn log(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.log(@{message}); },
            feature = "web_sys" => console::log_1(&JsValue::from_str(message)),
        };
    }

    /// [console.warn](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
    /// method implementation.
    /// This method outputs the provided message to the console as a warning.
    pub fn warn(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.warn(@{message}); },
            feature = "web_sys" => console::warn_1(&JsValue::from_str(message)),
        };
    }

    /// [console.info](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
    /// method implementation.
    /// This method outputs the provided message to the console as information.
    pub fn info(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.info(@{message}); },
            feature = "web_sys" => console::info_1(&JsValue::from_str(message)),
        };
    }

    /// [console.error](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
    /// method implementation.
    /// This method outputs the provided message to the console as an error.
    pub fn error(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.error(@{message}); },
            feature = "web_sys" => console::error_1(&JsValue::from_str(message)),
        };
    }

    /// [console.debug](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
    /// method implementation.
    pub fn debug(message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.debug(@{message}); },
            feature = "web_sys" => console::debug_1(&JsValue::from_str(message)),
        };
    }

    /// [console.count_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/count_named)
    /// method implementation.
    pub fn count_named(name: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.count(@{name}); },
            feature = "web_sys" => console::count_with_label(name),
        };
    }

    /// [console.count](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
    /// method implementation.
    pub fn count() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.count(); },
            feature = "web_sys" => console::count(),
        };
    }

    /// [console.time_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named)
    /// method implementation.
    pub fn time_named(name: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.time(@{name}); },
            feature = "web_sys" => console::time_with_label(name),
        };
    }

    /// [console.time_named_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named_end)
    /// method implementation.
    pub fn time_named_end(name: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.timeEnd(@{name}); },
            feature = "web_sys" => console::time_end_with_label(name),
        };
    }

    /// [console.time](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
    /// method implementation.
    pub fn time() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.time(); },
            feature = "web_sys" => console::time(),
        };
    }
    /// [console.time_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_end)
    /// method implementation.
    pub fn time_end() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.timeEnd(); },
            feature = "web_sys" => console::time_end(),
        };
    }

    /// [console.clear](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
    /// method implementation.
    pub fn clear() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.clear(); },
            feature = "web_sys" => console::clear(),
        };
    }

    /// [console.group](https://developer.mozilla.org/en-US/docs/Web/API/Console/group)
    /// method implementation.
    pub fn group() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.group(); },
            feature = "web_sys" => console::group_0(),
        };
    }

    /// [console.group_collapsed](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_collapsed)
    /// method implementation.
    pub fn group_collapsed() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.groupCollapsed(); },
            feature = "web_sys" => console::group_collapsed_0(),
        };
    }

    /// [console.group_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_end)
    /// method implementation.
    pub fn group_end() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.groupEnd(); },
            feature = "web_sys" => console::group_end(),
        };
    }

    /// [console.trace](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
    /// method implementation.
    /// This method outputs the current stack trace to the console.
    pub fn trace() {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.trace(); },
            feature = "web_sys" => console::trace_0(),
        };
    }

    /// [console.assert](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
    /// method implementation.
    pub fn assert(condition: bool, message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) console.assert(@{condition}, @{message}); },
            feature = "web_sys" => console::assert_with_condition_and_data_1(condition, &String::from(message).into()),
        };
    }
}
