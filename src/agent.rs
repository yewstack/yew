//! This module contains types to support multi-threading in Yew.

use crate::callback::Callback;
use crate::scheduler::{scheduler, Runnable, Shared};
use anymap::{self, AnyMap};
use bincode;
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use log::warn;
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap, HashSet};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use crate::utils;
        use js_sys::{Array, Reflect, Uint8Array};
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
        use web_sys::{Blob, BlobPropertyBag, DedicatedWorkerGlobalScope, MessageEvent, Url, Worker, WorkerOptions};
    }
}

/// Serializable messages to worker
#[derive(Serialize, Deserialize, Debug)]
enum ToWorker<T> {
    /// Client is connected
    Connected(HandlerId),
    /// Incoming message to Worker
    ProcessInput(HandlerId, T),
    /// Client is disconnected
    Disconnected(HandlerId),
    /// Worker should be terminated
    Destroy,
}

/// Serializable messages sent by worker to consumer
#[derive(Serialize, Deserialize, Debug)]
enum FromWorker<T> {
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    /// Outgoing message to consumer
    ProcessOutput(HandlerId, T),
}

/// Message packager, based on serde::Serialize/Deserialize
pub trait Packed {
    /// Pack serializable message into Vec<u8>
    fn pack(&self) -> Vec<u8>;
    /// Unpack deserializable message of byte slice
    fn unpack(data: &[u8]) -> Self;
}

impl<T: Serialize + for<'de> Deserialize<'de>> Packed for T {
    fn pack(&self) -> Vec<u8> {
        bincode::serialize(&self).expect("can't serialize an agent message")
    }

    fn unpack(data: &[u8]) -> Self {
        bincode::deserialize(&data).expect("can't deserialize an agent message")
    }
}

/// Type alias to a sharable Slab that owns optional callbacks that emit messages of the type of the specified Agent.
type SharedOutputSlab<AGN> = Shared<Slab<Option<Callback<<AGN as Agent>::Output>>>>;

/// Id of responses handler.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub struct HandlerId(usize, bool);

impl HandlerId {
    fn new(id: usize, respondable: bool) -> Self {
        HandlerId(id, respondable)
    }
    fn raw_id(self) -> usize {
        self.0
    }
    /// Indicates if a handler id corresponds to callback in the Agent runtime.
    pub fn is_respondable(self) -> bool {
        self.1
    }
}

/// This trait allows registering or getting the address of a worker.
pub trait Bridged: Agent + Sized + 'static {
    /// Creates a messaging bridge between a worker and the component.
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>>;
}

/// This trait allows the creation of a dispatcher to an existing agent that will not send replies when messages are sent.
pub trait Dispatched: Agent + Sized + 'static {
    /// Creates a dispatcher to the agent that will not send messages back.
    ///
    /// # Note
    /// Dispatchers don't have `HandlerId`s and therefore `Agent::handle` will be supplied `None`
    /// for the `id` parameter, and `connected` and `disconnected` will not be called.
    ///
    /// # Important
    /// Because the Agents using Context or Public reaches use the number of existing bridges to
    /// keep track of if the agent itself should exist, creating dispatchers will not guarantee that
    /// an Agent will exist to service requests sent from Dispatchers. You **must** keep at least one
    /// bridge around if you wish to use a dispatcher. If you are using agents in a write-only manner,
    /// then it is suggested that you create a bridge that handles no-op responses as high up in the
    /// component hierarchy as possible - oftentimes the root component for simplicity's sake.
    fn dispatcher() -> Dispatcher<Self>;
}

/// A newtype around a bridge to indicate that it is distinct from a normal bridge
pub struct Dispatcher<T>(Box<dyn Bridge<T>>);

impl<T> fmt::Debug for Dispatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Dispatcher<_>")
    }
}

