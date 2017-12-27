use html::Context;

pub trait AlertService {
    fn alert(&mut self, message: &str);
}

impl<MSG: 'static> AlertService for Context<MSG> {
    fn alert(&mut self, message: &str) {
        js! { alert(@{message}); }
    }
}
