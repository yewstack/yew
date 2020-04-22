use crate::ptr::rc_box::{
    clone_impl, clone_inner, decrement_and_possibly_deallocate, get_count, get_mut_boxed_content,
    get_ref_boxed_content, is_exclusive, try_unwrap, unwrap_clone, RcBox,
};
use crate::ptr::Irc;
use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::NonNull;

/// Mutable Reference Counted pointer
///
/// The `Mrc` has similar semantics to `std::rc::Rc` pointer,
/// with notable differences that it does not support `Weak` pointers,
/// it supports `std::ops::DerefMut` via the possibly allocating `make_mut` function,
/// and that it can create immutable handles to its data (`Irc`).
///
/// This should make it just slightly more size efficient and performant than `Rc`,
/// and should be more ergonomic to use than `Rc` given that you can mutably
/// assign to it without much ceremony.
///
/// Passing `Irc` pointers to children guarantee that no intermediate component can modify the value
/// behind the pointer.
/// This makes it ideal for passing around configuration data where some components can ergonomicly
/// "modify" and cheaply pass the pointers back to parent components, while other components can only read it.
///
/// # Note
/// Assigning to an `Mrc` within Yew when you have passed shared copies of the ptr to child components,
/// will always end up cloning the value stored in the `Mrc`. `Rc` makes this performance cost explicit,
/// by making you use `Rc::make_mut()`. Because cloning is unavoidable in the context it was designed for,
/// `Mrc` opts to provide nicer ergonomics around assignment.
///
/// # Example
/// ```
/// use yewtil::ptr::Mrc;
///
/// let mut mrc = Mrc::new(5);
/// *mrc = 10; // This just replaces the value because the mrc isn't shared.
///
/// assert_eq!(*mrc, 10);
///
/// let clone = mrc.clone();
/// *mrc = 20; // This operation clones the value and allocates space for it.
///
/// assert_eq!(*clone, 10);
/// assert_eq!(*mrc, 20);
/// ```
pub struct Mrc<T> {
    /// Pointer to the value and reference counter.
    ptr: NonNull<RcBox<T>>,
}

impl<T> Mrc<T> {
    /// Allocates a value behind a `Mrc` pointer.
    pub fn new(value: T) -> Self {
        let rc_box = RcBox::new(value);
        let ptr = rc_box.into_non_null();
        Self { ptr }
    }

    /// Attempts to get a mutable reference to the wrapped value.
    ///
    /// If the pointer is not shared, it will return `Some`,
    /// whereas if multiple `Mrc`s or `Irc`s point to the value, this will return None.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Mrc;
    /// let mut mrc = Mrc::new(0);
    /// assert!(mrc.get_mut().is_some());
    ///
    /// let _clone = mrc.clone();
    /// assert!(mrc.get_mut().is_none());
    /// ```
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_exclusive() {
            Some(get_mut_boxed_content(&mut self.ptr).value.as_mut())
        } else {
            None
        }
    }

    /// Tries to extract the value from the Mrc, returning the Mrc if there is one or
    /// more other pointers to the value.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Mrc;
    /// let mrc = Mrc::new(0);
    ///
    /// let clone = mrc.clone();
    /// let mrc = mrc.try_unwrap().expect_err("Should not be able to unwrap");
    ///
    /// std::mem::drop(clone);
    /// let value = mrc.try_unwrap().expect("Should get value");
    /// ```
    pub fn try_unwrap(self) -> Result<T, Self> {
        try_unwrap(self.ptr).map_err(|ptr| {
            Self { ptr } // Recover the ptr
        })
    }

    /// Gets the reference count of the `Mrc`.
    ///
    /// An exclusive `Mrc` will have a count of `1`.
    /// The count is incremented on any cloning action and is decremented when `drop` is called.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Mrc;
    /// let mrc = Mrc::new(0);
    /// assert_eq!(mrc.get_count(), 1);
    ///
    /// let _clone = mrc.clone();
    /// assert_eq!(mrc.get_count(), 2);
    ///
    /// std::mem::drop(_clone);
    /// assert_eq!(mrc.get_count(), 1);
    /// ```
    pub fn get_count(&self) -> usize {
        get_count(self.ptr)
    }

    /// Returns `true` if no other pointers to the value exist.
    ///
    /// ```
    /// use yewtil::ptr::Mrc;
    /// let mrc = Mrc::new(0);
    /// assert!(mrc.is_exclusive());
    ///
    /// let _clone = mrc.clone();
    /// assert!(!mrc.is_exclusive());
    ///
    /// std::mem::drop(_clone);
    /// assert!(mrc.is_exclusive());
    /// ```
    pub fn is_exclusive(&self) -> bool {
        is_exclusive(self.ptr)
    }

    /// Returns an immutable reference counted pointer,
    /// pointing to the same value and reference count.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::{Mrc, Irc};
    /// let mrc: Mrc<usize> =  Mrc::new(0);
    /// let _irc: Irc<usize> = mrc.irc();
    ///
    /// assert!(!mrc.is_exclusive());
    /// ```
    pub fn irc(&self) -> Irc<T> {
        get_ref_boxed_content(&self.ptr).inc_count();
        Irc { ptr: self.ptr }
    }

    /// Converts this Mrc into an Irc.
    /// # Example
    /// ```
    /// use yewtil::ptr::{Mrc, Irc};
    /// let mrc: Mrc<usize> =  Mrc::new(0);
    /// let irc: Irc<usize> = mrc.into_irc();
    ///
    /// assert!(irc.is_exclusive());
    /// ```
    pub fn into_irc(self) -> Irc<T> {
        // Because the Mrc is dropped, decrementing the count,
        // the count needs to be restored here.
        get_ref_boxed_content(&self.ptr).inc_count();
        Irc { ptr: self.ptr }
    }

    /// Checks pointers for equality.
    ///
    /// # Example
    /// ```
    /// use yewtil::ptr::Mrc;
    /// let mrc1 = Mrc::new(0);
    /// let mrc2 = Mrc::new(0);
    /// assert_eq!(mrc1, mrc2);
    /// assert!(!Mrc::ptr_eq(&mrc1, &mrc2))
    /// ```
    pub fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
        std::ptr::eq(lhs.ptr.as_ptr(), rhs.ptr.as_ptr())
    }
}