impl<T> Deref for Dispatcher<T> {
    type Target = dyn Bridge<T>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl<T> DerefMut for Dispatcher<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

/// Marker trait to indicate which Discoverers are able to be used with dispatchers.
pub trait Dispatchable: Discoverer {}

/// Implements rules to register a worker in a separate thread.
pub trait Threaded {
    /// Executes an agent in the current environment.
    /// Uses in `main` function of a worker.
    fn register();
}

impl<T> Threaded for T
where
    T: Agent<Reach = Public>,
{
    fn register() {
        let scope = AgentScope::<T>::new();
        let responder = WorkerResponder {};
        let link = AgentLink::connect(&scope, responder);
        let upd = AgentLifecycleEvent::Create(link);
        scope.send(upd);
        let handler = move |data: Vec<u8>| {
            let msg = ToWorker::<T::Input>::unpack(&data);
            match msg {
                ToWorker::Connected(id) => {
                    let upd = AgentLifecycleEvent::Connected(id);
                    scope.send(upd);
                }
                ToWorker::ProcessInput(id, value) => {
                    let upd = AgentLifecycleEvent::Input(value, id);
                    scope.send(upd);
                }
                ToWorker::Disconnected(id) => {
                    let upd = AgentLifecycleEvent::Disconnected(id);
                    scope.send(upd);
                }
                ToWorker::Destroy => {
                    let upd = AgentLifecycleEvent::Destroy;
                    scope.send(upd);
                    // Terminates web worker
                    cfg_match! {
                        feature = "std_web" => js! { self.close(); },
                        feature = "web_sys" => worker_self().close(),
                    };
                }
            }
        };
        let loaded: FromWorker<T::Output> = FromWorker::WorkerLoaded;
        let loaded = loaded.pack();
        cfg_match! {
            feature = "std_web" => js! {
                    var handler = @{handler};
                    self.onmessage = function(event) {
                        handler(event.data);
                    };
                    self.postMessage(@{loaded});
            },
            feature = "web_sys" => ({
                let worker = worker_self();
                worker.set_onmessage_closure(handler);
                worker.post_message_vec(loaded);
            }),
        };
    }
}

impl<T> Bridged for T
where
    T: Agent,
{
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>> {
        Self::Reach::spawn_or_join(Some(callback))
    }
}

impl<T> Dispatched for T
where
    T: Agent,
    <T as Agent>::Reach: Dispatchable,
{
    fn dispatcher() -> Dispatcher<T> {
        Dispatcher(Self::Reach::spawn_or_join::<T>(None))
    }
}

/// Determine a visibility of an agent.
#[doc(hidden)]
pub trait Discoverer {
    /// Spawns an agent and returns `Bridge` implementation.
    fn spawn_or_join<AGN: Agent>(_callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        unimplemented!(
            "The Reach type that you tried to use with this Agent does not have
Discoverer properly implemented for it yet. Please see
https://docs.rs/yew/latest/yew/agent/ for other Reach options."
        );
    }
}

/// Bridge to a specific kind of worker.
pub trait Bridge<AGN: Agent> {
    /// Send a message to an agent.
    fn send(&mut self, msg: AGN::Input);
}

// <<< SAME THREAD >>>

struct LocalAgent<AGN: Agent> {
    scope: AgentScope<AGN>,
    slab: SharedOutputSlab<AGN>,
}

type Last = bool;

impl<AGN: Agent> LocalAgent<AGN> {
    pub fn new(scope: &AgentScope<AGN>) -> Self {
        let slab = Rc::new(RefCell::new(Slab::new()));
        LocalAgent {
            scope: scope.clone(),
            slab,
        }
    }

    fn slab(&self) -> SharedOutputSlab<AGN> {
        self.slab.clone()
    }

    fn create_bridge(&mut self, callback: Option<Callback<AGN::Output>>) -> ContextBridge<AGN> {
        let respondable = callback.is_some();
        let mut slab = self.slab.borrow_mut();
        let id: usize = slab.insert(callback);
        let id = HandlerId::new(id, respondable);
        ContextBridge {
            scope: self.scope.clone(),
            id,
        }
    }

