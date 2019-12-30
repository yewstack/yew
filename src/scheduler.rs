//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Provides a mutable reference counted value
#[doc(hidden)]
pub type Shared<T> = Rc<RefCell<T>>;

thread_local! {
    static SCHEDULER: Rc<Scheduler> =
        Rc::new(Scheduler::new());
}

/// Provides a task queue for the current thread.
#[doc(hidden)]
pub fn scheduler() -> Rc<Scheduler> {
    SCHEDULER.with(Rc::clone)
}

/// A routine which could be run.
#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub trait Runnable {
    /// Runs a routine with a context instance.
    fn run(self: Box<Self>);
}

/// This is a global scheduler suitable to schedule and run any tasks.
#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct Scheduler {
    lock: Rc<AtomicBool>,
    sequence: Shared<VecDeque<Box<dyn Runnable>>>,
}

impl Clone for Scheduler {
    fn clone(&self) -> Self {
        Scheduler {
            lock: self.lock.clone(),
            sequence: self.sequence.clone(),
        }
    }
}

impl Scheduler {
    /// Creates a new scheduler with a context.
    fn new() -> Self {
        let sequence = VecDeque::new();
        Scheduler {
            lock: Rc::new(AtomicBool::new(false)),
            sequence: Rc::new(RefCell::new(sequence)),
        }
    }

    /// Adds a task to the queue and runs it, if possible
    #[doc(hidden)]
    pub fn put_and_try_run(&self, runnable: Box<dyn Runnable>) {
        self.sequence.borrow_mut().push_back(runnable);
        if self.lock.compare_and_swap(false, true, Ordering::Relaxed) {
            return;
        }

        loop {
            let do_next = self.sequence.borrow_mut().pop_front();
            if let Some(runnable) = do_next {
                runnable.run();
            } else {
                break;
            }
        }
        self.lock.store(false, Ordering::Relaxed);
    }
}
