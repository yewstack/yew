use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use crate::*;

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
    /// Send a message to an worker.
    pub fn send(&self, msg: T::Input) {
        let mut bridge = self.inner.borrow_mut();
        bridge.send(msg);
    }
}

/// A hook to bridge to an [`Worker`].
///
/// This hooks will only bridge the worker once over the entire component lifecycle.
///
/// Takes a callback as the only argument. The callback will be updated on every render to make
/// sure captured values (if any) are up to date.
///
/// # Examples
///
/// ```
/// # mod example {
/// use serde::{Deserialize, Serialize};
/// use yew::prelude::*;
/// use yew_agent::{use_bridge, UseBridgeHandle};
///
/// // This would usually live in the same file as your worker
/// #[derive(Serialize, Deserialize)]
/// pub enum WorkerResponseType {
///     IncrementCounter,
/// }
/// # mod my_worker_mod {
/// #   use yew_agent::{HandlerId, Public, WorkerLink};
/// #   use super::WorkerResponseType;
/// #   pub struct MyWorker {
/// #       pub link: WorkerLink<Self>,
/// #   }
///
/// #   impl yew_agent::Worker for MyWorker {
/// #       type Input = ();
/// #       type Output = WorkerResponseType;
/// #       type Reach = Public<Self>;
/// #       type Message = ();
/// #
/// #       fn create(link: WorkerLink<Self>) -> Self {
/// #           MyWorker { link }
/// #       }
/// #
/// #       fn update(&mut self, _msg: Self::Message) {
/// #           // do nothing
/// #       }
/// #
/// #       fn handle_input(&mut self, _msg: Self::Input, id: HandlerId) {
/// #           self.link.respond(id, WorkerResponseType::IncrementCounter);
/// #       }
/// #   }
/// # }
/// use my_worker_mod::MyWorker; // note that <MyWorker as yew_agent::Worker>::Output == WorkerResponseType
/// #[function_component(UseBridge)]
/// fn bridge() -> Html {
///     let counter = use_state(|| 0);
///
///     // a scoped block to clone the state in
///     {
///         let counter = counter.clone();
///         // response will be of type MyWorker::Output, i.e. WorkerResponseType
///         let bridge: UseBridgeHandle<MyWorker> = use_bridge(move |response| match response {
///             WorkerResponseType::IncrementCounter => {
///                 counter.set(*counter + 1);
///             }
///         });
///     }
///
///     html! {
///         <div>
///             {*counter}
///         </div>
///     }
/// }
/// # }
/// ```
#[hook]
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
        T::bridge({
            Rc::new(move |output| {
                let on_output = on_output_ref.borrow().clone();
                on_output(output);
            })
        })
    });

    UseBridgeHandle { inner: bridge }
}

impl<T: Worker> Clone for UseBridgeHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