    fn remove_bridge(&mut self, bridge: &ContextBridge<AGN>) -> Last {
        let mut slab = self.slab.borrow_mut();
        let _ = slab.remove(bridge.id.raw_id());
        slab.is_empty()
    }
}

thread_local! {
    static LOCAL_AGENTS_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

/// Create a single instance in the current thread.
#[allow(missing_debug_implementations)]
pub struct Context;

impl Discoverer for Context {
    fn spawn_or_join<AGN: Agent>(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let mut scope_to_init = None;
        let bridge = LOCAL_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            match pool.entry::<LocalAgent<AGN>>() {
                anymap::Entry::Occupied(mut entry) => {
                    // TODO(#940): Insert callback!
                    entry.get_mut().create_bridge(callback)
                }
                anymap::Entry::Vacant(entry) => {
                    let scope = AgentScope::<AGN>::new();
                    let launched = LocalAgent::new(&scope);
                    let responder = SlabResponder {
                        slab: launched.slab(),
                    };
                    scope_to_init = Some((scope, responder));
                    entry.insert(launched).create_bridge(callback)
                }
            }
        });
        if let Some((scope, responder)) = scope_to_init {
            let agent_link = AgentLink::connect(&scope, responder);
            let upd = AgentLifecycleEvent::Create(agent_link);
            scope.send(upd);
        }
        let upd = AgentLifecycleEvent::Connected(bridge.id);
        bridge.scope.send(upd);
        Box::new(bridge)
    }
}

impl Dispatchable for Context {}

struct SlabResponder<AGN: Agent> {
    slab: Shared<Slab<Option<Callback<AGN::Output>>>>,
}

impl<AGN: Agent> Responder<AGN> for SlabResponder<AGN> {
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        locate_callback_and_respond::<AGN>(&self.slab, id, output);
    }
}

/// The slab contains the callback, the id is used to look up the callback,
/// and the output is the message that will be sent via the callback.
fn locate_callback_and_respond<AGN: Agent>(
    slab: &SharedOutputSlab<AGN>,
    id: HandlerId,
    output: AGN::Output,
) {
    let callback = {
        let slab = slab.borrow();
        match slab.get(id.raw_id()).cloned() {
            Some(callback) => callback,
            None => {
                warn!("Id of handler does not exist in the slab: {}.", id.raw_id());
                return;
            }
        }
    };
    match callback {
        Some(callback) => callback.emit(output),
        None => warn!("The Id of the handler: {}, while present in the slab, is not associated with a callback.", id.raw_id()),
    }
}

struct ContextBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
    id: HandlerId,
}

impl<AGN: Agent> Bridge<AGN> for ContextBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let upd = AgentLifecycleEvent::Input(msg, self.id);
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for ContextBridge<AGN> {
    fn drop(&mut self) {
        let terminate_worker = LOCAL_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            let terminate_worker = {
                if let Some(launched) = pool.get_mut::<LocalAgent<AGN>>() {
                    launched.remove_bridge(self)
                } else {
                    false
                }
            };

            if terminate_worker {
                pool.remove::<LocalAgent<AGN>>();
            }

            terminate_worker
        });

        let upd = AgentLifecycleEvent::Disconnected(self.id);
        self.scope.send(upd);

        if terminate_worker {
            let upd = AgentLifecycleEvent::Destroy;
            self.scope.send(upd);
        }
    }
}

/// Create an instance in the current thread.
#[allow(missing_debug_implementations)]
pub struct Job;

impl Discoverer for Job {
    fn spawn_or_join<AGN: Agent>(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let callback = callback.expect("Callback required for Job");
        let scope = AgentScope::<AGN>::new();
        let responder = CallbackResponder { callback };
        let agent_link = AgentLink::connect(&scope, responder);
        let upd = AgentLifecycleEvent::Create(agent_link);
        scope.send(upd);
        let upd = AgentLifecycleEvent::Connected(SINGLETON_ID);
        scope.send(upd);
        let bridge = JobBridge { scope };
        Box::new(bridge)
    }
}

