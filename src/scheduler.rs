//! This module contains a scheduler.

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use slab::Slab;

/// Id of a runnable instance.
pub(crate) type RunnableIndex = usize;

/// The flag that means the routine should be destroyed.
pub(crate) type WillDestroy = bool;

/// Unspecified routine binded to a context.
pub(crate) type BoxedRunnable<CTX> = Box<Runnable<CTX>>;

/// A routine which could be run.
pub(crate) trait Runnable<CTX> {
    /// Runs a routine with a context instance.
    fn run<'a>(&mut self, context: &'a mut CTX) -> WillDestroy;
}

/// The `Pool` which keep a sequence of runnables to start next.
struct Pool<CTX> {
    slab: Slab<Rc<RefCell<BoxedRunnable<CTX>>>>,
    sequence: VecDeque<RunnableIndex>,
}

impl<CTX> Pool<CTX> {
    /// Put a runnable and return a unique id.
    fn register(&mut self, runnable: BoxedRunnable<CTX>) -> RunnableIndex {
        let runnable = Rc::new(RefCell::new(runnable));
        self.slab.insert(runnable)
    }

    fn unregister(&mut self, index: RunnableIndex) -> BoxedRunnable<CTX> {
        let runnable = self.slab.remove(index);
        Rc::try_unwrap(runnable).ok()
            .expect("runnable was locked")
            .into_inner()
    }

    fn put(&mut self, index: RunnableIndex) {
        self.sequence.push_back(index);
    }

    fn next(&mut self) -> Option<(RunnableIndex, Rc<RefCell<BoxedRunnable<CTX>>>)> {
        self.sequence.pop_front().and_then(|idx| {
            self.slab.get(idx).cloned().map(|runnable| (idx, runnable))
        })
    }
}

/// This is a global scheduler suitable to schedule and run any tasks.
pub struct Scheduler<CTX> {
    context: Rc<RefCell<CTX>>,
    pool: Rc<RefCell<Pool<CTX>>>,
}

impl<CTX> Clone for Scheduler<CTX> {
    fn clone(&self) -> Self {
        Scheduler {
            context: self.context.clone(),
            pool: self.pool.clone(),
        }
    }
}

impl<CTX> Scheduler<CTX> {
    /// Creates a new scheduler with a context.
    pub fn new(context: CTX) -> Self {
        let pool = Pool {
            slab: Slab::new(),
            sequence: VecDeque::new(),
        };
        Scheduler {
            context: Rc::new(RefCell::new(context)),
            pool: Rc::new(RefCell::new(pool)),
        }
    }

    pub(crate) fn register<T>(&mut self, runnable: T) -> RunnableIndex
    where
        T: Runnable<CTX> + 'static,
    {
        let runnable: BoxedRunnable<CTX> = Box::new(runnable);
        self.pool.try_borrow_mut()
            .expect("can't borrow slab to register a runnable")
            .register(runnable)
    }

    pub(crate) fn put_and_try_run(&mut self, index: RunnableIndex) {
        self.pool.borrow_mut().put(index);
        // Context lock also means the loop is runnging
        let mut unreg = Vec::new();
        if let Ok(ref mut context) = self.context.try_borrow_mut() {
            loop {
                let do_next = self.pool.borrow_mut().next();
                if let Some((idx, routine)) = do_next {
                    let will_destroy = routine.borrow_mut().run(context);
                    if will_destroy {
                        // TODO Filter deque (remove items with this id)
                        // because they must not be called and after
                        // the routine removed new call won't added with this id
                        // even if callback still exists
                        unreg.push(idx);
                    }
                } else {
                    break;
                }
            }
        }
        // Remove unnecessary routines only when loop finished completely,
        // because they could call each other
        for idx in unreg.into_iter() {
            self.pool.try_borrow_mut()
                .expect("can't borrow slab to unregister a runnable")
                .unregister(idx);
        }
    }
}
