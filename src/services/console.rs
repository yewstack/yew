use html::Context;

pub struct Console;

impl Console {
    pub fn log(&self, message: &str) { js! { console.log(@{message}); } }
    pub fn warn(&self, message: &str) { js! { console.warn(@{message}); } }
    pub fn info(&self, message: &str) { js! { console.info(@{message}); } }
    pub fn error(&self, message: &str) { js! { console.error(@{message}); } }
    pub fn debug(&self, message: &str) { js! { console.debug(@{message}); } }
    pub fn count_named(&self, name: &str) { js! { console.count(@{name}); } }
    pub fn count(&self) { js! { console.count(); } }

    pub fn time_named(&self, name: &str) { js! { console.time(@{name}); } }
    pub fn time_named_end(&self, name: &str) { js! { console.timeEnd(@{name}); } }

    pub fn time(&self) { js! { console.time(); } }
    pub fn time_end(&self) { js! { console.timeEnd(); } }

    pub fn clear(&self) { js! { console.clear(); } }
    pub fn group(&self) { js! { console.group(); } }
    pub fn group_collapsed(&self) { js! { console.groupCollapsed(); } }
    pub fn group_end(&self) { js! { console.groupEnd(); } }
    pub fn trace(&self) { js! { console.trace(); } }

    pub fn assert(&self, condition: bool, message: &str) { js! { console.assert(@{condition}, @{message}); } }
}

pub trait ConsoleService {
    fn get_console(&self) -> Console;
}

impl<MSG: 'static> ConsoleService for Context<MSG> {
    fn get_console(&self) -> Console {
        Console {}
    }
}