const SINGLETON_ID: HandlerId = HandlerId(0, true);

struct CallbackResponder<AGN: Agent> {
    callback: Callback<AGN::Output>,
}

impl<AGN: Agent> Responder<AGN> for CallbackResponder<AGN> {
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
        self.callback.emit(output);
    }
}

struct JobBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
}

impl<AGN: Agent> Bridge<AGN> for JobBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let upd = AgentLifecycleEvent::Input(msg, SINGLETON_ID);
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for JobBridge<AGN> {
    fn drop(&mut self) {
        let upd = AgentLifecycleEvent::Disconnected(SINGLETON_ID);
        self.scope.send(upd);
        let upd = AgentLifecycleEvent::Destroy;
        self.scope.send(upd);
    }
}

// <<< SEPARATE THREAD >>>

/// Create a new instance for every bridge.
#[allow(missing_debug_implementations)]
pub struct Private;

impl Discoverer for Private {
    fn spawn_or_join<AGN: Agent>(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let callback = callback.expect("Callback required for Private agents");
        let handler = move |data: Vec<u8>| {
            let msg = FromWorker::<AGN::Output>::unpack(&data);
            match msg {
                FromWorker::WorkerLoaded => {
                    // TODO(#948): Send `Connected` message
                }
                FromWorker::ProcessOutput(id, output) => {
                    assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
                    callback.emit(output);
                }
            }
        };
        // TODO(#947): Need somethig better...
        let name_of_resource = AGN::name_of_resource();
        let worker = cfg_match! {
            feature = "std_web" => js! {
                var worker = new Worker(@{name_of_resource});
                var handler = @{handler};
                worker.onmessage = function(event) {
                    handler(event.data);
                };
                return worker;
            },
            feature = "web_sys" => ({
                let worker = worker_new(name_of_resource, AGN::is_module());
                worker.set_onmessage_closure(handler);
                worker
            }),
        };
        let bridge = PrivateBridge {
            worker,
            _agent: PhantomData,
        };
        Box::new(bridge)
    }
}

/// A connection manager for components interaction with workers.
pub struct PrivateBridge<T: Agent> {
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    _agent: PhantomData<T>,
}

impl<AGN: Agent> fmt::Debug for PrivateBridge<AGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivateBridge<_>")
    }
}

impl<AGN: Agent> Bridge<AGN> for PrivateBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        // TODO(#937): Important! Implement.
        // Use a queue to collect a messages if an instance is not ready
        // and send them to an agent when it will reported readiness.
        let msg = ToWorker::ProcessInput(SINGLETON_ID, msg).pack();
        cfg_match! {
            feature = "std_web" => ({
                let worker = &self.worker;
                js! {
                    var worker = @{worker};
                    var bytes = @{msg};
                    worker.postMessage(bytes);
                };
            }),
            feature = "web_sys" => self.worker.post_message_vec(msg),
        }
    }
}

impl<AGN: Agent> Drop for PrivateBridge<AGN> {
    fn drop(&mut self) {
        // TODO(#946): Send `Destroy` message.
    }
}

struct RemoteAgent<AGN: Agent> {
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    slab: SharedOutputSlab<AGN>,
}

impl<AGN: Agent> RemoteAgent<AGN> {
    pub fn new(
        #[cfg(feature = "std_web")] worker: Value,
        #[cfg(feature = "web_sys")] worker: Worker,
        slab: SharedOutputSlab<AGN>,
    ) -> Self {
        RemoteAgent { worker, slab }
    }

    fn create_bridge(&mut self, callback: Option<Callback<AGN::Output>>) -> PublicBridge<AGN> {
        let respondable = callback.is_some();
        let mut slab = self.slab.borrow_mut();
        let id: usize = slab.insert(callback);
        let id = HandlerId::new(id, respondable);
        PublicBridge {
            worker: self.worker.clone(),
            id,
            _agent: PhantomData,
        }
    }

