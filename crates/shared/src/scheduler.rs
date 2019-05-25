//! This module contains a scheduler.

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) type Shared<T> = Rc<RefCell<T>>;

thread_local! {
    static SCHEDULER: Rc<Scheduler> =
        Rc::new(Scheduler::new());
}

pub(crate) fn scheduler() -> Rc<Scheduler> {
    SCHEDULER.with(Rc::clone)
}

/// A routine which could be run.
pub(crate) trait Runnable {
    /// Runs a routine with a context instance.
    fn run(&mut self);
}

/// This is a global scheduler suitable to schedule and run any tasks.
pub(crate) struct Scheduler {
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

    pub(crate) fn put_and_try_run(&self, runnable: Box<dyn Runnable>) {
        self.sequence.borrow_mut().push_back(runnable);
        if self.lock.compare_and_swap(false, true, Ordering::Relaxed) == false {
            loop {
                let do_next = self.sequence.borrow_mut().pop_front();
                if let Some(mut runnable) = do_next {
                    runnable.run();
                } else {
                    break;
                }
            }
            self.lock.store(false, Ordering::Relaxed);
        }
    }
}
