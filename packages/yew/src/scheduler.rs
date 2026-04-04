//! This module contains a scheduler.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
#[cfg(any(test, feature = "test"))]
mod flush_wakers {
    use std::cell::RefCell;
    use std::task::Waker;

    thread_local! {
        static FLUSH_WAKERS: RefCell<Vec<Waker>> = const { RefCell::new(Vec::new()) };
    }

    #[cfg(all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(feature = "not_browser_env")
    ))]
    pub(super) fn register(waker: Waker) {
        FLUSH_WAKERS.with(|w| {
            w.borrow_mut().push(waker);
        });
    }

    pub(super) fn wake_all() {
        FLUSH_WAKERS.with(|w| {
            for waker in w.borrow_mut().drain(..) {
                waker.wake();
            }
        });
    }
}

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
    const fn new() -> Self {
        Self { inner: Vec::new() }
    }

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
    const fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    #[cfg(any(feature = "ssr", feature = "csr"))]
    fn push(&mut self, component_id: usize, task: Box<dyn Runnable>) {
        self.inner.insert(component_id, QueueEntry { task });
    }

    /// Take a single entry, preferring parents over children
    #[inline]
    fn pop_topmost(&mut self) -> Option<QueueEntry> {
        self.inner.pop_first().map(|(_, v)| v)
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

impl Scheduler {
    const fn new() -> Self {
        Self {
            main: FifoQueue::new(),
            destroy: FifoQueue::new(),
            create: FifoQueue::new(),
            props_update: FifoQueue::new(),
            update: FifoQueue::new(),
            render: TopologicalQueue::new(),
            render_first: TopologicalQueue::new(),
            render_priority: TopologicalQueue::new(),
            rendered_first: TopologicalQueue::new(),
            rendered: TopologicalQueue::new(),
        }
    }
}

/// Execute closure with a mutable reference to the scheduler
#[inline]
fn with<R>(f: impl FnOnce(&mut Scheduler) -> R) -> R {
    thread_local! {
        /// This is a global scheduler suitable to schedule and run any tasks.
        ///
        /// Exclusivity of mutable access is controlled by only accessing it through a set of public
        /// functions.
        static SCHEDULER: RefCell<Scheduler> = const { RefCell::new(Scheduler::new()) };
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
        static LOCK: RefCell<()> = const { RefCell::new(()) };
    }

    LOCK.with(|l| {
        if let Ok(_lock) = l.try_borrow_mut() {
            scheduler_loop();
            #[cfg(any(test, feature = "test"))]
            flush_wakers::wake_all();
        }
    });
}

#[cfg(all(
    target_arch = "wasm32",
    not(target_os = "wasi"),
    not(feature = "not_browser_env")
))]
mod arch {
    use std::sync::atomic::{AtomicBool, Ordering};

    use wasm_bindgen::prelude::*;

    use crate::platform::spawn_local;

    // Really only used as a `Cell<bool>` that is also `Sync`
    static IS_SCHEDULED: AtomicBool = AtomicBool::new(false);
    fn check_scheduled() -> bool {
        // Since we can tolerate starting too many times, and we don't need to "see" any stores
        // done in the scheduler, Relaxed ordering is fine
        IS_SCHEDULED.load(Ordering::Relaxed)
    }
    fn set_scheduled(is: bool) {
        // See comment in check_scheduled why Relaxed ordering is fine
        IS_SCHEDULED.store(is, Ordering::Relaxed)
    }

    #[cfg(any(test, feature = "test"))]
    pub(super) fn is_scheduled() -> bool {
        check_scheduled()
    }

    const YIELD_DEADLINE_MS: f64 = 16.0;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = setTimeout)]
        fn set_timeout(handler: &js_sys::Function, timeout: i32) -> i32;
    }

    fn run_scheduler(mut queue: Vec<super::QueueEntry>) {
        let deadline = js_sys::Date::now() + YIELD_DEADLINE_MS;

        loop {
            super::with(|s| s.fill_queue(&mut queue));
            if queue.is_empty() {
                break;
            }
            for r in queue.drain(..) {
                r.task.run();
            }
            if js_sys::Date::now() >= deadline {
                // Only yield when no DOM-mutating work is pending, so event
                // handlers that fire during the yield see a consistent DOM.
                let can_yield = super::with(|s| s.can_yield());
                if can_yield {
                    let cb = Closure::once_into_js(move || run_scheduler(queue));
                    set_timeout(cb.unchecked_ref(), 0);
                    return;
                }
            }
        }

        set_scheduled(false);
        #[cfg(any(test, feature = "test"))]
        super::flush_wakers::wake_all();
    }

    /// We delay the start of the scheduler to the end of the micro task queue.
    /// So any messages that needs to be queued can be queued.
    /// Once running, we yield to the browser every ~16ms, but only at points
    /// where the DOM is in a consistent state (no pending renders/destroys).
    pub(crate) fn start() {
        if check_scheduled() {
            return;
        }
        set_scheduled(true);
        spawn_local(async {
            run_scheduler(vec![]);
        });
    }
}

#[cfg(any(
    not(target_arch = "wasm32"),
    target_os = "wasi",
    feature = "not_browser_env"
))]
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

/// Flush all pending scheduler work, ensuring all rendering and lifecycle callbacks complete.
///
/// On browser WebAssembly targets, the scheduler defers its work to the microtask queue.
/// This function registers a waker that is notified when `start_now()` finishes draining all
/// queues, providing proper event-driven render-complete notification without arbitrary sleeps.
///
/// On non-browser targets, the scheduler runs synchronously so this simply drains pending work.
///
/// Use this in tests after mounting or updating a component to ensure all rendering has
/// completed before making assertions.
#[cfg(all(
    any(test, feature = "test"),
    target_arch = "wasm32",
    not(target_os = "wasi"),
    not(feature = "not_browser_env")
))]
pub async fn flush() {
    std::future::poll_fn(|cx| {
        start_now();

        if arch::is_scheduled() {
            flush_wakers::register(cx.waker().clone());
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(())
        }
    })
    .await
}

/// Flush all pending scheduler work, ensuring all rendering and lifecycle callbacks complete.
///
/// On non-browser targets, the scheduler runs synchronously so this simply drains pending work.
#[cfg(all(
    any(test, feature = "test"),
    not(all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(feature = "not_browser_env")
    ))
))]
pub async fn flush() {
    start_now();
}

impl Scheduler {
    /// Returns true when no DOM-mutating work is pending, meaning it's safe to
    /// yield to the browser without leaving the DOM in an inconsistent state.
    #[cfg(all(
        target_arch = "wasm32",
        not(target_os = "wasi"),
        not(feature = "not_browser_env")
    ))]
    fn can_yield(&self) -> bool {
        self.destroy.inner.is_empty()
            && self.create.inner.is_empty()
            && self.render_first.inner.is_empty()
            && self.render.inner.is_empty()
            && self.render_priority.inner.is_empty()
    }

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
        // This is needed for hydration subsequent render to fix node refs.
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
            static FLAG: Cell<bool> = const { Cell::new(false) };
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
