//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
    fn run(self: Box<Self>);
}

/// This is a global scheduler suitable to schedule and run any tasks.
#[derive(Clone)]
pub(crate) struct Scheduler {
    lock: Rc<RefCell<()>>,
    main: Shared<VecDeque<Box<dyn Runnable>>>,
    create_component: Shared<VecDeque<Box<dyn Runnable>>>,
    mount_component: Shared<Vec<Box<dyn Runnable>>>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            lock: Rc::new(RefCell::new(())),
            main: Rc::new(RefCell::new(VecDeque::new())),
            create_component: Rc::new(RefCell::new(VecDeque::new())),
            mount_component: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(crate) fn push(&self, runnable: Box<dyn Runnable>) {
        self.main.borrow_mut().push_back(runnable);
        self.start();
    }

    pub(crate) fn push_create(&self, runnable: Box<dyn Runnable>) {
        self.create_component.borrow_mut().push_back(runnable);
        self.start();
    }

    pub(crate) fn push_mount(&self, runnable: Box<dyn Runnable>) {
        self.mount_component.borrow_mut().push(runnable);
        self.start();
    }

    pub(crate) fn start(&self) {
        let lock = self.lock.try_borrow_mut();
        if lock.is_err() {
            return;
        }

        loop {
            let do_next = self
                .create_component
                .borrow_mut()
                .pop_front()
                .or_else(|| self.mount_component.borrow_mut().pop())
                .or_else(|| self.main.borrow_mut().pop_front());
            if let Some(runnable) = do_next {
                runnable.run();
            } else {
                break;
            }
        }
    }
}
