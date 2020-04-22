use crate::ptr::rc_box::{
    clone_impl, clone_inner, decrement_and_possibly_deallocate, get_count, get_ref_boxed_content,
    is_exclusive, try_unwrap, unwrap_clone, RcBox,
};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ptr::NonNull;

/// Immutable Reference Counted pointer.
///
/// The `Irc` points to a value that cannot be mutated.
/// If a `Mrc` and `Irc` point to the same value, mutating the value via the `Mrc` will
/// clone and allocate, _then_ mutate the value, leaving the value pointed to by the `Irc` alone.
///
/// Unless the `Irc` is unwrapped, that value is mutated, and another `Irc` is created using that value
/// the `Irc` will guarantee that the value is not changed, and can be transparently passed to child components
/// with knowledge that its value matches that of an original `Mrc` or `Irc` that it was cloned from.
///
/// This makes `Irc`s ideal for passing around immutable views to data through components in Yew, as
/// cloning the `Irc` itself is cheap, and the `Irc` guarantees that its data cannot be changed by
/// some intermediate component without obvious unwrap --> modify --> rewrap operations.
pub struct Irc<T> {
    /// Pointer to the value and reference counter.
    pub(crate) ptr: NonNull<RcBox<T>>,
}

impl<T> Irc<T> {
    /// Allocates the value behind an `Irc` pointer.
    pub fn new(value: T) -> Self {
        let rc_box = RcBox::new(value);
        let ptr = rc_box.into_non_null();
        Self { ptr }
    }

    /// Tries to extract the value from the `Irc`, returning the `Irc` if there is one or
    /// more other smart pointers to the value.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Irc;
    /// let irc = Irc::new(0);
    ///
    /// let clone = irc.clone();
    /// let irc = irc.try_unwrap().expect_err("Should not be able to unwrap");
    ///
    /// std::mem::drop(clone);
    /// let value = irc.try_unwrap().expect("Should get value");
    /// ```
    pub fn try_unwrap(self) -> Result<T, Self> {
        try_unwrap(self.ptr).map_err(|ptr| {
            Self { ptr } // Recover the ptr
        })
    }

    /// Gets the reference count of the `Irc`.
    ///
    /// An exclusive `Irc` will have a count of `1`.
    /// The count is incremented on any cloning action and is decremented when `drop` is called.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Irc;
    /// let irc = Irc::new(0);
    /// assert_eq!(irc.get_count(), 1);
    ///
    /// let _clone = irc.clone();
    /// assert_eq!(irc.get_count(), 2);
    ///
    /// std::mem::drop(_clone);
    /// assert_eq!(irc.get_count(), 1);
    /// ```
    pub fn get_count(&self) -> usize {
        get_count(self.ptr)
    }

    //
    /// ```
    /// use yewtil::ptr::Irc;
    /// let irc = Irc::new(0);
    /// assert!(irc.is_exclusive());
    ///
    /// let _clone = irc.clone();
    /// assert!(!irc.is_exclusive());
    ///
    /// std::mem::drop(_clone);
    /// assert!(irc.is_exclusive());
    /// ```
    pub fn is_exclusive(&self) -> bool {
        is_exclusive(self.ptr)
    }
}

impl<T: Clone> Irc<T> {
    /// Unwraps the value from the `Irc`, cloning the value instead if another `Irc` or `Mrc` points
    /// to the same value.
    pub fn unwrap_clone(self) -> T {
        unwrap_clone(self.ptr)
    }
    /// Clones the wrapped value of the `Irc`.
    pub fn clone_inner(&self) -> T {
        clone_inner(self.ptr)
    }
}

impl<T> Drop for Irc<T> {
    fn drop(&mut self) {
        unsafe { decrement_and_possibly_deallocate(self.ptr) }
    }
}

impl<T: Default> Default for Irc<T> {
    fn default() -> Self {
        Irc::new(T::default())
    }
}

impl<T> Clone for Irc<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: clone_impl(self.ptr),
        }
    }
}

impl<T> AsRef<T> for Irc<T> {
    fn as_ref(&self) -> &T {
        get_ref_boxed_content(&self.ptr).value.as_ref()
    }
}

impl<T> Deref for Irc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Borrow<T> for Irc<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: PartialEq> PartialEq for Irc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T: Eq> Eq for Irc<T> {}

impl<T: PartialOrd> PartialOrd for Irc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T: Ord> Ord for Irc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T: Hash> Hash for Irc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: fmt::Debug> fmt::Debug for Irc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rc_box = get_ref_boxed_content(&self.ptr);
        f.debug_struct("Irc")
            .field("value", rc_box.value.as_ref())
            .field("count", &rc_box.get_count())
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new() {
        let _irc = Irc::new(0);
    }
}
