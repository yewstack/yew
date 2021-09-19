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
    main: Vec<Box<dyn Runnable>>,

    // Component queues
    destroy: Vec<Box<dyn Runnable>>,
    create: Vec<Box<dyn Runnable>>,
    update: Vec<Box<dyn Runnable>>,
    render_first: VecDeque<Box<dyn Runnable>>,
    render: RenderScheduler,

    /// Stacks to ensure child calls are always before parent calls
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
pub fn push(runnable: Box<dyn Runnable>) {
    with(|s| s.main.push(runnable));
    // Execute pending immediately. Necessary for runnables added outside the component lifecycle,
    // which would otherwise be delayed.
    start();
}

/// Push a component creation, first render and first rendered [Runnable]s to be executed
pub(crate) fn push_component_create(
    create: impl Runnable + 'static,
    first_render: impl Runnable + 'static,
    first_rendered: impl Runnable + 'static,
) {
    with(|s| {
        s.create.push(Box::new(create));
        s.render_first.push_back(Box::new(first_render));
        s.rendered_first.push(Box::new(first_rendered));
    });
}

/// Push a component destruction [Runnable] to be executed
pub(crate) fn push_component_destroy(runnable: impl Runnable + 'static) {
    with(|s| s.destroy.push(Box::new(runnable)));
}

/// Push a component render and rendered [Runnable]s to be executed
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
pub(crate) fn push_component_update(runnable: impl Runnable + 'static) {
    with(|s| s.update.push(Box::new(runnable)));
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
            let mut queue = vec![];
            loop {
                with(|s| s.fill_queue(&mut queue));
                if queue.is_empty() {
                    break;
                }
                for r in queue.drain(..) {
                    r.run();
                }
            }
        }
    });
}

impl Scheduler {
    /// Fill vector with tasks to be executed according to Runnable type execution priority
    ///
    /// This method is optimized for typical usage, where possible, but does not break on
    /// non-typical usage (like scheduling renders in [crate::Component::create()] or
    /// [crate::Component::rendered()] calls).
    fn fill_queue(&mut self, to_run: &mut Vec<Box<dyn Runnable>>) {
        // Placed first to avoid as much needless work as possible, handling all the other events.
        // Drained completely, because they are the highest priority events anyway.
        to_run.append(&mut self.destroy);

        // Create events can be batched, as they are typically just for object creation
        to_run.append(&mut self.create);

        // First render must never be skipped and takes priority over main, because it may need
        // to init `NodeRef`s
        //
        // Should be processed one at time, because they can spawn more create and rendered events
        // for their children.
        if let Some(r) = self.render_first.pop_front() {
            to_run.push(r);
        }

        // These typically do nothing and don't spawn any other events - can be batched.
        // Should be run only after all first renders have finished.
        if !to_run.is_empty() {
            return;
        }
        to_run.extend(self.rendered_first.drain(..).rev());

        // Updates are after the first render to ensure we always have the entire child tree
        // rendered, once an update is processed.
        //
        // Can be batched, as they can cause only non-first renders.
        to_run.append(&mut self.update);

        // Likely to cause duplicate renders via component updates, so placed before them
        to_run.append(&mut self.main);

        // Run after all possible updates to avoid duplicate renders.
        //
        // Should be processed one at time, because they can spawn more create and first render
        // events for their children.
        if !to_run.is_empty() {
            return;
        }
        if let Some(r) = self.render.pop() {
            to_run.push(r);
        }

        // These typically do nothing and don't spawn any other events - can be batched.
        // Should be run only after all renders have finished.
        if !to_run.is_empty() {
            return;
        }
        self.rendered.drain_into(to_run);
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
}

/// Deduplicating scheduler for component rendered calls with deduplication
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

    /// Drain all tasks into `dst`, if any
    fn drain_into(&mut self, dst: &mut Vec<Box<dyn Runnable>>) {
        for id in self.stack.drain(..).rev() {
            if let Some(t) = self.tasks.remove(&id) {
                dst.push(t);
            }
        }
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