    fn remove_bridge(&mut self, bridge: &PublicBridge<AGN>) -> Last {
        let mut slab = self.slab.borrow_mut();
        let _ = slab.remove(bridge.id.raw_id());
        slab.is_empty()
    }
}

thread_local! {
    static REMOTE_AGENTS_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
    static REMOTE_AGENTS_LOADED: RefCell<HashSet<TypeId>> = RefCell::new(HashSet::new());
    static REMOTE_AGENTS_EARLY_MSGS_QUEUE: RefCell<HashMap<TypeId, Vec<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// Create a single instance in a tab.
#[allow(missing_debug_implementations)]
pub struct Public;

impl Discoverer for Public {
    fn spawn_or_join<AGN: Agent>(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let bridge = REMOTE_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            match pool.entry::<RemoteAgent<AGN>>() {
                anymap::Entry::Occupied(mut entry) => {
                    // TODO(#945): Insert callback!
                    entry.get_mut().create_bridge(callback)
                }
                anymap::Entry::Vacant(entry) => {
                    let slab: Shared<Slab<Option<Callback<AGN::Output>>>> =
                        Rc::new(RefCell::new(Slab::new()));
                    let handler = {
                        let slab = slab.clone();
                        move |data: Vec<u8>,
                              #[cfg(feature = "std_web")] worker: Value,
                              #[cfg(feature = "web_sys")] worker: &Worker| {
                            let msg = FromWorker::<AGN::Output>::unpack(&data);
                            match msg {
                                FromWorker::WorkerLoaded => {
                                    // TODO(#944): Send `Connected` message
                                    REMOTE_AGENTS_LOADED.with(|loaded| {
                                        let _ = loaded.borrow_mut().insert(TypeId::of::<AGN>());
                                    });

                                    REMOTE_AGENTS_EARLY_MSGS_QUEUE.with(|queue| {
                                        let mut queue = queue.borrow_mut();
                                        if let Some(msgs) = queue.get_mut(&TypeId::of::<AGN>()) {
                                            for msg in msgs.drain(..) {
                                                cfg_match! {
                                                    feature = "std_web" => ({
                                                        let worker = &worker;
                                                        js! {@{worker}.postMessage(@{msg});};
                                                    }),
                                                    feature = "web_sys" => worker.post_message_vec(msg),
                                                }
                                            }
                                        }
                                    });
                                }
                                FromWorker::ProcessOutput(id, output) => {
                                    locate_callback_and_respond::<AGN>(&slab, id, output);
                                }
                            }
                        }
                    };
                    let name_of_resource = AGN::name_of_resource();
                    let worker = cfg_match! {
                        feature = "std_web" => js! {
                            var worker = new Worker(@{name_of_resource});
                            var handler = @{handler};
                            worker.onmessage = function(event) {
                                handler(event.data, worker);
                            };
                            return worker;
                        },
                        feature = "web_sys" => ({
                            let worker = worker_new(name_of_resource, AGN::is_module());
                            let worker_clone = worker.clone();
                            worker.set_onmessage_closure(move |data: Vec<u8>| {
                                handler(data, &worker_clone);
                            });
                            worker
                        }),
                    };
                    let launched = RemoteAgent::new(worker, slab);
                    entry.insert(launched).create_bridge(callback)
                }
            }
        });
        Box::new(bridge)
    }
}

impl Dispatchable for Public {}

/// A connection manager for components interaction with workers.
pub struct PublicBridge<AGN: Agent> {
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    id: HandlerId,
    _agent: PhantomData<AGN>,
}

impl<AGN: Agent> fmt::Debug for PublicBridge<AGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PublicBridge<_>")
    }
}

impl<AGN: Agent> PublicBridge<AGN> {
    fn worker_is_loaded(&self) -> bool {
        REMOTE_AGENTS_LOADED.with(|loaded| loaded.borrow().contains(&TypeId::of::<AGN>()))
    }

