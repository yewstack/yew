use std::fmt;
/// A wrapper around Option<T> that only allows items to be taken.
///
/// # Panics
/// It is expected to only take items from this structure in a way that
/// it will never be accessed after items have been taken.
///
/// Accessing this via `as_ref` or `as_mut` after taking the value will cause a panic.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Takeable<T>(Option<T>);

impl<T> Takeable<T> {
    pub(crate) fn new(item: T) -> Self {
        Takeable(Some(item))
    }

    /// This should only be called once.
    pub(crate) fn take(&mut self) -> T {
        self.0.take().expect("Can't take twice")
    }
}

impl<T> AsRef<T> for Takeable<T> {
    fn as_ref(&self) -> &T {
        self.0.as_ref().expect("Can't reference taken value")
    }
}

impl<T> AsMut<T> for Takeable<T> {
    fn as_mut(&mut self) -> &mut T {
        self.0.as_mut().expect("Can't reference taken value")
    }
}

impl<T: fmt::Debug> fmt::Debug for Takeable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.as_ref().expect("Can't reference taken value").fmt(f)
    }
}
