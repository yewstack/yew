use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};

use thiserror::Error;

use crate::platform::spawn_local;
use crate::Callback;

thread_local! {
    static SUSPENSION_ID: RefCell<usize> = RefCell::default();
}

/// A Suspension.
///
/// This type can be sent back as an `Err(_)` to suspend a component until the underlying task
/// completes.
#[derive(Error, Debug, Clone)]
#[error("suspend component rendering")]
pub struct Suspension {
    id: usize,
    listeners: Rc<RefCell<Vec<Callback<Self>>>>,

    resumed: Rc<AtomicBool>,
}

impl PartialEq for Suspension {
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

impl Suspension {
    /// Creates a Suspension.
    pub fn new() -> (Self, SuspensionHandle) {
        let id = SUSPENSION_ID.with(|m| {
            let mut m = m.borrow_mut();
            *m += 1;

            *m
        });

        let self_ = Suspension {
            id,
            listeners: Rc::default(),
            resumed: Rc::default(),
        };

        (self_.clone(), SuspensionHandle { inner: self_ })
    }

    /// Returns `true` if the current suspension is already resumed.
    pub fn resumed(&self) -> bool {
        self.resumed.load(Ordering::Relaxed)
    }

    /// Creates a Suspension that resumes when the [`Future`] resolves.
    pub fn from_future(f: impl Future<Output = ()> + 'static) -> Self {
        let (self_, handle) = Self::new();

        spawn_local(async move {
            f.await;
            handle.resume();
        });

        self_
    }

    /// Listens to a suspension and get notified when it resumes.
    pub(crate) fn listen(&self, cb: Callback<Self>) {
        if self.resumed() {
            cb.emit(self.clone());
            return;
        }

        let mut listeners = self.listeners.borrow_mut();

        listeners.push(cb);
    }

    fn resume_by_ref(&self) {
        // The component can resume rendering by returning a non-suspended result after a state is
        // updated, so we always need to check here.
        if !self.resumed() {
            self.resumed.store(true, Ordering::Relaxed);
            let listeners = self.listeners.borrow();

            for listener in listeners.iter() {
                listener.emit(self.clone());
            }
        }
    }
}

impl Future for Suspension {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.resumed() {
            return Poll::Ready(());
        }

        let waker = cx.waker().clone();
        self.listen(Callback::from(move |_| {
            waker.wake_by_ref();
        }));

        Poll::Pending
    }
}

/// A Suspension Result.
pub type SuspensionResult<T> = std::result::Result<T, Suspension>;

/// A Suspension Handle.
///
/// This type is used to control the corresponding [`Suspension`].
///
/// When the current struct is dropped or `resume` is called, it will resume rendering of current
/// component.
#[derive(Debug, PartialEq)]
pub struct SuspensionHandle {
    inner: Suspension,
}

impl SuspensionHandle {
    /// Resumes component rendering.
    pub fn resume(self) {
        self.inner.resume_by_ref();
    }
}

impl Drop for SuspensionHandle {
    fn drop(&mut self) {
        self.inner.resume_by_ref();
    }
}
