//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

/// Alias for `Rc<RefCell<T>>`
pub type Shared<T> = Rc<RefCell<T>>;

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

    fn drain_into(&mut self, queue: &mut Vec<Runnable>) {
        queue.append(&mut self.inner);
    }
}

#[derive(Default)]
struct TopologicalQueue {
    /// The Binary Tree Map guarantees components with lower id (parent) is rendered first
    inner: BTreeMap<usize, Runnable>,
}

impl TopologicalQueue {
    #[cfg(any(feature = "ssr", feature = "csr"))]
    fn push(&mut self, component_id: usize, task: Runnable) {
        self.inner.insert(component_id, task);
    }

    /// Take a single entry, preferring parents over children
    fn pop_topmost(&mut self) -> Option<Runnable> {
        // To be replaced with BTreeMap::pop_first once it is stable.
        let key = *self.inner.keys().next()?;
        self.inner.remove(&key)
    }

    /// Drain all entries, such that children are queued before parents
    fn drain_post_order_into(&mut self, queue: &mut Vec<Runnable>) {
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

    props_update: FifoQueue,

    render: TopologicalQueue,
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
pub fn push<F>(runnable: F)
where
    F: FnOnce() + 'static,
{
    with(|s| s.main.push(Box::new(runnable)));
    // Execute pending immediately. Necessary for runnables added outside the component lifecycle,
    // which would otherwise be delayed.
    start();
}

#[cfg(any(feature = "ssr", feature = "csr"))]
mod feat_csr_ssr {
    use super::*;
    /// Push a component render [Runnable]s to be executed
    pub(crate) fn push_component_render<F>(component_id: usize, render: F)
    where
        F: FnOnce() + 'static,
    {
        with(|s| {
            s.render.push(component_id, Box::new(render));
        });
    }
}

#[cfg(any(feature = "ssr", feature = "csr"))]
pub(crate) use feat_csr_ssr::*;

#[cfg(feature = "csr")]
mod feat_csr {
    use super::*;

    pub(crate) fn push_component_rendered<F>(component_id: usize, rendered: F, first_render: bool)
    where
        F: FnOnce() + 'static,
    {
        with(|s| {
            if first_render {
                s.rendered_first.push(component_id, Box::new(rendered));
            } else {
                s.rendered.push(component_id, Box::new(rendered));
            }
        });
    }

    pub(crate) fn push_component_props_update<F>(props_update: F)
    where
        F: FnOnce() + 'static,
    {
        with(|s| s.props_update.push(Box::new(props_update)));
    }
}

#[cfg(feature = "csr")]
pub(crate) use feat_csr::*;

#[cfg(feature = "hydration")]
mod feat_hydration {
    use super::*;

    pub(crate) fn push_component_priority_render<F>(component_id: usize, render: F)
    where
        F: FnOnce() + 'static,
    {
        with(|s| {
            s.render_priority.push(component_id, Box::new(render));
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
    fn fill_queue(&mut self, to_run: &mut Vec<Runnable>) {
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

        push(|| FLAG.with(|v| v.set(true)));
        FLAG.with(|v| assert!(v.get()));
    }
}
