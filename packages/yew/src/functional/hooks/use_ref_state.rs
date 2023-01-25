use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use super::{use_reducer, use_reducer_eq, Reducible, UseReducerDispatcher, UseReducerHandle};
use crate::functional::hook;
use crate::use_force_update;
use crate::UseForceUpdateHandle;

struct UseRefStateReducer<T> {
    value: Rc<RefCell<T>>,
}

impl<T> Reducible for UseRefStateReducer<T> {
    type Action = T;

    fn reduce(self: Rc<Self>, _: Self::Action) -> Rc<Self> {
        Rc::new(Self {
            value: self.value.clone(),
        })
    }
}

impl<T> PartialEq for UseRefStateReducer<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        *(self.value).borrow() == *(rhs.value).borrow()
    }
}

#[hook]
pub fn use_ref_state<T, F>(init_fn: F) -> UseRefStateHandle<T>
where
    T: 'static,
    F: FnOnce() -> T,
{
    let update_handle = use_force_update();
    let inner = use_reducer(move || UseRefStateReducer {
        value: Rc::new(RefCell::new(init_fn())),
    });

    UseRefStateHandle {
        update_handle,
        inner,
    }
}

/// [`use_ref_state`] but only re-renders when `prev_state != next_state`.
///
/// This hook requires the state to implement [`PartialEq`].
#[hook]
pub fn use_ref_state_eq<T, F>(init_fn: F) -> UseRefStateHandle<T>
where
    T: PartialEq + 'static,
    F: FnOnce() -> T,
{
    let update_handle = use_force_update();
    let inner = use_reducer_eq(move || UseRefStateReducer {
        value: Rc::new(RefCell::new(init_fn())),
    });

    UseRefStateHandle {
        update_handle,
        inner,
    }
}

/// State handle for the [`use_ref_state`] hook.
pub struct UseRefStateHandle<T> {
    update_handle: UseForceUpdateHandle,
    inner: UseReducerHandle<UseRefStateReducer<T>>,
}

impl<T: fmt::Debug> fmt::Debug for UseRefStateHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseRefStateHandle")
            .field("value", &format!("{:?}", *(*self.inner.value).borrow()))
            .finish()
    }
}

impl<T> UseRefStateHandle<T> {
    /// Mutate the value
    pub fn mutate<F>(&self, mut mutator: F)
    where
        F: FnMut(&mut T),
    {
        mutator(&mut (*self.inner.value).borrow_mut());
        self.update_handle.force_update();
    }
}

impl<T> Deref for UseRefStateHandle<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.inner.value
    }
}

impl<T> Clone for UseRefStateHandle<T> {
    fn clone(&self) -> Self {
        Self {
            update_handle: self.update_handle.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseRefStateHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

/// Setter handle for [`use_ref_state`] and [`use_ref_state_eq`] hook
pub struct UseRefStateMutator<T: 'static> {
    inner: UseReducerDispatcher<UseRefStateReducer<T>>,
}

impl<T> Clone for UseRefStateMutator<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> fmt::Debug for UseRefStateMutator<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseRefStateSetter").finish()
    }
}

impl<T> PartialEq for UseRefStateMutator<T> {
    fn eq(&self, rhs: &Self) -> bool {
        self.inner == rhs.inner
    }
}

impl<T> UseRefStateMutator<T> {
    /// Replaces the value
    pub fn set(&self, value: T) {
        self.inner.dispatch(value)
    }
}