impl<T: Clone> Mrc<T> {
    /// Returns a mutable reference to the value if it has exclusive access.
    /// If it does not have exclusive access, it will make a clone of the data to acquire exclusive access.
    ///
    /// # Example
    /// ```
    ///# use yewtil::ptr::Mrc;
    /// let mut mrc: Mrc<usize> = Mrc::new(0);
    ///
    /// let _mut_ref: &mut usize = mrc.make_mut();
    /// assert_eq!(mrc.get_count(), 1);
    ///
    /// let clone = mrc.clone();
    /// assert_eq!(mrc.get_count(), 2);
    ///
    /// let _mut_ref: &mut usize = mrc.make_mut();
    /// assert_eq!(mrc.get_count(), 1);
    /// assert!(!Mrc::ptr_eq(&mrc, &clone))
    /// ```
    pub fn make_mut(&mut self) -> &mut T {
        if !self.is_exclusive() {
            let rc_box = RcBox::new(self.clone_inner());
            let ptr = rc_box.into_non_null();

            // decrement the count for the boxed content at the current pointer
            // because this Mrc will point to a new value.

            // This doesn't need to check to deallocate, because the count is guaranteed to be > 1.
            get_ref_boxed_content(&self.ptr).dec_count();

            // Replace the pointers
            self.ptr = ptr;
        }

        get_mut_boxed_content(&mut self.ptr).value.as_mut()
    }

    /// Consumes the `Mrc` and returns the value from the `Mrc` if it is not shared
    /// or clones the value if another `Mrc` or `Irc` has access to it.
    pub fn unwrap_clone(self) -> T {
        unwrap_clone(self.ptr)
    }
    /// Clones the value wrapped by the `Mrc`..
    pub fn clone_inner(&self) -> T {
        clone_inner(self.ptr)
    }
}

impl<T> Drop for Mrc<T> {
    fn drop(&mut self) {
        unsafe { decrement_and_possibly_deallocate(self.ptr) }
    }
}

impl<T> Clone for Mrc<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: clone_impl(self.ptr),
        }
    }
}

impl<T: Default> Default for Mrc<T> {
    fn default() -> Self {
        Mrc::new(T::default())
    }
}

impl<T: Clone> DerefMut for Mrc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.make_mut()
    }
}

impl<T: Clone> AsMut<T> for Mrc<T> {
    fn as_mut(&mut self) -> &mut T {
        self.make_mut()
    }
}

impl<T: Clone> BorrowMut<T> for Mrc<T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.make_mut()
    }
}

impl<T> AsRef<T> for Mrc<T> {
    fn as_ref(&self) -> &T {
        get_ref_boxed_content(&self.ptr).value.as_ref()
    }
}

impl<T> Deref for Mrc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Borrow<T> for Mrc<T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<T: PartialEq> PartialEq for Mrc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<T: Eq> Eq for Mrc<T> {}

impl<T: PartialOrd> PartialOrd for Mrc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T: Ord> Ord for Mrc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T: Hash> Hash for Mrc<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: fmt::Debug> fmt::Debug for Mrc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rc_box = get_ref_boxed_content(&self.ptr);
        f.debug_struct("Irc")
            .field("value", rc_box.value.as_ref())
            .field("count", &rc_box.get_count())
            .finish()
    }
}
