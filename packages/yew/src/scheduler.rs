//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// Alias for `Rc<RefCell<T>>`
pub type Shared<T> = Rc<RefCell<T>>;

/// A routine which could be run.
pub trait Runnable {
    /// Runs a routine with a context instance.
    fn run(self: Box<Self>);
}

struct QueueEntry {
    task: Box<dyn Runnable>,
}

#[derive(Default)]
struct FifoQueue {
    inner: Vec<QueueEntry>,
}

impl FifoQueue {
    fn push(&mut self, task: Box<dyn Runnable>) {
        self.inner.push(QueueEntry { task });
    }

    fn drain_into(&mut self, queue: &mut Vec<QueueEntry>) {
        queue.append(&mut self.inner);
    }
}

#[derive(Default)]

struct TopologicalQueue {
    /// The Binary Tree Map guarantees components with lower id (parent) is rendered first
    inner: BTreeMap<usize, QueueEntry>,
}

impl TopologicalQueue {
    #[cfg(any(feature = "ssr", feature = "csr"))]
    fn push(&mut self, component_id: usize, task: Box<dyn Runnable>) {
        self.inner.insert(component_id, QueueEntry { task });
    }

    /// Take a single entry, preferring parents over children
    fn pop_topmost(&mut self) -> Option<QueueEntry> {
        // To be replaced with BTreeMap::pop_first once it is stable.
        let key = *self.inner.keys().next()?;
        self.inner.remove(&key)
    }

    /// Drain all entries, such that children are queued before parents
    fn drain_post_order_into(&mut self, queue: &mut Vec<QueueEntry>) {
        if self.inner.is_empty() {
            return;
        }
        let rendered = std::mem::take(&mut self.inner);
        // Children rendered lifecycle happen before parents.
        queue.extend(rendered.into_values().rev());
    }
}

/// This is a global scheduler suitable to schedule and run any tasks.
#[derive(Default)]
#[allow(missing_debug_implementations)] // todo
struct Scheduler {
    // Main queue
    main: FifoQueue,

    // Component queues
    destroy: FifoQueue,
    create: FifoQueue,

    props_update: FifoQueue,
    update: FifoQueue,

    render: TopologicalQueue,
    render_first: TopologicalQueue,
    render_priority: TopologicalQueue,

    rendered_first: TopologicalQueue,
    rendered: TopologicalQueue,
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
pub fn push(runnable: Box<dyn Runnable>) {
    with(|s| s.main.push(runnable));
    // Execute pending immediately. Necessary for runnables added outside the component lifecycle,
    // which would otherwise be delayed.
    start();
}

#[cfg(any(feature = "ssr", feature = "csr"))]
mod feat_csr_ssr {
    use super::*;
    /// Push a component creation, first render and first rendered [Runnable]s to be executed
    pub(crate) fn push_component_create(
        component_id: usize,
        create: Box<dyn Runnable>,
        first_render: Box<dyn Runnable>,
    ) {
        with(|s| {
            s.create.push(create);
            s.render_first.push(component_id, first_render);
        });
    }

    /// Push a component destruction [Runnable] to be executed
    pub(crate) fn push_component_destroy(runnable: Box<dyn Runnable>) {
        with(|s| s.destroy.push(runnable));
    }

    /// Push a component render [Runnable]s to be executed
    pub(crate) fn push_component_render(component_id: usize, render: Box<dyn Runnable>) {
        with(|s| {
            s.render.push(component_id, render);
        });
    }

    /// Push a component update [Runnable] to be executed
    pub(crate) fn push_component_update(runnable: Box<dyn Runnable>) {
        with(|s| s.update.push(runnable));
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
pub(crate) use feat_csr_ssr::*;

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    pub(crate) fn push_component_rendered(
        component_id: usize,
        rendered: Box<dyn Runnable>,
        first_render: bool,
    ) {
        with(|s| {
            if first_render {
                s.rendered_first.push(component_id, rendered);
            } else {
                s.rendered.push(component_id, rendered);
            }
        });
    }

    pub(crate) fn push_component_props_update(props_update: Box<dyn Runnable>) {
        with(|s| s.props_update.push(props_update));
    }
}

#[cfg(feature = "csr")]
pub(crate) use feat_csr::*;

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    pub(crate) fn push_component_priority_render(component_id: usize, render: Box<dyn Runnable>) {
        with(|s| {
            s.render_priority.push(component_id, render);
        });
    }
}

#[cfg(feature = "hydration")]
pub(crate) use feat_hydration::*;

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
                r.task.run();
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

#[cfg(target_arch = "wasm32")]
mod arch {
    use crate::platform::spawn_local;

    /// We delay the start of the scheduler to the end of the micro task queue.
    /// So any messages that needs to be queued can be queued.
    pub(crate) fn start() {
        spawn_local(async {
            super::start_now();
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod arch {
    // Delayed rendering is not very useful in the context of server-side rendering.
    // There are no event listeners or other high priority events that need to be
    // processed and we risk of having a future un-finished.
    // Until scheduler is future-capable which means we can join inside a future,
    // it can remain synchronous.
    pub(crate) fn start() {
        super::start_now();
    }
}

pub(crate) use arch::*;

impl Scheduler {
    /// Fill vector with tasks to be executed according to Runnable type execution priority
    ///
    /// This method is optimized for typical usage, where possible, but does not break on
    /// non-typical usage (like scheduling renders in [crate::Component::create()] or
    /// [crate::Component::rendered()] calls).
    fn fill_queue(&mut self, to_run: &mut Vec<QueueEntry>) {
        // Placed first to avoid as much needless work as possible, handling all the other events.
        // Drained completely, because they are the highest priority events anyway.
        self.destroy.drain_into(to_run);

        // Create events can be batched, as they are typically just for object creation
        self.create.drain_into(to_run);

        // These typically do nothing and don't spawn any other events - can be batched.
        // Should be run only after all first renders have finished.
        if !to_run.is_empty() {
            return;
        }

        // First render must never be skipped and takes priority over main, because it may need
        // to init `NodeRef`s
        //
        // Should be processed one at time, because they can spawn more create and rendered events
        // for their children.
        if let Some(r) = self.render_first.pop_topmost() {
            to_run.push(r);
            return;
        }

        self.props_update.drain_into(to_run);

        // Priority rendering
        //
        // This is needed for hydration susequent render to fix node refs.
        if let Some(r) = self.render_priority.pop_topmost() {
            to_run.push(r);
            return;
        }

        // Children rendered lifecycle happen before parents.
        self.rendered_first.drain_post_order_into(to_run);

        // Updates are after the first render to ensure we always have the entire child tree
        // rendered, once an update is processed.
        //
        // Can be batched, as they can cause only non-first renders.
        self.update.drain_into(to_run);

        // Likely to cause duplicate renders via component updates, so placed before them
        self.main.drain_into(to_run);

        // Run after all possible updates to avoid duplicate renders.
        //
        // Should be processed one at time, because they can spawn more create and first render
        // events for their children.
        if !to_run.is_empty() {
            return;
        }

        // Should be processed one at time, because they can spawn more create and rendered events
        // for their children.
        if let Some(r) = self.render.pop_topmost() {
            to_run.push(r);
            return;
        }
        // These typically do nothing and don't spawn any other events - can be batched.
        // Should be run only after all renders have finished.
        // Children rendered lifecycle happen before parents.
        self.rendered.drain_post_order_into(to_run);
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
