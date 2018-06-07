//! This module contains types to support multi-threading in Yew.

use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
use anymap::{AnyMap, Entry};
use slab::Slab;
use stdweb::Value;
use stdweb::unstable::TryInto;
use scheduler::{Runnable, scheduler};
use callback::Callback;
use Shared;

type Pair = (Box<Fn()>, Box<Fn(HandlerId, Vec<u8>)>);

thread_local! {
    pub(crate) static AGENTS: RefCell<HashMap<TypeId, Pair>> =
        RefCell::new(HashMap::new());
}

/// WARNING! This thing depends on implementation of `TypeId` of Rust.
type RawTypeId = u64;

#[derive(Serialize, Deserialize, Debug)]
enum ToWorker {
    SelectType(RawTypeId),
    ProcessInput(usize, Vec<u8>),
}

impl Transferable for ToWorker { }

#[derive(Serialize, Deserialize, Debug)]
enum FromWorker {
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    TypeDetected,
    ProcessOutput(usize, Vec<u8>),
}

impl Transferable for FromWorker { }

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
pub struct HandlerId(usize);

type HandlersPool<T> = Rc<RefCell<Slab<Callback<T>>>>;

/// This traits allow to get addres or register worker.
// TODO Maybe use somethig like `App` for `Component`s.
pub trait Worker: Agent + Sized + 'static {
    /// Creates a messaging bridge between a worker and the component.
    fn bridge(callback: Callback<Self::Output>) -> Box<Bridge<Self>>;
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
        // Register function which puts every incoming message to a scheduler.
        let scope_base: AgentScope<T> = AgentScope::new();
        let scope = scope_base.clone();
        let creator = move || {
            let responder = WorkerResponder { };
            let link = AgentLink::connect(&scope, responder);
            let upd = AgentUpdate::Create(link);
            scope.send(upd);
        };
        let scope = scope_base.clone();
        let routine = move |id: HandlerId, data: Vec<u8>| {
            let msg: T::Input = bincode::deserialize(&data)
                .expect("can't deserialize an input message");
            let upd = AgentUpdate::Input(msg, id);
            scope.send(upd);
        };
        AGENTS.with(move |agents| {
            let type_id = TypeId::of::<Self>();
            let pair: Pair = (Box::new(creator), Box::new(routine));
            agents.borrow_mut().insert(type_id, pair);
        });
    }
}

impl<T> Worker for T
where
    T: Agent,
{
    fn bridge(callback: Callback<Self::Output>) -> Box<Bridge<Self>> {
        Self::Reach::spawn_or_join(callback)
    }
}

/// Determine a visibility of an agent.
#[doc(hidden)]
pub trait Discoverer {
    /// Spawns an agent and returns `Bridge` implementation.
    fn spawn_or_join<AGN: Agent>(_callback: Callback<AGN::Output>) -> Box<Bridge<AGN>> {
        unimplemented!();
    }
}

/// Bridge to a specific kind of worker.
pub trait Bridge<AGN: Agent> {
    /// Send a message to an agent.
    fn send(&self, msg: AGN::Input);
}

// <<< SAME THREAD >>>

struct LaunchedAgent<AGN: Agent> {
    scope: AgentScope<AGN>,
    slab: Shared<Slab<Callback<AGN::Output>>>,
}

type Last = bool;

impl<AGN: Agent> LaunchedAgent<AGN> {
    pub fn new(scope: &AgentScope<AGN>) -> Self {
        let slab = Rc::new(RefCell::new(Slab::new()));
        LaunchedAgent {
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
            id,
        }
    }

    fn remove_bridge(&mut self, bridge: &ContextBridge<AGN>) -> Last {
        let mut slab = self.slab.borrow_mut();
        let _ = slab.remove(bridge.id);
        slab.is_empty()
    }
}

