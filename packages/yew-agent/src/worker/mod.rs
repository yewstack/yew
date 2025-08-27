//! This module contains the worker agent implementation.
//!
//! This is a low-level implementation that uses an actor model.
//!
//! # Example
//!
//! ```
//! # mod example {
//! use serde::{Deserialize, Serialize};
//! use yew::prelude::*;
//! use yew_agent::worker::{use_worker_bridge, UseWorkerBridgeHandle};
//!
//! // This would usually live in the same file as your worker
//! #[derive(Serialize, Deserialize)]
//! pub enum WorkerResponseType {
//!     IncrementCounter,
//! }
//! # mod my_worker_mod {
//! #   use yew_agent::worker::{HandlerId, WorkerScope};
//! #   use super::WorkerResponseType;
//! #   pub struct MyWorker {}
//! #
//! #   impl yew_agent::worker::Worker for MyWorker {
//! #       type Input = ();
//! #       type Output = WorkerResponseType;
//! #       type Message = ();
//! #
//! #       fn create(scope: &WorkerScope<Self>) -> Self {
//! #           MyWorker {}
//! #       }
//! #
//! #       fn update(&mut self, scope: &WorkerScope<Self>, _msg: Self::Message) {
//! #           // do nothing
//! #       }
//! #
//! #       fn received(&mut self, scope: &WorkerScope<Self>, _msg: Self::Input, id: HandlerId) {
//! #           scope.respond(id, WorkerResponseType::IncrementCounter);
//! #       }
//! #   }
//! # }
//! use my_worker_mod::MyWorker; // note that <MyWorker as yew_agent::Worker>::Output == WorkerResponseType
//! #[component(UseWorkerBridge)]
//! fn bridge() -> Html {
//!     let counter = use_state(|| 0);
//!
//!     // a scoped block to clone the state in
//!     {
//!         let counter = counter.clone();
//!         // response will be of type MyWorker::Output, i.e. WorkerResponseType
//!         let bridge: UseWorkerBridgeHandle<MyWorker> = use_worker_bridge(move |response| match response {
//!             WorkerResponseType::IncrementCounter => {
//!                 counter.set(*counter + 1);
//!             }
//!         });
//!     }
//!
//!     html! {
//!         <div>
//!             {*counter}
//!         </div>
//!     }
//! }
//! # }
//! ```

mod bridge;
mod handler_id;
mod hooks;
mod lifecycle;
mod messages;
mod native_worker;
mod provider;
mod registrar;
mod scope;
mod spawner;
mod traits;

use std::cell::RefCell;
use std::rc::Rc;

pub use bridge::WorkerBridge;
pub use handler_id::HandlerId;
pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub(crate) use provider::WorkerProviderState;
pub use provider::{WorkerProvider, WorkerProviderProps};
pub use registrar::WorkerRegistrar;
pub use scope::{WorkerDestroyHandle, WorkerScope};
pub use spawner::WorkerSpawner;
pub use traits::Worker;

/// Alias for `Rc<RefCell<T>>`
type Shared<T> = Rc<RefCell<T>>;

/// Alias for `Rc<dyn Fn(IN)>`
type Callback<IN> = Rc<dyn Fn(IN)>;
