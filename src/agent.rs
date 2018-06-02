//! This module contains types to support multi-threading in Yew.

use std::any::TypeId;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use bincode;
use stdweb::Value;
use stdweb::unstable::TryInto;
//use callback::Callback;

thread_local! {
    pub(crate) static AGENTS: RefCell<HashMap<TypeId, Box<FnMut(Vec<u8>)>>> =
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

/// Declares the behavior of the agent.
pub trait Agent: Sized + 'static {
    /// Type of an input messagae.
    type Input: Message;
    /// Type of an output message.
    type Output;

    /// Spawns an agent and returns `Addr` of an instance.
    fn spawn() -> Addr<Self> {
        let worker_base = js! {
            // TODO Use relative path. But how?
            var worker = new Worker("main.js");
            return worker;
        };
        let worker = worker_base.clone();
        let send_to_app = move |msg: ToWorker| {
            let bytes = bincode::serialize(&msg)
                .expect("can't serialize message for app");
            let worker = worker.clone();
            js! {
                var worker = @{worker};
                worker.postMessage(@{bytes});
            };
        };
        let routine = move |data: Vec<u8>| {
            let msg: FromWorker = bincode::deserialize(&data)
                .expect("can't deserialize a message from a worker");
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
            var routine = @{routine};
            worker.onmessage = function(event) {
                routine(event.data);
            };
        };
        Addr {
            worker: worker_base,
            _agent: PhantomData,
        }
    }

    /// Executes an agent in the current environment.
    /// Uses in `main` function of a worker.
    fn register() {
        let mut this = Self::create();
        let routine = move |data: Vec<u8>| {
            let msg: Self::Input = bincode::deserialize(&data)
                .expect("can't deserialize an input message");
            this.handle(msg);
        };
        AGENTS.with(move |agents| {
            let type_id = TypeId::of::<Self>();
            agents.borrow_mut().insert(type_id, Box::new(routine));
        });
    }

    /// Creates an instance of an agent.
    fn create() -> Self;

    /// This metthod called on every incoming message.
    fn handle(&mut self, msg: Self::Input);
}

/// Address of an agent.
pub struct Addr<T> {
    worker: Value,
    _agent: PhantomData<T>,
}

impl<T: Agent> Addr<T> {
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
    let routine = move |data: Vec<u8>| {
        let msg = data.into();
        match msg {
            ToWorker::SelectType(raw_type_id) => {
                let type_id: TypeId = unsafe { ::std::mem::transmute(raw_type_id) };
                handler = AGENTS.with(move |agents| {
                    agents.borrow_mut().remove(&type_id)
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
        let routine = @{routine};
        console.log("Mounted...", self);
        self.onmessage = function(event) {
            // TODO Send type_id, but how?
            console.log("Received...", event.data);
            routine(event.data);
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
