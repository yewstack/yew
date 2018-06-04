//! This module contains types to support multi-threading in Yew.

use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
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
    /// Spawns an agent and returns `Addr` of an instance.
    fn spawn() -> Addr<Self>;
    /// Executes an agent in the current environment.
    /// Uses in `main` function of a worker.
    fn register();
}

impl<T> Worker for T
where
    T: Agent,
{
    fn spawn() -> Addr<Self> {
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
        let slab: Slab<Callback<T::Output>> = Slab::new();
        let callbacks_base = Rc::new(RefCell::new(slab));
        let callbacks = callbacks_base.clone();
        let handshake = move |data: Vec<u8>| {
            let msg = FromWorker::unpack(&data);
            info!("Received from worker: {:?}", msg);
            match msg {
                FromWorker::WorkerLoaded => {
                    let type_id = TypeId::of::<Self>();
                    let raw_type_id: RawTypeId = unsafe { ::std::mem::transmute(type_id) };
                    let msg = ToWorker::SelectType(raw_type_id);
                    send_to_app(msg);
                },
                FromWorker::TypeDetected => {
                    info!("Worker handshake finished");
                },
                FromWorker::ProcessOutput(id, data) => {
                    let msg = T::Output::unpack(&data);
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
        Addr {
            worker: worker_base,
            callbacks: callbacks_base,
        }
    }

    fn register() {
        // Register function which puts every incoming message to a scheduler.
        let scope_base: AgentScope<T> = AgentScope::new();
        let scope = scope_base.clone();
        let creator = move || {
            let link = AgentLink::connect(&scope);
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

/// Declares the behavior of the agent.
pub trait Agent: Sized + 'static {
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
pub struct Bridge<T: Agent> {
    worker: Value,
    callbacks: HandlersPool<T::Output>,
    id: usize,
}

impl<T: Agent> Bridge<T> {
    fn new(worker: Value, callbacks: HandlersPool<T::Output>, callback: Callback<T::Output>) -> Self {
        let id = callbacks.borrow_mut().insert(callback);
        Bridge { worker, callbacks, id }
    }

    /// Send a message to an agent.
    pub fn send(&self, msg: T::Input) {
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
            console.log("Sending...", bytes);
            worker.postMessage(bytes);
        };
    }
}

impl<T: Agent> Drop for Bridge<T> {
    fn drop(&mut self) {
        let _ = self.callbacks.borrow_mut().remove(self.id);
    }
}

/// Address of an agent.
pub struct Addr<T: Agent> {
    // TODO Wrap this value with special Rc and track when to terminate the worker.
    worker: Value,
    callbacks: HandlersPool<T::Output>,
}

impl<T> Addr<T>
where
    T: Agent,
{
    /// Creates bridge connection between a component and the agent.
    pub fn bridge(&mut self, callback: Callback<T::Output>) -> Bridge<T> {
        Bridge::new(self.worker.clone(), self.callbacks.clone(), callback)
    }

}

impl<T: Agent> Drop for Addr<T> {
    fn drop(&mut self) {
        // TODO Use Rc if it will implement Clone
        let worker = &self.worker;
        js! {
            let worker = @{worker};
            //worker.terminate();
        };
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
        console.log("Mounted...", self);
        self.onmessage = function(event) {
            // TODO Send type_id, but how?
            console.log("Received...", event.data);
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

/// Link to agent's scope for creating callbacks.
pub struct AgentLink<AGN: Agent> {
    scope: AgentScope<AGN>,
}

impl<AGN: Agent> AgentLink<AGN> {
    /// Create link for a scope.
    fn connect(scope: &AgentScope<AGN>) -> Self {
        AgentLink {
            scope: scope.clone(),
        }
    }

    /// Send response to an actor.
    pub fn response(&self, HandlerId(id): HandlerId, output: AGN::Output) {
        let msg = FromWorker::ProcessOutput(id, output.pack());
        let data = msg.pack();
        js! {
            var data = @{data};
            self.postMessage(data);
        };
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
