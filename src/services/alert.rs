//! This module contains the implementation of a service
//! to show alerts in a browser.

use html::Context;

/// An abstract alert service.
pub trait AlertService {
    /// Calls [alert](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    /// function.
    fn alert(&mut self, message: &str);
}

impl<MSG: 'static> AlertService for Context<MSG> {
    fn alert(&mut self, message: &str) {
        js! { alert(@{message}); }
    }
}
