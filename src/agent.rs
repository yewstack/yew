//! This module contains types to support multi-threading in Yew.

use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;
use serde::{Serialize, Deserialize};
use bincode;
use anymap::{AnyMap, Entry};
use slab::Slab;
use stdweb::Value;
use scheduler::{Runnable, Shared, scheduler};
use callback::Callback;

#[derive(Serialize, Deserialize)]
enum ToWorker<T> {
    Connected(HandlerId),
    ProcessInput(HandlerId, T),
    Disconnected(HandlerId),
    Destroy,
}

impl<T> Transferable for ToWorker<T>
where
    T: Serialize + for <'de> Deserialize<'de>,
{
}

#[derive(Serialize, Deserialize)]
enum FromWorker<T> {
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    ProcessOutput(HandlerId, T),
}

impl<T> Transferable for FromWorker<T>
where
    T: Serialize + for <'de> Deserialize<'de>,
{
}


/// Represents a message which you could send to an agent.
pub trait Transferable
where
    Self: Serialize + for <'de> Deserialize<'de>,
{
}

trait Packed {
    fn pack(&self) -> Vec<u8>;
    fn unpack(data: &Vec<u8>) -> Self;
}

impl<T: Transferable> Packed for T {
    fn pack(&self) -> Vec<u8> {
        bincode::serialize(&self)
            .expect("can't serialize a transferable object")
    }

    fn unpack(data: &Vec<u8>) -> Self {
        bincode::deserialize(&data)
            .expect("can't deserialize a transferable object")
    }
}

/// Id of responses handler.
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
pub struct HandlerId(usize);

impl From<usize> for HandlerId {
    fn from(id: usize) -> Self {
        HandlerId(id)
    }
}

impl HandlerId {
    fn raw_id(&self) -> usize {
        self.0
    }
}

/// This traits allow to get addres or register worker.
pub trait Bridged: Agent + Sized + 'static {
    /// Creates a messaging bridge between a worker and the component.
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>>;
}

/// Implements rules to register a worker in a separate thread.
pub trait Threaded {
    /// Executes an agent in the current environment.
    /// Uses in `main` function of a worker.
    fn register();
}

impl<T> Threaded for T
where
    T: Agent<Reach=Public>,
{
    fn register() {
        let scope = AgentScope::<T>::new();
        let responder = WorkerResponder { };
        let link = AgentLink::connect(&scope, responder);
        let upd = AgentUpdate::Create(link);
        scope.send(upd);
        let handler = move |data: Vec<u8>| {
            let msg = ToWorker::<T::Input>::unpack(&data);
            match msg {
                ToWorker::Connected(id) => {
                    let upd = AgentUpdate::Connected(id);
                    scope.send(upd);
                },
                ToWorker::ProcessInput(id, value) => {
                    let upd = AgentUpdate::Input(value, id);
                    scope.send(upd);
                },
                ToWorker::Disconnected(id) => {
                    let upd = AgentUpdate::Disconnected(id);
                    scope.send(upd);
                },
                ToWorker::Destroy => {
                    let upd = AgentUpdate::Destroy;
                    scope.send(upd);
                    js! {
                        // Terminates web worker
                        self.close();
                    };
                },
            }
        };
        let loaded: FromWorker<T::Output> = FromWorker::WorkerLoaded;
        let loaded = loaded.pack();
        js! {
            var handler = @{handler};
            self.onmessage = function(event) {
                handler(event.data);
            };
            self.postMessage(@{loaded});
        };
    }
}

impl<T> Bridged for T
where
    T: Agent,
{
    fn bridge(callback: Callback<Self::Output>) -> Box<dyn Bridge<Self>> {
        Self::Reach::spawn_or_join(callback)
    }
}