    fn msg_to_queue(&self, msg: Vec<u8>) {
        REMOTE_AGENTS_EARLY_MSGS_QUEUE.with(|queue| {
            let mut queue = queue.borrow_mut();
            match queue.entry(TypeId::of::<AGN>()) {
                hash_map::Entry::Vacant(record) => {
                    record.insert(vec![msg]);
                }
                hash_map::Entry::Occupied(ref mut record) => {
                    record.get_mut().push(msg);
                }
            }
        });
    }
}

fn send_to_remote<AGN: Agent>(
    #[cfg(feature = "std_web")] worker: &Value,
    #[cfg(feature = "web_sys")] worker: &Worker,
    msg: ToWorker<AGN::Input>,
) {
    // TODO(#937): Important! Implement.
    // Use a queue to collect a messages if an instance is not ready
    // and send them to an agent when it will reported readiness.
    let msg = msg.pack();
    cfg_match! {
        feature = "std_web" => js! {
            var worker = @{worker};
            var bytes = @{msg};
            worker.postMessage(bytes);
        },
        feature = "web_sys" => worker.post_message_vec(msg),
    };
}

impl<AGN: Agent> Bridge<AGN> for PublicBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(self.id, msg);
        if self.worker_is_loaded() {
            send_to_remote::<AGN>(&self.worker, msg);
        } else {
            self.msg_to_queue(msg.pack());
        }
    }
}

impl<AGN: Agent> Drop for PublicBridge<AGN> {
    fn drop(&mut self) {
        let terminate_worker = REMOTE_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            let terminate_worker = {
                if let Some(launched) = pool.get_mut::<RemoteAgent<AGN>>() {
                    launched.remove_bridge(self)
                } else {
                    false
                }
            };

            if terminate_worker {
                pool.remove::<RemoteAgent<AGN>>();
            }

            terminate_worker
        });

        let disconnected = ToWorker::Disconnected(self.id);
        send_to_remote::<AGN>(&self.worker, disconnected);

        if terminate_worker {
            let destroy = ToWorker::Destroy;
            send_to_remote::<AGN>(&self.worker, destroy);

            REMOTE_AGENTS_LOADED.with(|loaded| {
                loaded.borrow_mut().remove(&TypeId::of::<AGN>());
            });

            REMOTE_AGENTS_EARLY_MSGS_QUEUE.with(|queue| {
                queue.borrow_mut().remove(&TypeId::of::<AGN>());
            });
        }
    }
}

/// Create a single instance in a browser.
#[allow(missing_debug_implementations)]
pub struct Global;

impl Discoverer for Global {}

/// Declares the behavior of the agent.
pub trait Agent: Sized + 'static {
    /// Reach capability of the agent.
    type Reach: Discoverer;
    /// Type of an input message.
    type Message;
    /// Incoming message type.
    type Input: Serialize + for<'de> Deserialize<'de>;
    /// Outgoing message type.
    type Output: Serialize + for<'de> Deserialize<'de>;

    /// Creates an instance of an agent.
    fn create(link: AgentLink<Self>) -> Self;

    /// This method called on every update message.
    fn update(&mut self, msg: Self::Message);

    /// This method called on when a new bridge created.
    fn connected(&mut self, _id: HandlerId) {}

    /// This method called on every incoming message.
    fn handle_input(&mut self, msg: Self::Input, id: HandlerId);

    /// This method called on when a new bridge destroyed.
    fn disconnected(&mut self, _id: HandlerId) {}

    /// This method called when the agent is destroyed.
    fn destroy(&mut self) {}

    /// Represents the name of loading resorce for remote workers which
    /// have to live in a separate files.
    fn name_of_resource() -> &'static str {
        "main.js"
    }

    /// Signifies if resource is a module.
    /// This has pending browser support.
    fn is_module() -> bool {
        false
    }
}

