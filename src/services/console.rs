use html::Context;
use super::{Level};

pub trait ConsoleService {
    fn console(&mut self, level: Level, message: &str);
}

impl<MSG: 'static> ConsoleService for Context<MSG> {
    fn console(&mut self, level: Level, message: &str) {
        match level {
          Level::Log => { js! { console.log(@{message}); } },
          Level::Warn => { js! { console.warn(@{message}); } },
          Level::Error => { js! { console.error(@{message}); } },
        }
    }
}