/// Determine a visibility of an agent.
#[doc(hidden)]
pub trait Discoverer {
    /// Spawns an agent and returns `Bridge` implementation.
    fn spawn_or_join<AGN: Agent>(_callback: Callback<AGN::Output>) -> Box<dyn Bridge<AGN>> {
        unimplemented!();
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
    slab: Shared<Slab<Callback<AGN::Output>>>,
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

    fn slab(&self) -> Shared<Slab<Callback<AGN::Output>>> {
        self.slab.clone()
    }

    fn create_bridge(&mut self, callback: Callback<AGN::Output>) -> ContextBridge<AGN> {
        let id = self.slab.borrow_mut().insert(callback);
        ContextBridge {
            scope: self.scope.clone(),
            id: id.into(),
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
pub struct Context;

impl Discoverer for Context {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<dyn Bridge<AGN>> {
        let mut scope_to_init = None;
        let bridge = LOCAL_AGENTS_POOL.with(|pool| {
            match pool.borrow_mut().entry::<LocalAgent<AGN>>() {
                Entry::Occupied(mut entry) => {
                    // TODO Insert callback!
                    entry.get_mut().create_bridge(callback)
                },
                Entry::Vacant(entry) => {
                    let scope = AgentScope::<AGN>::new();
                    let launched = LocalAgent::new(&scope);
                    let responder = SlabResponder { slab: launched.slab() };
                    scope_to_init = Some((scope.clone(), responder));
                    entry.insert(launched).create_bridge(callback)
                },
            }
        });
        if let Some((scope, responder)) = scope_to_init {
            let agent_link = AgentLink::connect(&scope, responder);
            let upd = AgentUpdate::Create(agent_link);
            scope.send(upd);
        }
        let upd = AgentUpdate::Connected(bridge.id.into());
        bridge.scope.send(upd);
        Box::new(bridge)
    }
}

struct SlabResponder<AGN: Agent> {
    slab: Shared<Slab<Callback<AGN::Output>>>,
}

impl<AGN: Agent> Responder<AGN> for SlabResponder<AGN> {
    fn response(&self, id: HandlerId, output: AGN::Output) {
        let callback = self.slab.borrow().get(id.raw_id()).cloned();
        if let Some(callback) = callback {
            callback.emit(output);
        } else {
            warn!("Id of handler not exists <slab>: {}", id.raw_id());
        }
    }
}

struct ContextBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
    id: HandlerId,
}

impl<AGN: Agent> Bridge<AGN> for ContextBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let upd = AgentUpdate::Input(msg, self.id);
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for ContextBridge<AGN> {
    fn drop(&mut self) {
        LOCAL_AGENTS_POOL.with(|pool| {
            let terminate_worker = {
                if let Some(launched) = pool.borrow_mut().get_mut::<LocalAgent<AGN>>() {
                    launched.remove_bridge(self)
                } else {
                    false
                }
            };
            let upd = AgentUpdate::Disconnected(self.id);
            self.scope.send(upd);
            if terminate_worker {
                let upd = AgentUpdate::Destroy;
                self.scope.send(upd);
                pool.borrow_mut().remove::<LocalAgent<AGN>>();
            }
        });
    }
}

/// Create an instance in the current thread.
pub struct Job;

impl Discoverer for Job {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<dyn Bridge<AGN>> {
        let scope = AgentScope::<AGN>::new();
        let responder = CallbackResponder { callback };
        let agent_link = AgentLink::connect(&scope, responder);
        let upd = AgentUpdate::Create(agent_link);
        scope.send(upd);
        let upd = AgentUpdate::Connected(SINGLETON_ID);
        scope.send(upd);
        let bridge = JobBridge { scope };
        Box::new(bridge)
    }
}

const SINGLETON_ID: HandlerId = HandlerId(0);

struct CallbackResponder<AGN: Agent> {
    callback: Callback<AGN::Output>,
}

impl<AGN: Agent> Responder<AGN> for CallbackResponder<AGN> {
    fn response(&self, id: HandlerId, output: AGN::Output) {
        assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
        self.callback.emit(output);
    }
}

struct JobBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
}

impl<AGN: Agent> Bridge<AGN> for JobBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let upd = AgentUpdate::Input(msg, SINGLETON_ID);
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for JobBridge<AGN> {
    fn drop(&mut self) {
        let upd = AgentUpdate::Disconnected(SINGLETON_ID);
        self.scope.send(upd);
        let upd = AgentUpdate::Destroy;
        self.scope.send(upd);
    }
}

// <<< SEPARATE THREAD >>>

/// Create a new instance for every bridge.
pub struct Private;

impl Discoverer for Private {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<dyn Bridge<AGN>> {
        let handler = move |data: Vec<u8>| {
            let msg = FromWorker::<AGN::Output>::unpack(&data);
            match msg {
                FromWorker::WorkerLoaded => {
                    // TODO Send `Connected` message
                },
                FromWorker::ProcessOutput(id, output) => {
                    assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
                    callback.emit(output);
                },
            }
        };
        // TODO Need somethig better...
        let name_of_resource = AGN::name_of_resource();
        let worker = js! {
            var worker = new Worker(@{name_of_resource});
            var handler = @{handler};
            worker.onmessage = function(event) {
                handler(event.data);
            };
            return worker;
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
    worker: Value,
    _agent: PhantomData<T>,
}

impl<AGN: Agent> Bridge<AGN> for PrivateBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        // TODO Important! Implement.
        // Use a queue to collect a messages if an instance is not ready
        // and send them to an agent when it will reported readiness.
        let msg = ToWorker::ProcessInput(SINGLETON_ID, msg).pack();
        let worker = &self.worker;
        js! {
            var worker = @{worker};
            var bytes = @{msg};
            worker.postMessage(bytes);
        };
    }
}

impl<AGN: Agent> Drop for PrivateBridge<AGN> {
    fn drop(&mut self) {
        // TODO Send `Destroy` message.
    }
}

struct RemoteAgent<AGN: Agent> {
    worker: Value,
    slab: Shared<Slab<Callback<AGN::Output>>>,
}

impl<AGN: Agent> RemoteAgent<AGN> {
    pub fn new(worker: &Value, slab: Shared<Slab<Callback<AGN::Output>>>) -> Self {
        RemoteAgent {
            worker: worker.clone(),
            slab,
        }
    }

    fn create_bridge(&mut self, callback: Callback<AGN::Output>) -> PublicBridge<AGN> {
        let id = self.slab.borrow_mut().insert(callback);
        PublicBridge {
            worker: self.worker.clone(),
            id: id.into(),
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
}

/// Create a single instance in a tab.
pub struct Public;

impl Discoverer for Public {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<dyn Bridge<AGN>> {
        let bridge = REMOTE_AGENTS_POOL.with(|pool| {
            match pool.borrow_mut().entry::<RemoteAgent<AGN>>() {
                Entry::Occupied(mut entry) => {
                    // TODO Insert callback!
                    entry.get_mut().create_bridge(callback)
                },
                Entry::Vacant(entry) => {
                    let slab_base: Shared<Slab<Callback<AGN::Output>>> =
                        Rc::new(RefCell::new(Slab::new()));
                    let slab = slab_base.clone();
                    let handler = move |data: Vec<u8>| {
                        let msg = FromWorker::<AGN::Output>::unpack(&data);
                        match msg {
                            FromWorker::WorkerLoaded => {
                                // TODO Use `AtomicBool` lock to check its loaded
                                // TODO Send `Connected` message
                            },
                            FromWorker::ProcessOutput(id, output) => {
                                let callback = slab.borrow().get(id.raw_id()).cloned();
                                if let Some(callback) = callback {
                                    callback.emit(output);
                                } else {
                                    warn!("Id of handler for remote worker not exists <slab>: {}", id.raw_id());
                                }
                            },
                        }
                    };
                    let name_of_resource = AGN::name_of_resource();
                    let worker = js! {
                        var worker = new Worker(@{name_of_resource});
                        var handler = @{handler};
                        worker.onmessage = function(event) {
                            handler(event.data);
                        };
                        return worker;
                    };
                    let launched = RemoteAgent::new(&worker, slab_base);
                    entry.insert(launched).create_bridge(callback)
                },
            }
        });
        Box::new(bridge)
    }
}

/// A connection manager for components interaction with workers.
pub struct PublicBridge<T: Agent> {
    worker: Value,
    id: HandlerId,
    _agent: PhantomData<T>,
}

impl<AGN: Agent> PublicBridge<AGN> {
    fn send_to_remote(&self, msg: ToWorker<AGN::Input>) {
        // TODO Important! Implement.
        // Use a queue to collect a messages if an instance is not ready
        // and send them to an agent when it will reported readiness.
        let msg = msg.pack();
        let worker = &self.worker;
        js! {
            var worker = @{worker};
            var bytes = @{msg};
            worker.postMessage(bytes);
        };
    }
}

impl<AGN: Agent> Bridge<AGN> for PublicBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(self.id, msg);
        self.send_to_remote(msg);
    }
}

impl<AGN: Agent> Drop for PublicBridge<AGN> {
    fn drop(&mut self) {
        REMOTE_AGENTS_POOL.with(|pool| {
            let terminate_worker = {
                if let Some(launched) = pool.borrow_mut().get_mut::<RemoteAgent<AGN>>() {
                    launched.remove_bridge(self)
                } else {
                    false
                }
            };
            let upd = ToWorker::Disconnected(self.id);
            self.send_to_remote(upd);
            if terminate_worker {
                let upd = ToWorker::Destroy;
                self.send_to_remote(upd);
                pool.borrow_mut().remove::<RemoteAgent<AGN>>();
            }
        });
    }
}


/// Create a single instance in a browser.
pub struct Global;

impl Discoverer for Global { }

/// Declares the behavior of the agent.
pub trait Agent: Sized + 'static {
    /// Reach capaility of the agent.
    type Reach: Discoverer;
    /// Type of an input messagae.
    type Message;
    /// Incoming message type.
    type Input: Transferable;
    /// Outgoing message type.
    type Output: Transferable;

    /// Creates an instance of an agent.
    fn create(link: AgentLink<Self>) -> Self;

    /// This method called on every update message.
    fn update(&mut self, msg: Self::Message);

    /// This method called on when a new bridge created.
    fn connected(&mut self, _id: HandlerId) { }

    /// This method called on every incoming message.
    fn handle(&mut self, msg: Self::Input, id: HandlerId);

    /// This method called on when a new bridge destroyed.
    fn disconnected(&mut self, _id: HandlerId) { }

    /// Creates an instance of an agent.
    fn destroy(&mut self) { }

    /// Represents the name of loading resorce for remote workers which
    /// have to live in a separate files.
    fn name_of_resource() -> &'static str { "main.js" }


}

/// This sctruct holds a reference to a component and to a global scheduler.
pub struct AgentScope<AGN: Agent> {
    shared_agent: Shared<AgentRunnable<AGN>>,
}

impl<AGN: Agent> Clone for AgentScope<AGN> {
    fn clone(&self) -> Self {
        AgentScope {
            shared_agent: self.shared_agent.clone(),
        }
    }
}

impl<AGN: Agent> AgentScope<AGN> {
    fn new() -> Self {
        let shared_agent = Rc::new(RefCell::new(AgentRunnable::new()));
        AgentScope { shared_agent }
    }

