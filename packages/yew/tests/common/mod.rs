#![allow(dead_code)]
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use slab::Slab;
use yew::html::ImplicitClone;
use yew::suspense::{Suspension, SuspensionHandle, SuspensionResult};
use yew::{hook, use_reducer, use_state, Reducible};

#[track_caller]
pub fn obtain_result() -> String {
    gloo_utils::document()
        .get_element_by_id("result")
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

#[track_caller]
pub fn obtain_result_by_id(id: &str) -> String {
    gloo_utils::document()
        .get_element_by_id(id)
        .expect("No result found. Most likely, the application crashed and burned")
        .inner_html()
}

#[derive(Clone)]
pub struct TriggerBus(Rc<(Cell<bool>, RefCell<Slab<Box<dyn FnMut(bool)>>>)>);

impl TriggerBus {
    pub fn new() -> Self {
        Self(Rc::new((Cell::new(false), RefCell::new(Slab::new()))))
    }

    pub fn activate(&self) {
        let this = &self.0;
        if !this.0.get() {
            this.0.set(true); // no race problem
            let mut reg = this.1.borrow_mut();
            for (_, t) in reg.iter_mut() {
                t(true);
            }
        }
    }

    pub fn deactivate(&self) {
        let this = &self.0;
        if this.0.get() {
            this.0.set(false); // no race problem
            let mut reg = this.1.borrow_mut();
            for (_, t) in reg.iter_mut() {
                t(false);
            }
        }
    }

    fn read(&self) -> bool {
        self.0 .0.get()
    }

    fn subscribe(&self, trigger: Box<dyn FnMut(bool)>) -> usize {
        self.0 .1.borrow_mut().insert(trigger)
    }

    fn unsubscribe(&self, id: usize) {
        let _ = self.0 .1.borrow_mut().remove(id);
    }
}

impl Future for TriggerBus {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.read() {
            Poll::Ready(())
        } else {
            let mut waker = Some(cx.waker().clone());
            self.subscribe(Box::new(move |b: bool| {
                if !b {
                    return;
                }
                if let Some(w) = waker.take() {
                    w.wake()
                }
            }));
            // soft todo: bother to unsubscribe
            Poll::Pending
        }
    }
}

impl PartialEq for TriggerBus {
    fn eq(&self, _: &Self) -> bool {
        true // Pretend, don't cause re-renders
    }
}

impl ImplicitClone for TriggerBus {}

#[hook]
pub fn use_trigger(bus: &TriggerBus) -> SuspensionResult<Box<dyn Fn()>> {
    struct BusTracker {
        suspension: RefCell<Option<(Suspension, SuspensionHandle)>>,
    }
    impl Reducible for BusTracker {
        type Action = bool;

        fn reduce(self: Rc<Self>, action: bool) -> Rc<Self> {
            match action {
                true => {
                    if let Some(susp) = self.suspension.borrow_mut().take() {
                        susp.1.resume()
                    }
                }
                false => *self.suspension.borrow_mut() = Some(Suspension::new()),
            }
            self
        }
    }
    struct BusSubscriber {
        id: usize,
        bus: TriggerBus,
    }
    impl Drop for BusSubscriber {
        fn drop(&mut self) {
            self.bus.unsubscribe(self.id)
        }
    }
    let sleep_state = use_reducer(|| BusTracker {
        suspension: RefCell::new((!bus.read()).then(Suspension::new)),
    });
    let _ = {
        let sleep_state = sleep_state.dispatcher();
        let bus = bus.clone();
        use_state(move || {
            let id = bus.subscribe(Box::new(move |b| sleep_state.dispatch(b)));
            BusSubscriber { id, bus }
        })
    };

    let state = (*sleep_state).suspension.borrow();
    match &*state {
        Some(ref state) if !state.0.resumed() => Err(state.0.clone()),
        _ => {
            let bus = bus.clone();
            Ok(Box::new(move || bus.deactivate()))
        }
    }
}