thread_local! {
    static CONTEXT_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

/// Create a single instance in the current thread.
pub struct Context;

impl Discoverer for Context {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<Bridge<AGN>> {
        let mut scope_to_init = None;
        let bridge = CONTEXT_POOL.with(|pool| {
            match pool.borrow_mut().entry::<LaunchedAgent<AGN>>() {
                Entry::Occupied(mut entry) => {
                    // TODO Insert callback!
                    entry.get_mut().create_bridge(callback)
                },
                Entry::Vacant(entry) => {
                    let scope = AgentScope::<AGN>::new();
                    let launched = LaunchedAgent::new(&scope);
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
        Box::new(bridge)
    }
}

struct SlabResponder<AGN: Agent> {
    slab: Shared<Slab<Callback<AGN::Output>>>,
}

impl<AGN: Agent> Responder<AGN> for SlabResponder<AGN> {
    fn response(&self, id: usize, output: AGN::Output) {
        let callback = self.slab.borrow().get(id).cloned();
        if let Some(callback) = callback {
            callback.emit(output);
        } else {
            warn!("Id of handler not exists <slab>: {}", id);
        }
    }
}

struct ContextBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
    id: usize,
}

impl<AGN: Agent> Bridge<AGN> for ContextBridge<AGN> {
    fn send(&self, msg: AGN::Input) {
        let upd = AgentUpdate::Input(msg, HandlerId(self.id));
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for ContextBridge<AGN> {
    fn drop(&mut self) {
        CONTEXT_POOL.with(|pool| {
            let terminate_worker = {
                if let Some(launched) = pool.borrow_mut().get_mut::<LaunchedAgent<AGN>>() {
                    launched.remove_bridge(self)
                } else {
                    false
                }
            };
            if terminate_worker {
                pool.borrow_mut().remove::<LaunchedAgent<AGN>>();
            }
        });
    }
}

/// Create an instance in the current thread.
pub struct Job;

impl Discoverer for Job {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<Bridge<AGN>> {
        let scope = AgentScope::<AGN>::new();
        let responder = CallbackResponder { callback };
        let agent_link = AgentLink::connect(&scope, responder);
        let upd = AgentUpdate::Create(agent_link);
        scope.send(upd);
        let bridge = JobBridge { scope };
        Box::new(bridge)
    }
}

const JOB_SINGLE_ID: usize = 0;

struct CallbackResponder<AGN: Agent> {
    callback: Callback<AGN::Output>,
}

impl<AGN: Agent> Responder<AGN> for CallbackResponder<AGN> {
    fn response(&self, id: usize, output: AGN::Output) {
        assert_eq!(id, JOB_SINGLE_ID);
        self.callback.emit(output);
    }
}

struct JobBridge<AGN: Agent> {
    scope: AgentScope<AGN>,
}

impl<AGN: Agent> Bridge<AGN> for JobBridge<AGN> {
    fn send(&self, msg: AGN::Input) {
        let upd = AgentUpdate::Input(msg, HandlerId(JOB_SINGLE_ID));
        self.scope.send(upd);
    }
}

impl<AGN: Agent> Drop for JobBridge<AGN> {
    fn drop(&mut self) {
        let upd = AgentUpdate::Destroy;
        self.scope.send(upd);
    }
}

// <<< SEPARATE THREAD >>>

/// Create a new instance for every bridge.
pub struct Private;

impl Discoverer for Private { }

/// Create a single instance in a tab.
pub struct Public;

impl Discoverer for Public {
    fn spawn_or_join<AGN: Agent>(callback: Callback<AGN::Output>) -> Box<Bridge<AGN>> {
        let worker_base = js! {
            // TODO Use relative path. But how?
            var worker = new Worker("main.js");
            return worker;
        };
        let worker = worker_base.clone();
        let send_to_app = move |msg: ToWorker| {
            let bytes = msg.pack();
            let worker = worker.clone();
            js! {
                var worker = @{worker};
                worker.postMessage(@{bytes});
            };
        };
        let slab: Slab<Callback<AGN::Output>> = Slab::new();
        let callbacks_base = Rc::new(RefCell::new(slab));
        let callbacks = callbacks_base.clone();
        let handshake = move |data: Vec<u8>| {
            let msg = FromWorker::unpack(&data);
            match msg {
                FromWorker::WorkerLoaded => {
                    let type_id = TypeId::of::<AGN>();
                    let raw_type_id: RawTypeId = unsafe { ::std::mem::transmute(type_id) };
                    let msg = ToWorker::SelectType(raw_type_id);
                    send_to_app(msg);
                },
                FromWorker::TypeDetected => {
                    info!("Worker handshake finished");
                },
                FromWorker::ProcessOutput(id, data) => {
                    let msg = AGN::Output::unpack(&data);
                    let callback = callbacks.borrow().get(id).cloned();
                    if let Some(callback) = callback {
                        callback.emit(msg);
                    }
                },
            }
        };
        let worker = worker_base.clone();
        js! {
            var worker = @{worker};
            // TODO Send type id (but on ready event)
            var handshake = @{handshake};
            worker.onmessage = function(event) {
                handshake(event.data);
            };
        };
        let id = callbacks_base.borrow_mut().insert(callback);
        let bridge = PublicBridge {
            worker: worker_base,
            callbacks: callbacks_base,
            id,
        };
        Box::new(bridge)
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

    /// This metthod called on every update message.
    fn update(&mut self, msg: Self::Message);

    /// This metthod called on every incoming message.
    fn handle(&mut self, msg: Self::Input, id: HandlerId);

    /// Creates an instance of an agent.
    fn destroy(&mut self) { }

}

/// A connection manager for components interaction with workers.
pub struct PublicBridge<T: Agent> {
    worker: Value,
    callbacks: HandlersPool<T::Output>,
    id: usize,
}

impl<AGN: Agent> Bridge<AGN> for PublicBridge<AGN> {
    fn send(&self, msg: AGN::Input) {
        // TODO Important! Implement.
        // Use a queue to collect a messages if an instance is not ready
        // and send them to an agent when it will reported readiness.
        let bytes = bincode::serialize(&msg)
            .expect("can't serialize message for agent");
        let msg = ToWorker::ProcessInput(self.id, bytes).pack();
        let worker = &self.worker;
        js! {
            var worker = @{worker};
            var bytes = @{msg};
            worker.postMessage(bytes);
        };
    }
}

impl<AGN: Agent> Drop for PublicBridge<AGN> {
    fn drop(&mut self) {
        let _ = self.callbacks.borrow_mut().remove(self.id);
    }
}

/// This function selects the agent to start.
pub(crate) fn run_agent() {
    let sender = |msg: FromWorker| {
        let data = msg.pack();
        js! {
            var data = @{data};
            self.postMessage(data);
        };
    };
    let mut handler = None;
    let handshake = move |data: Vec<u8>| {
        let msg = ToWorker::unpack(&data);
        match msg {
            ToWorker::SelectType(raw_type_id) => {
                let type_id: TypeId = unsafe { ::std::mem::transmute(raw_type_id) };
                handler = AGENTS.with(move |agents| {
                    let mut agents = agents.borrow_mut();
                    let result = agents.remove(&type_id);
                    agents.clear(); // Drop unnecessary types of handlers
                    result.map(|(creator, handler)| {
                        creator();
                        handler
                    })
                });
            },
            ToWorker::ProcessInput(id, data) => {
                let func = handler.as_mut()
                    .expect("TypeId of agent was not selected.");
                let handler_id = HandlerId(id);
                func(handler_id, data);
            },
        }
    };
    js! {
        let handshake = @{handshake};
        self.onmessage = function(event) {
            handshake(event.data);
        };
        // TODO Clean up the allocated memory
    };
    sender(FromWorker::WorkerLoaded);
}

pub(crate) fn detect_ambit() -> Ambit {
    let res = js! {
        return !(self.document === undefined);
    };
    let is_window = res.try_into().expect("can't check the type of self environment");
    if is_window { Ambit::Application } else { Ambit::Agent }

}

/// Represents the kind of environment where the instance lives.
pub enum Ambit {
    /// `Window` environment
    Application,
    /// `Worker` environment
    Agent,
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
        let runnable: Box<Runnable> = Box::new(envelope);
        scheduler().put_and_try_run(runnable);
    }
}

trait Responder<AGN: Agent> {
    fn response(&self, id: usize, output: AGN::Output);
}

struct WorkerResponder {
}

impl<AGN: Agent> Responder<AGN> for WorkerResponder {
    fn response(&self, id: usize, output: AGN::Output) {
        let msg = FromWorker::ProcessOutput(id, output.pack());
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
    responder: Box<Responder<AGN>>,
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
    pub fn response(&self, HandlerId(id): HandlerId, output: AGN::Output) {
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
    Input(AGN::Input, HandlerId),
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
            AgentUpdate::Input(inp, id) => {
                this.agent.as_mut()
                    .expect("agent was not created to process inputs")
                    .handle(inp, id);
            }
            AgentUpdate::Destroy => {
                let mut agent = this.agent.take()
                    .expect("trying to destroy not existent agent");
                agent.destroy();
            }
        }
    }
}
