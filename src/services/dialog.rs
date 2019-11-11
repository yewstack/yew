//! This module contains the implementation of a service
//! to show alerts and confirm dialogs in a browser.

use stdweb::Value;
#[allow(unused_imports)]
use stdweb::{_js_impl, js};

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
        js! { @(no_return) alert(@{message}); }
    }

    /// Calls [confirm](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    /// function.
    pub fn confirm(&mut self, message: &str) -> bool {
        let value: Value = js! { return confirm(@{message}); };
        match value {
            Value::Bool(result) => result,
            _ => false,
        }
    }
}
