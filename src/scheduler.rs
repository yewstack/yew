use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use slab::Slab;

pub type RunnableIndex = usize;

// TODO 1) Could be replaced with a struct (not a closure)

pub type Runnable<CTX> = Box<BeRunnable<CTX>>;

pub trait BeRunnable<CTX> {
    fn run<'a>(&mut self, context: &'a mut CTX);
}

impl<T, CTX> BeRunnable<CTX> for T
where
    T: FnMut(&mut CTX),
{
    fn run<'a>(&mut self, context: &'a mut CTX) {
        self(context);
    }
}

struct Pool<CTX> {
    slab: Slab<Rc<RefCell<Runnable<CTX>>>>,
    sequence: VecDeque<RunnableIndex>,
}

impl<CTX> Pool<CTX> {
    fn register(&mut self, runnable: Runnable<CTX>) -> RunnableIndex {
        let runnable = Rc::new(RefCell::new(runnable));
        self.slab.insert(runnable)
    }

    fn put(&mut self, index: RunnableIndex) {
        self.sequence.push_back(index);
    }

    fn next(&mut self) -> Option<Rc<RefCell<Runnable<CTX>>>> {
        self.sequence.pop_front().and_then(|idx| {
            self.slab.get(idx).cloned()
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

    pub(crate) fn register<F>(&mut self, closure: F) -> RunnableIndex
    where
        F: FnMut(&mut CTX) + 'static,
    {
        let runnable: Runnable<CTX> = Box::new(closure);
        self.pool.try_borrow_mut()
            .expect("can't borrow slab to register a runnable")
            .register(runnable)
    }

    pub(crate) fn unregister(&mut self, index: RunnableIndex) -> Runnable<CTX> {
        unimplemented!();
    }

    pub(crate) fn put_and_try_run(&mut self, index: RunnableIndex) {
        self.pool.borrow_mut().put(index);
        // Context lock also means the loop is runnging
        if let Ok(ref mut context) = self.context.try_borrow_mut() {
            loop {
                let do_next = self.pool.borrow_mut().next();
                if let Some(routine) = do_next {
                    routine.borrow_mut().run(context);
                } else {
                    break;
                }
            }
        }
    }
}
