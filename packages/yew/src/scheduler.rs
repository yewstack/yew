//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::{hash_map::Entry, HashMap, VecDeque};
use std::rc::Rc;

/// Alias for Rc<RefCell<T>>
pub type Shared<T> = Rc<RefCell<T>>;

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
    destroy: VecDeque<(usize, Box<dyn Runnable>)>,
    create: VecDeque<Box<dyn Runnable>>,
    update: VecDeque<Box<dyn Runnable>>,
    render_first: VecDeque<Box<dyn Runnable>>,
    render: RenderScheduler,

    /// Deduplicating stacks to ensure child calls are always before parent calls
    rendered_first: Vec<Box<dyn Runnable>>,
    rendered: RenderedScheduler,
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

    SCHEDULER.with(|s| f(&mut *s.borrow_mut()))
}

/// Push a generic [Runnable] to be executed
#[inline]
pub fn push(runnable: Box<dyn Runnable>) {
    with(|s| s.main.push_back(runnable));
    // Execute pending immediately. Necessary for runnables added outside the component lifecycle,
    // which would otherwise be delayed.
    start();
}

/// Push a component creation, first render and rendered [Runnable]s to be executed
#[inline]
pub(crate) fn push_component_create(
    create: impl Runnable + 'static,
    first_render: impl Runnable + 'static,
    first_rendered: impl Runnable + 'static,
) {
    with(|s| {
        s.create.push_back(Box::new(create));
        s.render_first.push_back(Box::new(first_render));
        s.rendered_first.push(Box::new(first_rendered));
    });
}

/// Push a component destruction [Runnable] to be executed
#[inline]
pub(crate) fn push_component_destroy(component_id: usize, runnable: impl Runnable + 'static) {
    with(|s| s.destroy.push_back((component_id, Box::new(runnable))));
}

/// Push a component render and rendered [Runnable]s to be executed
#[inline]
pub(crate) fn push_component_render(
    component_id: usize,
    render: impl Runnable + 'static,
    rendered: impl Runnable + 'static,
) {
    with(|s| {
        s.render.schedule(component_id, Box::new(render));
        s.rendered.schedule(component_id, Box::new(rendered));
    });
}

/// Push a component update [Runnable] to be executed
#[inline]
pub(crate) fn push_component_update(runnable: impl Runnable + 'static) {
    with(|s| s.update.push_back(Box::new(runnable)));
}

/// Execute any pending [Runnable]s
pub(crate) fn start() {
    thread_local! {
        // The lock is used to prevent recursion. If the lock cannot be acquired, it is because the
        // `start()` method is being called recursively as part of a `runnable.run()`.
        static LOCK: RefCell<()> = Default::default();
    }

    LOCK.with(|l| {
        if let Ok(_lock) = l.try_borrow_mut() {
            while let Some(runnable) = with(|s| s.next_runnable()) {
                runnable.run();
            }
        }
    });
}

impl Scheduler {
    /// Pop next Runnable to be executed according to Runnable type execution priority
    fn next_runnable(&mut self) -> Option<Box<dyn Runnable>> {
        // Placed first to avoid as much needless work as possible, handling all the other events
        if let Some((id, runnable)) = self.destroy.pop_front() {
            // Potentially avoids 2 scheduler cycles on removed components
            self.render.unschedule(&id);
            self.rendered.unschedule(&id);

            return Some(runnable);
        }

        self.create
            .pop_front()
            // First render must never be skipped and takes priority over main, because it may need
            // to init `NodeRef`s
            .or_else(|| self.render_first.pop_front())
            .or_else(|| self.rendered_first.pop())
            // Updates are after the first render to ensure we always have the entire child tree
            // rendered, once an update is processed.
            .or_else(|| self.update.pop_front())
            // Likely to cause duplicate renders, so placed before them
            .or_else(|| self.main.pop_front())
            // Most expensive, as these call `Component::view()`
            .or_else(|| self.render.pop())
            // `rendered()` should be run last, when we are sure there are no more renders, to
            // reduce workload.
            .or_else(|| self.rendered.pop())
    }
}

/// Task to be executed for specific component
struct QueueTask {
    /// Tasks in the queue to skip for this component
    skip: usize,

    /// Runnable to execute
    runnable: Box<dyn Runnable>,
}

/// Scheduler for non-first component renders with deduplication
#[derive(Default)]
struct RenderScheduler {
    /// Task registry by component ID
    tasks: HashMap<usize, QueueTask>,

    /// Task queue by component ID
    queue: VecDeque<usize>,
}

impl RenderScheduler {
    /// Schedule render task execution
    fn schedule(&mut self, component_id: usize, runnable: Box<dyn Runnable>) {
        self.queue.push_back(component_id);
        match self.tasks.entry(component_id) {
            Entry::Vacant(e) => {
                e.insert(QueueTask { skip: 0, runnable });
            }
            Entry::Occupied(mut e) => {
                let v = e.get_mut();
                v.skip += 1;

                // Technically the 2 runners should be functionally identical, but might as well
                // overwrite it for good measure, accounting for future changes. We have it here
                // anyway.
                v.runnable = runnable;
            }
        }
    }

    /// Try to pop a task from the queue, if any
    fn pop(&mut self) -> Option<Box<dyn Runnable>> {
        while let Some(id) = self.queue.pop_front() {
            match self.tasks.entry(id) {
                Entry::Occupied(mut e) => {
                    let v = e.get_mut();
                    if v.skip == 0 {
                        return Some(e.remove().runnable);
                    }
                    v.skip -= 1;
                }
                Entry::Vacant(_) => (),
            }
        }
        None
    }

    /// Invalidate all render tasks for a given component_id
    fn unschedule(&mut self, component_id: &usize) {
        self.tasks.remove(component_id);
    }
}

/// Scheduler for component rendered calls with deduplication
#[derive(Default)]
struct RenderedScheduler {
    /// Task registry by component ID
    tasks: HashMap<usize, Box<dyn Runnable>>,

    /// Task stack by component ID
    stack: Vec<usize>,
}

impl RenderedScheduler {
    /// Schedule rendered task execution
    fn schedule(&mut self, component_id: usize, runnable: Box<dyn Runnable>) {
        if self.tasks.insert(component_id, runnable).is_none() {
            self.stack.push(component_id);
        }
    }

    /// Try to pop a task from the stack, if any
    fn pop(&mut self) -> Option<Box<dyn Runnable>> {
        while let Some(id) = self.stack.pop() {
            if let Some(runnable) = self.tasks.remove(&id) {
                return Some(runnable);
            }
        }
        None
    }

    /// Invalidate all rendered tasks for a given component_id
    fn unschedule(&mut self, component_id: &usize) {
        self.tasks.remove(component_id);
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

        struct Test;
        impl Runnable for Test {
            fn run(self: Box<Self>) {
                FLAG.with(|v| v.set(true));
            }
        }

        push(Box::new(Test));
        FLAG.with(|v| assert!(v.get()));
    }
}

// TODO: 100% coverage for scheduler
