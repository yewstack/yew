use std::rc::Rc;
use std::time::Duration;

use gloo::timers::future::sleep;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

#[derive(PartialEq)]
pub struct SleepState {
    s: Suspension,
}

impl SleepState {
    fn new() -> Self {
        let s = Suspension::from_future(async {
            sleep(Duration::from_secs(5)).await;
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

#[hook]
pub fn use_sleep() -> SuspensionResult<Rc<dyn Fn()>> {
    let sleep_state = use_reducer(SleepState::new);

    if sleep_state.s.resumed() {
        Ok(Rc::new(move || sleep_state.dispatch(())))
    } else {
        Err(sleep_state.s.clone())
    }
}
