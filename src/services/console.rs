//! This module contains a service implementation to use browser's console.

/// A service to use methods of a
/// [Console](https://developer.mozilla.org/en-US/docs/Web/API/Console).
pub struct ConsoleService;

impl ConsoleService {
    /// [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
    /// method implementation.
    pub fn log(&self, message: &str) { js! { @(no_return) console.log(@{message}); } }

    /// [console.warn](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
    /// method implementation.
    pub fn warn(&self, message: &str) { js! { @(no_return) console.warn(@{message}); } }

    /// [console.info](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
    /// method implementation.
    pub fn info(&self, message: &str) { js! { @(no_return) console.info(@{message}); } }

    /// [console.error](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
    /// method implementation.
    pub fn error(&self, message: &str) { js! { @(no_return) console.error(@{message}); } }

    /// [console.debug](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
    /// method implementation.
    pub fn debug(&self, message: &str) { js! { @(no_return) console.debug(@{message}); } }

    /// [console.count_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/count_named)
    /// method implementation.
    pub fn count_named(&self, name: &str) { js! { @(no_return) console.count(@{name}); } }

    /// [console.count](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
    /// method implementation.
    pub fn count(&self) { js! { @(no_return) console.count(); } }


    /// [console.time_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named)
    /// method implementation.
    pub fn time_named(&self, name: &str) { js! { @(no_return) console.time(@{name}); } }

    /// [console.time_named_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named_end)
    /// method implementation.
    pub fn time_named_end(&self, name: &str) { js! { @(no_return) console.timeEnd(@{name}); } }


    /// [console.time](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
    /// method implementation.
    pub fn time(&self) { js! { @(no_return) console.time(); } }
    /// [console.time_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_end)
    /// method implementation.
    pub fn time_end(&self) { js! { @(no_return) console.timeEnd(); } }


    /// [console.clear](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
    /// method implementation.
    pub fn clear(&self) { js! { @(no_return) console.clear(); } }

    /// [console.group](https://developer.mozilla.org/en-US/docs/Web/API/Console/group)
    /// method implementation.
    pub fn group(&self) { js! { @(no_return) console.group(); } }

    /// [console.group_collapsed](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_collapsed)
    /// method implementation.
    pub fn group_collapsed(&self) { js! { @(no_return) console.groupCollapsed(); } }

    /// [console.group_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_end)
    /// method implementation.
    pub fn group_end(&self) { js! { @(no_return) console.groupEnd(); } }

    /// [console.trace](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
    /// method implementation.
    pub fn trace(&self) { js! { @(no_return) console.trace(); } }

    /// [console.assert](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
    /// method implementation.
    pub fn assert(&self, condition: bool, message: &str) { js! { @(no_return) console.assert(@{condition}, @{message}); } }
}
