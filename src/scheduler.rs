use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use slab::Slab;

pub type RunnableIndex = usize;

// TODO 1) Could be replaced with a struct (not a closure)
// TODO 2) Use context here withount a refcounter!
//pub type Runnable<CTX> = Box<for<'a> FnMut(&'a mut CTX) + 'static>;

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

/// This is a global scheduler suitable to schedule and run any tasks.
pub struct Scheduler<CTX> {
    context: Rc<RefCell<CTX>>,
    slab: Rc<RefCell<Slab<Rc<RefCell<Runnable<CTX>>>>>>,
    sequence: Rc<RefCell<VecDeque<RunnableIndex>>>,
}

impl<CTX> Clone for Scheduler<CTX> {
    fn clone(&self) -> Self {
        Scheduler {
            context: self.context.clone(),
            slab: self.slab.clone(),
            sequence: self.sequence.clone(),
        }
    }
}

impl<CTX> Scheduler<CTX> {
    pub fn new(context: CTX) -> Self {
        Scheduler {
            context: Rc::new(RefCell::new(context)),
            slab: Rc::new(RefCell::new(Slab::new())),
            sequence: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn register(&mut self, runnable: Runnable<CTX>) -> RunnableIndex {
        let runnable = Rc::new(RefCell::new(runnable));
        self.slab.try_borrow_mut()
            .expect("can't borrow slab to register a runnable")
            .insert(runnable)
    }

    pub fn unregister(&mut self, index: RunnableIndex) -> Runnable<CTX> {
        let rc = self.slab.try_borrow_mut()
            .expect("can't borrow slab to unregister a runnable")
            .remove(index);
        Rc::try_unwrap(rc).ok().expect("unregistered slot was empty").into_inner()
    }

    pub fn put_and_try_run(&mut self, index: RunnableIndex) {
        assert!(self.slab.borrow().contains(index));
        self.sequence.try_borrow_mut()
            .expect("can't borrow a sequence to push a new index to run")
            .push_back(index);
        // If context not locked we should start the processing over it
        if let Ok(ref mut context) = self.context.try_borrow_mut() {
            loop {
                let the_next = self.sequence.try_borrow_mut()
                    .expect("can't borrow a sequence to take an index of runnable")
                    .pop_front();
                if let Some(idx) = the_next {
                    let routine = self.slab.try_borrow()
                        .expect("can't borrow slab to get a reference to a routine")
                        .get(idx)
                        .cloned()
                        .expect("routine not registered");
                    routine.try_borrow_mut()
                        .expect("can't borrow a routine to run")
                        .run(context);
                } else {
                    break;
                }
            }
        }
    }
}
