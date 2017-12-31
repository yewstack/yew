//! This module contains a service implementation to use browser's console.

use html::Context;

/// A service to use methods of a
/// [Console](https://developer.mozilla.org/en-US/docs/Web/API/Console).
pub struct Console;

impl Console {
    /// [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)
    /// method implementation.
    pub fn log(&self, message: &str) { js! { console.log(@{message}); } }

    /// [console.warn](https://developer.mozilla.org/en-US/docs/Web/API/Console/warn)
    /// method implementation.
    pub fn warn(&self, message: &str) { js! { console.warn(@{message}); } }

    /// [console.info](https://developer.mozilla.org/en-US/docs/Web/API/Console/info)
    /// method implementation.
    pub fn info(&self, message: &str) { js! { console.info(@{message}); } }

    /// [console.error](https://developer.mozilla.org/en-US/docs/Web/API/Console/error)
    /// method implementation.
    pub fn error(&self, message: &str) { js! { console.error(@{message}); } }

    /// [console.debug](https://developer.mozilla.org/en-US/docs/Web/API/Console/debug)
    /// method implementation.
    pub fn debug(&self, message: &str) { js! { console.debug(@{message}); } }

    /// [console.count_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/count_named)
    /// method implementation.
    pub fn count_named(&self, name: &str) { js! { console.count(@{name}); } }

    /// [console.count](https://developer.mozilla.org/en-US/docs/Web/API/Console/count)
    /// method implementation.
    pub fn count(&self) { js! { console.count(); } }


    /// [console.time_named](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named)
    /// method implementation.
    pub fn time_named(&self, name: &str) { js! { console.time(@{name}); } }

    /// [console.time_named_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_named_end)
    /// method implementation.
    pub fn time_named_end(&self, name: &str) { js! { console.timeEnd(@{name}); } }


    /// [console.time](https://developer.mozilla.org/en-US/docs/Web/API/Console/time)
    /// method implementation.
    pub fn time(&self) { js! { console.time(); } }
    /// [console.time_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/time_end)
    /// method implementation.
    pub fn time_end(&self) { js! { console.timeEnd(); } }


    /// [console.clear](https://developer.mozilla.org/en-US/docs/Web/API/Console/clear)
    /// method implementation.
    pub fn clear(&self) { js! { console.clear(); } }

    /// [console.group](https://developer.mozilla.org/en-US/docs/Web/API/Console/group)
    /// method implementation.
    pub fn group(&self) { js! { console.group(); } }

    /// [console.group_collapsed](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_collapsed)
    /// method implementation.
    pub fn group_collapsed(&self) { js! { console.groupCollapsed(); } }

    /// [console.group_end](https://developer.mozilla.org/en-US/docs/Web/API/Console/group_end)
    /// method implementation.
    pub fn group_end(&self) { js! { console.groupEnd(); } }

    /// [console.trace](https://developer.mozilla.org/en-US/docs/Web/API/Console/trace)
    /// method implementation.
    pub fn trace(&self) { js! { console.trace(); } }

    /// [console.assert](https://developer.mozilla.org/en-US/docs/Web/API/Console/assert)
    /// method implementation.
    pub fn assert(&self, condition: bool, message: &str) { js! { console.assert(@{condition}, @{message}); } }
}

/// An abstract service which return a `Console` instance.
pub trait ConsoleService {
    /// Returns console from a context.
    fn get_console(&self) -> Console;
}

impl<MSG: 'static> ConsoleService for Context<MSG> {
    fn get_console(&self) -> Console {
        Console {}
    }
}
