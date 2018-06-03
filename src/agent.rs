//! This module contains types to support multi-threading in Yew.

use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;
use std::marker::PhantomData;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
use stdweb::Value;
use stdweb::unstable::TryInto;
use scheduler::{Scheduler, Runnable};
use callback::Callback;
use Shared;

type Pair = (Box<Fn()>, Box<Fn(Vec<u8>)>);

thread_local! {
    pub(crate) static AGENTS: RefCell<HashMap<TypeId, Pair>> =
        RefCell::new(HashMap::new());
}

/// WARNING! This thing depends on implementation of `TypeId` of Rust.
type RawTypeId = u64;

#[derive(Serialize, Deserialize, Debug)]
enum ToWorker {
    SelectType(RawTypeId),
    ProcessInput(Vec<u8>),
}

impl From<Vec<u8>> for ToWorker {
    fn from(data: Vec<u8>) -> Self {
        bincode::deserialize(&data)
            .expect("can't deserialize a message from a component")
    }
}

impl Into<Vec<u8>> for ToWorker {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self)
            .expect("can't serialize a message from a component")
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum FromWorker {
    /// Worker sends this message when `wasm` bundle has loaded.
    WorkerLoaded,
    TypeDetected,
    ProcessOutput(Vec<u8>),
}

impl From<Vec<u8>> for FromWorker {
    fn from(data: Vec<u8>) -> Self {
        bincode::deserialize(&data)
            .expect("can't deserialize a message from a worker")
    }
}

impl Into<Vec<u8>> for FromWorker {
    fn into(self) -> Vec<u8> {
        bincode::serialize(&self)
            .expect("can't serialize a message from a worker")
    }
}

/// Represents a message which you could send to an agent.
pub trait Message
where
    Self: Serialize + for <'de> Deserialize<'de>,
{
}

/// This traits allow to get addres or register worker.
// TODO Maybe use somethig like `App` for `Component`s.
pub trait Worker: Sized + 'static {
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
            let bytes: Vec<u8> = msg.into();
            let worker = worker.clone();
            js! {
                var worker = @{worker};
                worker.postMessage(@{bytes});
            };
        };
        let handshake = move |data: Vec<u8>| {
            let msg = FromWorker::from(data);
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
                FromWorker::ProcessOutput(_) => {
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
            _agent: PhantomData,
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
        let routine = move |data: Vec<u8>| {
            let msg: T::Input = bincode::deserialize(&data)
                .expect("can't deserialize an input message");
            let upd = AgentUpdate::Input(msg);
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
    type Input: Message;
    /// Outgoing message type.
    type Output;

    /// Creates an instance of an agent.
    fn create(link: AgentLink<Self>) -> Self;

    /// This metthod called on every update message.
    fn update(&mut self, msg: Self::Message);

    /// This metthod called on every incoming message.
    fn handle(&mut self, msg: Self::Input);

    /// Creates an instance of an agent.
    fn destroy(&mut self) { }

}

/// Address of an agent.
pub struct Addr<T> {
    worker: Value,
    _agent: PhantomData<T>,
}

impl<T> Addr<T>
where
    T: Agent,
{
    /// Send a message to an agent.
    pub fn send(&self, msg: T::Input) {
        // TODO Important! Implement.
        // Use a queue to collect a messages if an instance is not ready
        // and send them to an agent when it will reported readiness.
        let bytes = bincode::serialize(&msg)
            .expect("can't serialize message for agent");
        let msg: Vec<u8> = ToWorker::ProcessInput(bytes).into();
        let worker = &self.worker;
        js! {
            var worker = @{worker};
            var bytes = @{msg};
            console.log("Sending...", bytes);
            worker.postMessage(bytes);
        };
    }
}

impl<T> Drop for Addr<T> {
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
        let data: Vec<u8> = msg.into();
        js! {
            var data = @{data};
            self.postMessage(data);
        };
    };
    let mut handler = None;
    let handshake = move |data: Vec<u8>| {
        let msg = data.into();
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
            ToWorker::ProcessInput(data) => {
                let func = handler.as_mut()
                    .expect("TypeId of agent was not selected.");
                func(data);
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
    scheduler: Scheduler<()>, // TODO Use thread-local `Scheduler`
}

impl<AGN: Agent> Clone for AgentScope<AGN> {
    fn clone(&self) -> Self {
        AgentScope {
            shared_agent: self.shared_agent.clone(),
            scheduler: self.scheduler.clone(),
        }
    }
}

impl<AGN: Agent> AgentScope<AGN> {
    fn new() -> Self {
        let shared_agent = Rc::new(RefCell::new(AgentRunnable::new()));
        let scheduler = Scheduler::new(());
        AgentScope { shared_agent, scheduler }
    }

    fn send(&self, update: AgentUpdate<AGN>) {
        let envelope = AgentEnvelope {
            shared_agent: self.shared_agent.clone(),
            message: Some(update),
        };
        let runnable: Box<Runnable<()>> = Box::new(envelope);
        self.scheduler.put_and_try_run(runnable);
    }
}

/// Link to agent scope for creating callbacks.
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

    /// This method sends messages back to the component's loop.
    pub fn send_back<F, IN>(&mut self, function: F) -> Callback<IN>
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
    Input(AGN::Input),
    Destroy,
}

struct AgentEnvelope<AGN: Agent> {
    shared_agent: Shared<AgentRunnable<AGN>>,
    message: Option<AgentUpdate<AGN>>,
}

impl<AGN> Runnable<()> for AgentEnvelope<AGN>
where
    AGN: Agent,
{
    fn run<'a>(&mut self, context: &mut ()) {
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
            AgentUpdate::Input(inp) => {
                this.agent.as_mut()
                    .expect("agent was not created to process inputs")
                    .handle(inp);
            }
            AgentUpdate::Destroy => {
                let mut agent = this.agent.take()
                    .expect("trying to destroy not existent agent");
                agent.destroy();
            }
        }
    }
}
