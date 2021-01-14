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
    /// This lock is used to prevent recursion in [Scheduler#start()](Scheduler#start())
    lock: Rc<RefCell<()>>,
    main: Shared<VecDeque<Box<dyn Runnable>>>,
    pub(crate) component: ComponentScheduler,
}

pub(crate) enum ComponentRunnableType {
    Create,
    Update,
    Render,
    Rendered,
    Destroy,
}

#[derive(Clone)]
pub(crate) struct ComponentScheduler {
    // Queues
    destroy: Shared<VecDeque<Box<dyn Runnable>>>,
    create: Shared<VecDeque<Box<dyn Runnable>>>,
    update: Shared<VecDeque<Box<dyn Runnable>>>,
    render: Shared<VecDeque<Box<dyn Runnable>>>,

    // Stack
    rendered: Shared<Vec<Box<dyn Runnable>>>,
}

impl ComponentScheduler {
    fn new() -> Self {
        ComponentScheduler {
            destroy: Rc::new(RefCell::new(VecDeque::new())),
            create: Rc::new(RefCell::new(VecDeque::new())),
            update: Rc::new(RefCell::new(VecDeque::new())),
            render: Rc::new(RefCell::new(VecDeque::new())),
            rendered: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub(crate) fn push_update_batch(&self, it: impl IntoIterator<Item = Box<dyn Runnable>>) {
        self.update.borrow_mut().extend(it);
    }

    pub(crate) fn push(&self, run_type: ComponentRunnableType, runnable: Box<dyn Runnable>) {
        match run_type {
            ComponentRunnableType::Create => self.create.borrow_mut().push_back(runnable),
            ComponentRunnableType::Update => self.update.borrow_mut().push_back(runnable),
            ComponentRunnableType::Render => self.render.borrow_mut().push_back(runnable),
            ComponentRunnableType::Rendered => self.rendered.borrow_mut().push(runnable),
            ComponentRunnableType::Destroy => self.destroy.borrow_mut().push_back(runnable),
        };
    }

    fn next_runnable(&self) -> Option<Box<dyn Runnable>> {
        self.destroy
            .borrow_mut()
            .pop_front()
            .or_else(|| self.create.borrow_mut().pop_front())
            .or_else(|| self.update.borrow_mut().pop_front())
            .or_else(|| self.render.borrow_mut().pop_front())
            .or_else(|| self.rendered.borrow_mut().pop())
    }
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            lock: Rc::new(RefCell::new(())),
            main: Rc::new(RefCell::new(VecDeque::new())),
            component: ComponentScheduler::new(),
        }
    }

    pub(crate) fn push(&self, runnable: Box<dyn Runnable>) {
        self.main.borrow_mut().push_back(runnable);
        self.start();
    }

    fn next_runnable(&self) -> Option<Box<dyn Runnable>> {
        self.component
            .next_runnable()
            .or_else(|| self.main.borrow_mut().pop_front())
    }

    pub(crate) fn start(&self) {
        // The lock is used to prevent recursion. If the lock
        // cannot be acquired, it is because the `start()` method
        // is being called recursively as part of a `runnable.run()`.
        if let Ok(_lock) = self.lock.try_borrow_mut() {
            while let Some(runnable) = self.next_runnable() {
                runnable.run();
            }
        }
    }
}
