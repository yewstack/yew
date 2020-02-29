//! This module contains the implementation of a service
//! to show alerts and confirm dialogs in a browser.

use cfg_if::cfg_if;
use cfg_match::cfg_match;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use crate::utils;
    }
}

/// A dialog service.
#[derive(Default, Debug)]
pub struct DialogService {}

impl DialogService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// Calls [alert](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    /// function.
    pub fn alert(&mut self, message: &str) {
        cfg_match! {
            feature = "std_web" => js! { @(no_return) alert(@{message}); },
            feature = "web_sys" => utils::window().alert_with_message(message).unwrap(),
        };
    }

    /// Calls [confirm](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    /// function.
    pub fn confirm(&mut self, message: &str) -> bool {
        cfg_match! {
            feature = "std_web" => ({
                let value: Value = js! { return confirm(@{message}); };
                match value {
                    Value::Bool(result) => result,
                    _ => false,
                }
            }),
            feature = "web_sys" => utils::window().confirm_with_message(message).unwrap(),
        }
    }
}