    fn send(&self, update: AgentUpdate<AGN>) {
        let envelope = AgentEnvelope {
            shared_agent: self.shared_agent.clone(),
            message: Some(update),
        };
        let runnable: Box<dyn Runnable> = Box::new(envelope);
        scheduler().put_and_try_run(runnable);
    }
}

trait Responder<AGN: Agent> {
    fn response(&self, id: HandlerId, output: AGN::Output);
}

struct WorkerResponder {
}

impl<AGN: Agent> Responder<AGN> for WorkerResponder {
    fn response(&self, id: HandlerId, output: AGN::Output) {
        let msg = FromWorker::ProcessOutput(id, output);
        let data = msg.pack();
        js! {
            var data = @{data};
            self.postMessage(data);
        };
    }
}

/// Link to agent's scope for creating callbacks.
pub struct AgentLink<AGN: Agent> {
    scope: AgentScope<AGN>,
    responder: Box<dyn Responder<AGN>>,
}

impl<AGN: Agent> AgentLink<AGN> {
    /// Create link for a scope.
    fn connect<T>(scope: &AgentScope<AGN>, responder: T) -> Self
    where
        T: Responder<AGN> + 'static,
    {
        AgentLink {
            scope: scope.clone(),
            responder: Box::new(responder),
        }
    }

