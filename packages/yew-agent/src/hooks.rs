use std::cell::RefCell;
use std::rc::Rc;

use crate::*;
use yew::prelude::*;

/// State handle for [`use_bridge`] hook
pub struct UseBridgeHandle<T>
where
    T: Bridged,
{
    inner: Rc<RefCell<Box<dyn Bridge<T>>>>,
}

impl<T> UseBridgeHandle<T>
where
    T: Bridged,
{
    /// Send a message to an agent.
    pub fn send(&self, msg: T::Input) {
        let mut bridge = self.inner.borrow_mut();
        bridge.send(msg);
    }
}

/// A hook to bridge to an Agent.
///
/// This hooks will only bridge the agent once over the entire component lifecycle.
///
/// Takes a callback as the only argument. The callback will be updated on every render to make
/// sure captured values (if any) are up to date.
pub fn use_bridge<T, F>(on_output: F) -> UseBridgeHandle<T>
where
    T: Bridged,
    F: Fn(T::Output) + 'static,
{
    let on_output = Rc::new(on_output);

    let on_output_clone = on_output.clone();
    let on_output_ref = use_mut_ref(move || on_output_clone);

    // Refresh the callback on every render.
    {
        let mut on_output_ref = on_output_ref.borrow_mut();
        *on_output_ref = on_output;
    }

    let bridge = use_mut_ref(move || {
        T::bridge(Callback::from(move |output| {
            let on_output = on_output_ref.borrow().clone();
            on_output(output);
        }))
    });

    UseBridgeHandle { inner: bridge }
}

impl<T: Agent> Clone for UseBridgeHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
