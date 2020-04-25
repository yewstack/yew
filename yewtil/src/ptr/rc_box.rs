use crate::ptr::takeable::Takeable;
use crate::ptr::IsZero;
use std::cell::Cell;
use std::ptr::NonNull;

#[derive(Debug)]
pub(crate) struct RcBox<T> {
    pub(crate) value: Takeable<T>,
    count: Cell<usize>,
}

/// The boxed content used in Irc and Mrc.
impl<T> RcBox<T> {
    #[inline]
    pub(crate) fn new(value: T) -> Self {
        Self {
            value: Takeable::new(value),
            count: Cell::new(1),
        }
    }

    #[inline]
    pub(crate) fn into_non_null(self) -> NonNull<Self> {
        unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(self))) }
    }

    /// Gets the reference count of the node
    pub(crate) fn get_count(&self) -> usize {
        self.count.get()
    }

    /// Increments the reference count of the node.
    #[inline]
    pub(crate) fn inc_count(&self) {
        let mut count = self.count.get();
        count += 1;
        self.count.set(count);
    }

    /// Decrements the reference count of the node.
    /// It will return true if the count hits zero.
    /// This can be used to determine if the node should be deallocated.
    #[inline]
    pub(crate) fn dec_count(&self) -> IsZero {
        let mut count = self.count.get();
        count -= 1;
        self.count.set(count);
        count == 0
    }

    #[inline]
    pub(crate) fn is_exclusive(&self) -> bool {
        self.get_count() == 1
    }
}

#[inline]
pub(crate) unsafe fn decrement_and_possibly_deallocate<T>(node: NonNull<RcBox<T>>) {
    // If the ref-count becomes 0
    if node.as_ref().dec_count() {
        std::ptr::drop_in_place(node.as_ptr());
    }
}

#[inline(always)]
pub(crate) fn get_mut_boxed_content<T>(ptr: &mut NonNull<RcBox<T>>) -> &mut RcBox<T> {
    unsafe { ptr.as_mut() }
}
#[inline(always)]
pub(crate) fn get_ref_boxed_content<T>(ptr: &NonNull<RcBox<T>>) -> &RcBox<T> {
    unsafe { ptr.as_ref() }
}

#[inline]
pub(crate) fn get_count<T>(ptr: NonNull<RcBox<T>>) -> usize {
    get_ref_boxed_content(&ptr).get_count()
}

#[inline]
pub(crate) fn is_exclusive<T>(ptr: NonNull<RcBox<T>>) -> bool {
    get_ref_boxed_content(&ptr).is_exclusive()
}

#[inline]
pub(crate) fn try_unwrap<T>(mut ptr: NonNull<RcBox<T>>) -> Result<T, NonNull<RcBox<T>>> {
    if is_exclusive(ptr) {
        Ok(get_mut_boxed_content(&mut ptr).value.take())
    } else {
        // The ptr's drop has ran, decrementing the count
        // This restores the value.
        get_ref_boxed_content(&ptr).inc_count();
        Err(ptr)
    }
}

#[inline]
pub(crate) fn clone_inner<T: Clone>(ptr: NonNull<RcBox<T>>) -> T {
    get_ref_boxed_content(&ptr).value.as_ref().clone()
}

#[inline]
pub(crate) fn unwrap_clone<T: Clone>(mut ptr: NonNull<RcBox<T>>) -> T {
    if is_exclusive(ptr) {
        get_mut_boxed_content(&mut ptr).value.take()
    } else {
        clone_inner(ptr)
    }
}

/// Clones the pointer after incrementing the reference count.
#[inline]
pub(crate) fn clone_impl<T>(ptr: NonNull<RcBox<T>>) -> NonNull<RcBox<T>> {
    // Increment the ref count
    get_ref_boxed_content(&ptr).inc_count();
    // rerturn the ptr
    ptr
}
