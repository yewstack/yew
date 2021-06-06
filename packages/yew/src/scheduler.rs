//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// Alias for Rc<RefCell<T>>
pub type Shared<T> = Rc<RefCell<T>>;

thread_local! {
    /// This is a global scheduler suitable to schedule and run any tasks.
    ///
    /// Exclusivity of mutable access is controlled by only accessing it through a set of public
    /// functions.
    static SCHEDULER: RefCell<Scheduler> = Default::default();
}

/// A routine which could be run.
pub trait Runnable {
    /// Runs a routine with a context instance.
    fn run(self: Box<Self>);
}

/// This is a global scheduler suitable to schedule and run any tasks.
#[derive(Default)]
#[allow(missing_debug_implementations)] // todo
struct Scheduler {
    // Main queue
    main: VecDeque<Box<dyn Runnable>>,

    // Component queues
    destroy: VecDeque<Box<dyn Runnable>>,
    create: VecDeque<Box<dyn Runnable>>,
    update: VecDeque<Box<dyn Runnable>>,
    render: VecDeque<Box<dyn Runnable>>,

    // Stack
    rendered: Vec<Box<dyn Runnable>>,
}

/// Execute closure with a mutable reference to the scheduler
#[inline]
fn with(f: impl FnOnce(&mut Scheduler)) {
    SCHEDULER.with(|s| f(&mut *s.borrow_mut()));
}

/// Push a generic Runnable to be executed
#[inline]
pub fn push(runnable: Box<dyn Runnable>) {
    with(|s| s.main.push_back(runnable));
}

/// Push a component creation Runnable to be executed
#[inline]
pub(crate) fn push_component_create(runnable: Box<dyn Runnable>) {
    with(|s| s.create.push_back(runnable));
}

/// Push a component destruction Runnable to be executed
#[inline]
pub(crate) fn push_component_destroy(runnable: Box<dyn Runnable>) {
    with(|s| s.destroy.push_back(runnable));
}

/// Push a component render Runnable to be executed
#[inline]
pub(crate) fn push_component_render(runnable: Box<dyn Runnable>) {
    with(|s| s.render.push_back(runnable));
}

/// Push a component Runnable to be executed after a component is rendered
#[inline]
pub(crate) fn push_component_rendered(runnable: Box<dyn Runnable>) {
    with(|s| s.rendered.push(runnable));
}

/// Push a component update Runnable to be executed
#[inline]
pub(crate) fn push_component_update(runnable: Box<dyn Runnable>) {
    with(|s| s.update.push_back(runnable));
}

/// Push a batch of component updates to be executed
#[inline]
pub(crate) fn push_component_updates(it: impl IntoIterator<Item = Box<dyn Runnable>>) {
    with(|s| s.update.extend(it));
}

/// Execute any pending Runnables
pub(crate) fn start() {
    thread_local! {
        // The lock is used to prevent recursion. If the lock cannot be acquired, it is because the
        // `start()` method is being called recursively as part of a `runnable.run()`.
        static LOCK: RefCell<()> = Default::default();
    }

    LOCK.with(|l| {
        if let Ok(_lock) = l.try_borrow_mut() {
            while let Some(runnable) = SCHEDULER.with(|s| s.borrow_mut().next_runnable()) {
                runnable.run();
            }
        }
    });
}

impl Scheduler {
    /// Pop next Runnable to be executed according to Runnable type execution priority
    fn next_runnable(&mut self) -> Option<Box<dyn Runnable>> {
        self.destroy
            .pop_front()
            .or_else(|| self.create.pop_front())
            .or_else(|| self.update.pop_front())
            .or_else(|| self.render.pop_front())
            .or_else(|| self.rendered.pop())
            .or_else(|| self.main.pop_front())
    }
}
