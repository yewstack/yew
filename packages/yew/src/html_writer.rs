//! module to provide an html writer that can be written without mutable borrowing.

mod feat_ssr {
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub(crate) struct HtmlWriter {
        inner: RefCell<String>,
    }

    impl HtmlWriter {
        pub fn push_str(&self, s: &str) {
            self.inner.borrow_mut().push_str(s);
        }

        pub fn into_inner(self) -> String {
            self.inner.into_inner()
        }
    }
}

pub use feat_ssr::*;
