use std::rc::Rc;

use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(PartialEq)]
pub struct SleepState {
    s: Suspension,
}

impl SleepState {
    fn new() -> Self {
        let (s, handle) = Suspension::new();

        spawn_local(async move {
            TimeoutFuture::new(5_000).await;

            handle.resume();
        });

        Self { s }
    }
}

impl Reducible for SleepState {
    type Action = ();

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        Self::new().into()
    }
}

pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
    let sleep_state = use_reducer(SleepState::new);

    if sleep_state.s.resumed() {
        Ok(Rc::new(move || sleep_state.dispatch(())))
    } else {
        Err(sleep_state.s.clone())
    }
}
