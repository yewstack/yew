//! This module contains the worker agent implementation.
//!
//! This is a low-level implementation that wraps gloo-worker.
//!
//! # Example
//!
//! ```
//! # mod example {
//! use serde::{Deserialize, Serialize};
//! use yew::prelude::*;
//! use yew_agent::{use_bridge, UseBridgeHandle};
//!
//! // This would usually live in the same file as your worker
//! #[derive(Serialize, Deserialize)]
//! pub enum WorkerResponseType {
//!     IncrementCounter,
//! }
//! # mod my_worker_mod {
//! #   use yew_agent::{HandlerId, Public, WorkerLink};
//! #   use super::WorkerResponseType;
//! #   pub struct MyWorker {
//! #       pub link: WorkerLink<Self>,
//! #   }
//!
//! #   impl yew_agent::Worker for MyWorker {
//! #       type Input = ();
//! #       type Output = WorkerResponseType;
//! #       type Reach = Public<Self>;
//! #       type Message = ();
//! #
//! #       fn create(link: WorkerLink<Self>) -> Self {
//! #           MyWorker { link }
//! #       }
//! #
//! #       fn update(&mut self, _msg: Self::Message) {
//! #           // do nothing
//! #       }
//! #
//! #       fn handle_input(&mut self, _msg: Self::Input, id: HandlerId) {
//! #           self.link.respond(id, WorkerResponseType::IncrementCounter);
//! #       }
//! #   }
//! # }
//! use my_worker_mod::MyWorker; // note that <MyWorker as yew_agent::Worker>::Output == WorkerResponseType
//! #[function_component(UseBridge)]
//! fn bridge() -> Html {
//!     let counter = use_state(|| 0);
//!
//!     // a scoped block to clone the state in
//!     {
//!         let counter = counter.clone();
//!         // response will be of type MyWorker::Output, i.e. WorkerResponseType
//!         let bridge: UseBridgeHandle<MyWorker> = use_bridge(move |response| match response {
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

mod hooks;
mod provider;

pub(crate) use gloo_worker::WorkerBridge;
#[doc(inline)]
pub use gloo_worker::{HandlerId, Worker, WorkerDestroyHandle, WorkerRegistrar, WorkerScope};
pub use hooks::{
    use_worker_bridge, use_worker_subscription, UseWorkerBridgeHandle, UseWorkerSubscriptionHandle,
};
pub(crate) use provider::WorkerProviderState;
pub use provider::{WorkerProvider, WorkerProviderProps};
