//! This module contains a scheduler.

use std::cell::RefCell;

use crate::platform::spawn_local;

type Runnable = Box<dyn FnOnce()>;

#[derive(Default)]
struct FifoQueue {
    inner: Vec<Runnable>,
}

impl FifoQueue {
    #[inline(always)]
    fn push(&mut self, task: Runnable) {
        self.inner.push(task);
    }

    #[inline(always)]
    fn drain_into(&mut self, queue: &mut Vec<Runnable>) {
        queue.append(&mut self.inner);
    }
}

/// This is a global scheduler suitable to schedule and run any tasks.
#[derive(Default)]
#[allow(missing_debug_implementations)] // todo
struct Scheduler {
    // Main queue
    main: FifoQueue,
}

/// Execute closure with a mutable reference to the scheduler
#[inline]
fn with<R>(f: impl FnOnce(&mut Scheduler) -> R) -> R {
    thread_local! {
        /// This is a global scheduler suitable to schedule and run any tasks.
        ///
        /// Exclusivity of mutable access is controlled by only accessing it through a set of public
        /// functions.
        static SCHEDULER: RefCell<Scheduler> = Default::default();
    }

    SCHEDULER.with(|s| f(&mut s.borrow_mut()))
}

/// Push a generic [Runnable] to be executed
#[inline(always)]
pub fn push<F>(runnable: F)
where
    F: FnOnce() + 'static,
{
    with(|s| s.main.push(Box::new(runnable)));
    // Execute pending immediately. Necessary for runnables added outside the component lifecycle,
    // which would otherwise be delayed.
    start();
}

/// Execute any pending [Runnable]s
pub(crate) fn start_now() {
    #[tracing::instrument(level = tracing::Level::DEBUG)]
    fn scheduler_loop() {
        let mut queue = vec![];
        loop {
            with(|s| s.fill_queue(&mut queue));
            if queue.is_empty() {
                break;
            }
            for r in queue.drain(..) {
                r();
            }
        }
    }

    thread_local! {
        // The lock is used to prevent recursion. If the lock cannot be acquired, it is because the
        // `start()` method is being called recursively as part of a `runnable.run()`.
        static LOCK: RefCell<()> = Default::default();
    }

    LOCK.with(|l| {
        if let Ok(_lock) = l.try_borrow_mut() {
            scheduler_loop();
        }
    });
}

/// We delay the start of the scheduler to the end of the micro task queue.
/// So any messages that needs to be queued can be queued.
pub(crate) fn start() {
    spawn_local(async {
        start_now();
    });
}

impl Scheduler {
    /// Fill vector with tasks to be executed according to Runnable type execution priority
    ///
    /// This method is optimized for typical usage, where possible, but does not break on
    /// non-typical usage (like scheduling renders in [crate::Component::create()] or
    /// [crate::Component::rendered()] calls).
    fn fill_queue(&mut self, to_run: &mut Vec<Runnable>) {
        // Likely to cause duplicate renders via component updates, so placed before them
        self.main.drain_into(to_run);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_executes_runnables_immediately() {
        use std::cell::Cell;

        thread_local! {
            static FLAG: Cell<bool> = Default::default();
        }

        push(|| FLAG.with(|v| v.set(true)));
        FLAG.with(|v| assert!(v.get()));
    }
}