/// This struct holds a reference to a component and to a global scheduler.
pub struct AgentScope<AGN: Agent> {
    shared_agent: Shared<AgentRunnable<AGN>>,
}

impl<AGN: Agent> fmt::Debug for AgentScope<AGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentScope<_>")
    }
}

impl<AGN: Agent> Clone for AgentScope<AGN> {
    fn clone(&self) -> Self {
        AgentScope {
            shared_agent: self.shared_agent.clone(),
        }
    }
}

impl<AGN: Agent> AgentScope<AGN> {
    /// Create agent scope
    pub fn new() -> Self {
        let shared_agent = Rc::new(RefCell::new(AgentRunnable::new()));
        AgentScope { shared_agent }
    }
    /// Schedule message for sending to agent
    pub fn send(&self, update: AgentLifecycleEvent<AGN>) {
        let envelope = AgentEnvelope {
            shared_agent: self.shared_agent.clone(),
            update,
        };
        let runnable: Box<dyn Runnable> = Box::new(envelope);
        scheduler().push(runnable);
    }
}

impl<AGN: Agent> Default for AgentScope<AGN> {
    fn default() -> Self {
        Self::new()
    }
}

/// Defines communication from Worker to Consumers
pub trait Responder<AGN: Agent> {
    /// Implementation for communication channel from Worker to Consumers
    fn respond(&self, id: HandlerId, output: AGN::Output);
}

struct WorkerResponder {}

impl<AGN: Agent> Responder<AGN> for WorkerResponder {
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        let msg = FromWorker::ProcessOutput(id, output);
        let data = msg.pack();
        cfg_match! {
            feature = "std_web" => js! {
                var data = @{data};
                self.postMessage(data);
            },
            feature = "web_sys" => worker_self().post_message_vec(data),
        };
    }
}

/// Link to agent's scope for creating callbacks.
pub struct AgentLink<AGN: Agent> {
    scope: AgentScope<AGN>,
    responder: Rc<dyn Responder<AGN>>,
}

impl<AGN: Agent> AgentLink<AGN> {
    /// Create link for a scope.
    pub fn connect<T>(scope: &AgentScope<AGN>, responder: T) -> Self
    where
        T: Responder<AGN> + 'static,
    {
        AgentLink {
            scope: scope.clone(),
            responder: Rc::new(responder),
        }
    }

    /// Send response to an agent.
    pub fn respond(&self, id: HandlerId, output: AGN::Output) {
        self.responder.respond(id, output);
    }

    /// Create a callback which will send a message to the agent when invoked.
    pub fn callback<F, IN>(&self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> AGN::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            scope.send(AgentLifecycleEvent::Message(output));
        };
        closure.into()
    }
}

impl<AGN: Agent> fmt::Debug for AgentLink<AGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgentLink<_>")
    }
}

impl<AGN: Agent> Clone for AgentLink<AGN> {
    fn clone(&self) -> Self {
        AgentLink {
            scope: self.scope.clone(),
            responder: self.responder.clone(),
        }
    }
}

struct AgentRunnable<AGN> {
    agent: Option<AGN>,
    // TODO(#939): Use agent field to control create message this flag
    destroyed: bool,
}

impl<AGN> AgentRunnable<AGN> {
    fn new() -> Self {
        AgentRunnable {
            agent: None,
            destroyed: false,
        }
    }
}

/// Local Agent messages
#[derive(Debug)]
pub enum AgentLifecycleEvent<AGN: Agent> {
    /// Request to create link
    Create(AgentLink<AGN>),
    /// Internal Agent message
    Message(AGN::Message),
    /// Client connected
    Connected(HandlerId),
    /// Received mesasge from Client
    Input(AGN::Input, HandlerId),
    /// Client disconnected
    Disconnected(HandlerId),
    /// Request to destroy agent
    Destroy,
}

struct AgentEnvelope<AGN: Agent> {
    shared_agent: Shared<AgentRunnable<AGN>>,
    update: AgentLifecycleEvent<AGN>,
}

