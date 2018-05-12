//! This module contains a scheduler.

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

/// The flag that means the routine should be destroyed.
pub(crate) type WillDestroy = bool;

/// Unspecified routine binded to a context.
pub(crate) type BoxedRunnable<CTX> = Box<Runnable<CTX>>;

/// A routine which could be run.
pub(crate) trait Runnable<CTX> {
    /// Runs a routine with a context instance.
    fn run<'a>(&mut self, context: &'a mut CTX) -> WillDestroy;
}

pub(crate) type Shared<T> = Rc<RefCell<T>>;

/// This is a global scheduler suitable to schedule and run any tasks.
pub struct Scheduler<CTX> {
    context: Shared<CTX>,
    sequence: Shared<VecDeque<Shared<BoxedRunnable<CTX>>>>,
}

impl<CTX> Clone for Scheduler<CTX> {
    fn clone(&self) -> Self {
        Scheduler {
            context: self.context.clone(),
            sequence: self.sequence.clone(),
        }
    }
}

impl<CTX> Scheduler<CTX> {
    /// Creates a new scheduler with a context.
    pub fn new(context: CTX) -> Self {
        let sequence = VecDeque::new();
        Scheduler {
            context: Rc::new(RefCell::new(context)),
            sequence: Rc::new(RefCell::new(sequence)),
        }
    }

    pub(crate) fn put_and_try_run(&mut self, runnable: Shared<BoxedRunnable<CTX>>) {
        self.sequence.borrow_mut().push_back(runnable);
        // Context lock also means the loop is running
        if let Ok(ref mut context) = self.context.try_borrow_mut() {
            loop {
                let do_next = self.sequence.borrow_mut().pop_front();
                if let Some(routine) = do_next {
                    routine.borrow_mut().run(context);
                } else {
                    break;
                }
            }
        }
    }
}
