use std::rc::Rc;
use std::cell::RefCell;
use slab::Slab;

pub type RunnableIndex = usize;

// TODO 1) Could be replaced with a struct (not a closure)
// TODO 2) Use context here withount a refcounter!
//pub type Runnable<CTX> = Box<for<'a> FnMut(&'a mut CTX) + 'static>;

pub type Runnable<CTX> = Box<BeRunnable<CTX>>;

pub trait BeRunnable<CTX> {
    //fn run<'a>(&mut self, context: &'a mut CTX);
}

impl<'a, T, CTX> BeRunnable<CTX> for T
where
    T: FnMut(&'a mut CTX),
    CTX: 'static,
{
    /*
    fn run(&mut self, context: &mut CTX) {
        self(context);
    }
    */
}

pub type SharedScheduler<CTX> = Rc<RefCell<Scheduler<CTX>>>;

/// This is a global scheduler suitable to schedule and run any tasks.
pub struct Scheduler<CTX> {
    context: CTX,
    slab: Slab<Runnable<CTX>>,
    sequence: Vec<RunnableIndex>,
}

impl<CTX> Scheduler<CTX> {
    pub fn new(context: CTX) -> Self {
        Scheduler {
            context,
            slab: Slab::new(),
            sequence: Vec::new(),
        }
    }

    pub fn register(&mut self, runnable: Runnable<CTX>) -> RunnableIndex {
        self.slab.insert(runnable)
    }

    pub fn unregister(&mut self, index: RunnableIndex) -> Runnable<CTX> {
        self.slab.remove(index)
    }

    pub fn schedule(&mut self, index: RunnableIndex) {
        assert!(self.slab.contains(index));
        self.sequence.push(index);
    }
}