impl<AGN> Runnable for AgentEnvelope<AGN>
where
    AGN: Agent,
{
    fn run(self: Box<Self>) {
        let mut this = self.shared_agent.borrow_mut();
        if this.destroyed {
            return;
        }
        match self.update {
            AgentLifecycleEvent::Create(link) => {
                this.agent = Some(AGN::create(link));
            }
            AgentLifecycleEvent::Message(msg) => {
                this.agent
                    .as_mut()
                    .expect("agent was not created to process messages")
                    .update(msg);
            }
            AgentLifecycleEvent::Connected(id) => {
                this.agent
                    .as_mut()
                    .expect("agent was not created to send a connected message")
                    .connected(id);
            }
            AgentLifecycleEvent::Input(inp, id) => {
                this.agent
                    .as_mut()
                    .expect("agent was not created to process inputs")
                    .handle_input(inp, id);
            }
            AgentLifecycleEvent::Disconnected(id) => {
                this.agent
                    .as_mut()
                    .expect("agent was not created to send a disconnected message")
                    .disconnected(id);
            }
            AgentLifecycleEvent::Destroy => {
                let mut agent = this
                    .agent
                    .take()
                    .expect("trying to destroy not existent agent");
                agent.destroy();
            }
        }
    }
}

#[cfg(feature = "web_sys")]
fn worker_new(name_of_resource: &str, is_module: bool) -> Worker {
    let href = utils::document().location().unwrap().href().unwrap();
    let script_url = format!("{}{}", href, name_of_resource);
    let wasm_url = format!("{}{}", href, name_of_resource.replace(".js", "_bg.wasm"));
    let array = Array::new();
    array.push(
        &format!(
            r#"importScripts("{}");

            let initialized = wasm_bindgen("{}").catch(err => {{
                // Propagate to main `onerror`:
                setTimeout(() => {{
                    throw err;
                }});

                // Rethrow to keep promise rejected and prevent execution of further commands:
                throw err;
            }});

            self.onmessage = async (event) => {{
                await initialized;
                wasm_bindgen.child_entry_point(event.data);
            }};"#,
            script_url, wasm_url
        )
        .into(),
    );
    let blob = Blob::new_with_str_sequence_and_options(
        &array,
        BlobPropertyBag::new().type_("application/javascript"),
    )
    .unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    if is_module {
        let options = WorkerOptions::new();
        Reflect::set(
            options.as_ref(),
            &JsValue::from_str("type"),
            &JsValue::from_str("module"),
        )
        .unwrap();
        Worker::new_with_options(&url, &options).expect("failed to spawn worker")
    } else {
        Worker::new(&url).expect("failed to spawn worker")
    }
}

#[cfg(feature = "web_sys")]
fn worker_self() -> DedicatedWorkerGlobalScope {
    JsValue::from(js_sys::global()).into()
}

#[cfg(feature = "web_sys")]
trait WorkerExt {
    fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>));

    fn post_message_vec(&self, data: Vec<u8>);
}

#[cfg(feature = "web_sys")]
macro_rules! worker_ext_impl {
    ($($type:ident),+) => {$(
        impl WorkerExt for $type {
            fn set_onmessage_closure(&self, handler: impl 'static + Fn(Vec<u8>)) {
                let handler = move |message: MessageEvent| {
                    let data = Uint8Array::from(message.data()).to_vec();
                    handler(data);
                };
                let closure = Closure::wrap(Box::new(handler) as Box<dyn Fn(MessageEvent)>);
                self.set_onmessage(Some(closure.as_ref().unchecked_ref()));
                closure.forget();
            }

            fn post_message_vec(&self, data: Vec<u8>) {
                self.post_message(&Uint8Array::from(data.as_slice()))
                    .expect("failed to post message");
            }
        }
    )+};
}

#[cfg(feature = "web_sys")]
worker_ext_impl! {
    Worker, DedicatedWorkerGlobalScope
}
