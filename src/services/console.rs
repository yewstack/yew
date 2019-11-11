//! This module contains a service implementation to use browser's console.

#[allow(unused_imports)]
use stdweb::{_js_impl, js};

/// A service to use methods of a
/// [Console](https://developer.mozilla.org/en-US/docs/Web/API/Console).
#[derive(Default, Debug)]
pub struct ConsoleService {}

impl ConsoleService {
    /// Creates a new service instance connected to `App` by provided `sender`.
    pub fn new() -> Self {
        Self {}
    }

    /// [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
    /// method implementation.
    pub fn log(&mut self, message: &str) {
        js! { @(no_return) console.log(@{message}); }
    }

    /// [console.warn](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
    /// method implementation.
    pub fn warn(&mut self, message: &str) {
        js! { @(no_return) console.warn(@{message}); }
    }

    /// [console.info](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
    /// method implementation.
    pub fn info(&mut self, message: &str) {
        js! { @(no_return) console.info(@{message}); }
    }

    /// [console.error](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
    /// method implementation.
    pub fn error(&mut self, message: &str) {
        js! { @(no_return) console.error(@{message}); }
    }

    /// [console.debug](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
    /// method implementation.
    pub fn debug(&mut self, message: &str) {
        js! { @(no_return) console.debug(@{message}); }
    }

    /// [console.count_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/count_named)
    /// method implementation.
    pub fn count_named(&mut self, name: &str) {
        js! { @(no_return) console.count(@{name}); }
    }

    /// [console.count](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
    /// method implementation.
    pub fn count(&mut self) {
        js! { @(no_return) console.count(); }
    }

    /// [console.time_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named)
    /// method implementation.
    pub fn time_named(&mut self, name: &str) {
        js! { @(no_return) console.time(@{name}); }
    }

    /// [console.time_named_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named_end)
    /// method implementation.
    pub fn time_named_end(&mut self, name: &str) {
        js! { @(no_return) console.timeEnd(@{name}); }
    }

    /// [console.time](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
    /// method implementation.
    pub fn time(&mut self) {
        js! { @(no_return) console.time(); }
    }
    /// [console.time_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_end)
    /// method implementation.
    pub fn time_end(&mut self) {
        js! { @(no_return) console.timeEnd(); }
    }

    /// [console.clear](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
    /// method implementation.
    pub fn clear(&mut self) {
        js! { @(no_return) console.clear(); }
    }

    /// [console.group](https://developer.mozilla.org/en-US/docs/Web/API/Console/group)
    /// method implementation.
    pub fn group(&mut self) {
        js! { @(no_return) console.group(); }
    }

    /// [console.group_collapsed](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_collapsed)
    /// method implementation.
    pub fn group_collapsed(&mut self) {
        js! { @(no_return) console.groupCollapsed(); }
    }

    /// [console.group_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_end)
    /// method implementation.
    pub fn group_end(&mut self) {
        js! { @(no_return) console.groupEnd(); }
    }

    /// [console.trace](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
    /// method implementation.
    pub fn trace(&mut self) {
        js! { @(no_return) console.trace(); }
    }

    /// [console.assert](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
    /// method implementation.
    pub fn assert(&mut self, condition: bool, message: &str) {
        js! { @(no_return) console.assert(@{condition}, @{message}); }
    }
}
