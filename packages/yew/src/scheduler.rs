//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::BTreeMap;
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

    props_update: Vec<Box<dyn Runnable>>,
    update: Vec<Box<dyn Runnable>>,

    /// The Binary Tree Map guarantees components with lower id (parent) is rendered first and
    /// no more than 1 render can be scheduled before a component is rendered.
    ///
    /// Parent can destroy child components but not otherwise, we can save unnecessary render by
    /// rendering parent first.
    render: BTreeMap<usize, Box<dyn Runnable>>,
    render_first: BTreeMap<usize, Box<dyn Runnable>>,
    render_priority: BTreeMap<usize, Box<dyn Runnable>>,

    /// Binary Tree Map to guarantee children rendered are always called before parent calls
    rendered_first: BTreeMap<usize, Box<dyn Runnable>>,
    rendered: BTreeMap<usize, Box<dyn Runnable>>,
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
            s.render_first.insert(component_id, first_render);
        });
    }

    /// Push a component destruction [Runnable] to be executed
    pub(crate) fn push_component_destroy(runnable: Box<dyn Runnable>) {
        with(|s| s.destroy.push(runnable));
    }

    /// Push a component render [Runnable]s to be executed
    pub(crate) fn push_component_render(component_id: usize, render: Box<dyn Runnable>) {
        with(|s| {
            s.render.insert(component_id, render);
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
                s.rendered_first.insert(component_id, rendered);
            } else {
                s.rendered.insert(component_id, rendered);
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
            s.render_priority.insert(component_id, render);
        });
    }
}

#[cfg(feature = "hydration")]
pub(crate) use feat_hydration::*;

/// Execute any pending [Runnable]s
pub(crate) fn start_now() {
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
    fn fill_queue(&mut self, to_run: &mut Vec<Box<dyn Runnable>>) {
        // Placed first to avoid as much needless work as possible, handling all the other events.
        // Drained completely, because they are the highest priority events anyway.
        to_run.append(&mut self.destroy);

        // Create events can be batched, as they are typically just for object creation
        to_run.append(&mut self.create);

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
        //
        // To be replaced with BTreeMap::pop_first once it is stable.
        if let Some(r) = self
            .render_first
            .keys()
            .next()
            .cloned()
            .and_then(|m| self.render_first.remove(&m))
        {
            to_run.push(r);
        }

        if !to_run.is_empty() {
            return;
        }

        to_run.append(&mut self.props_update);

        // Priority rendering
        //
        // This is needed for hydration susequent render to fix node refs.
        if let Some(r) = self
            .render_priority
            .keys()
            .next()
            .cloned()
            .and_then(|m| self.render_priority.remove(&m))
        {
            to_run.push(r);
            return;
        }

        if !self.rendered_first.is_empty() {
            let rendered_first = std::mem::take(&mut self.rendered_first);
            // Children rendered lifecycle happen before parents.
            to_run.extend(rendered_first.into_values().rev());
        }

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

        // To be replaced with BTreeMap::pop_first once it is stable.
        // Should be processed one at time, because they can spawn more create and rendered events
        // for their children.
        if let Some(r) = self
            .render
            .keys()
            .next()
            .cloned()
            .and_then(|m| self.render.remove(&m))
        {
            to_run.push(r);
        }

        // These typically do nothing and don't spawn any other events - can be batched.
        // Should be run only after all renders have finished.
        if !to_run.is_empty() {
            return;
        }

        if !self.rendered.is_empty() {
            let rendered = std::mem::take(&mut self.rendered);
            // Children rendered lifecycle happen before parents.
            to_run.extend(rendered.into_values().rev());
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