    /// Send response to an actor.
    pub fn response(&self, id: HandlerId, output: AGN::Output) {
        self.responder.response(id, output);
    }

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&self, function: F) -> Callback<IN>
    where
        F: Fn(IN) -> AGN::Message + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input);
            let msg = AgentUpdate::Message(output);
            scope.clone().send(msg);
        };
        closure.into()
    }
}

struct AgentRunnable<AGN> {
    agent: Option<AGN>,
    // TODO Use agent field to control create message this flag
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

enum AgentUpdate<AGN: Agent> {
    Create(AgentLink<AGN>),
    Message(AGN::Message),
    Connected(HandlerId),
    Input(AGN::Input, HandlerId),
    Disconnected(HandlerId),
    Destroy,
}

struct AgentEnvelope<AGN: Agent> {
    shared_agent: Shared<AgentRunnable<AGN>>,
    message: Option<AgentUpdate<AGN>>,
}

impl<AGN> Runnable for AgentEnvelope<AGN>
where
    AGN: Agent,
{
    fn run(&mut self) {
        let mut this = self.shared_agent.borrow_mut();
        if this.destroyed {
            return;
        }
        let upd = self.message.take().expect("agent's envelope called twice");
        match upd {
            AgentUpdate::Create(env) => {
                this.agent = Some(AGN::create(env));
            }
            AgentUpdate::Message(msg) => {
                this.agent.as_mut()
                    .expect("agent was not created to process messages")
                    .update(msg);
            }
            AgentUpdate::Connected(id) => {
                this.agent.as_mut()
                    .expect("agent was not created to send a connected message")
                    .connected(id);
            }
            AgentUpdate::Input(inp, id) => {
                this.agent.as_mut()
                    .expect("agent was not created to process inputs")
                    .handle(inp, id);
            }
            AgentUpdate::Disconnected(id) => {
                this.agent.as_mut()
                    .expect("agent was not created to send a disconnected message")
                    .disconnected(id);
            }
            AgentUpdate::Destroy => {
                let mut agent = this.agent.take()
                    .expect("trying to destroy not existent agent");
                agent.destroy();
            }
        }
    }
}
