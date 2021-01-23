use super::WorkerExt;
use super::*;
use crate::callback::Callback;
use crate::scheduler::Shared;
use anymap::{self, AnyMap};
use queue::Queue;
use slab::Slab;
use std::any::TypeId;
use std::cell::RefCell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use web_sys::Worker;

thread_local! {
    static REMOTE_AGENTS_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
    static QUEUE: Queue<TypeId> = Queue::new();
}

/// Create a single instance in a tab.
#[allow(missing_debug_implementations)]
pub struct Public<AGN> {
    _agent: PhantomData<AGN>,
}

impl<AGN> Discoverer for Public<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    type Agent = AGN;

    fn spawn_or_join(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let bridge = REMOTE_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            match pool.entry::<RemoteAgent<AGN>>() {
                anymap::Entry::Occupied(mut entry) => entry.get_mut().create_bridge(callback),
                anymap::Entry::Vacant(entry) => {
                    let slab: Shared<Slab<Option<Callback<AGN::Output>>>> =
                        Rc::new(RefCell::new(Slab::new()));
                    let handler = {
                        let slab = slab.clone();
                        move |data: Vec<u8>, worker: &Worker| {
                            let msg = FromWorker::<AGN::Output>::unpack(&data);
                            match msg {
                                FromWorker::WorkerLoaded => {
                                    QUEUE.with(|queue| {
                                        queue.insert_loaded_agent(TypeId::of::<AGN>());

                                        if let Some(msgs) =
                                            queue.remove_msg_queue(&TypeId::of::<AGN>())
                                        {
                                            for msg in msgs {
                                                worker.post_message_vec(msg)
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
                    let worker = {
                        let worker = worker_new(name_of_resource, AGN::is_module());
                        let worker_clone = worker.clone();
                        worker.set_onmessage_closure(move |data: Vec<u8>| {
                            handler(data, &worker_clone);
                        });
                        worker
                    };
                    let launched = RemoteAgent::new(worker, slab);
                    entry.insert(launched).create_bridge(callback)
                }
            }
        });
        Box::new(bridge)
    }
}

impl<AGN> Dispatchable for Public<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
}

/// A connection manager for components interaction with workers.
pub struct PublicBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    worker: Worker,
    id: HandlerId,
    _agent: PhantomData<AGN>,
}

impl<AGN> fmt::Debug for PublicBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PublicBridge<_>")
    }
}

impl<AGN> PublicBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    /// Send a message to the worker, queuing the message if necessary
    fn send_message(&self, msg: ToWorker<AGN::Input>) {
        QUEUE.with(|queue| {
            if queue.is_worker_loaded(&TypeId::of::<AGN>()) {
                send_to_remote::<AGN>(&self.worker, msg);
            } else {
                queue.add_msg_to_queue(msg.pack(), TypeId::of::<AGN>());
            }
        });
    }
}

impl<AGN> Bridge<AGN> for PublicBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(self.id, msg);
        self.send_message(msg);
    }
}

impl<AGN> Drop for PublicBridge<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
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
        self.send_message(disconnected);

        if terminate_worker {
            let destroy = ToWorker::Destroy;
            self.send_message(destroy);

            QUEUE.with(|queue| {
                queue.remove_agent(&TypeId::of::<AGN>());
            });
        }
    }
}

struct WorkerResponder {}

impl<AGN> Responder<AGN> for WorkerResponder
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        let msg = FromWorker::ProcessOutput(id, output);
        let data = msg.pack();
        worker_self().post_message_vec(data);
    }
}

impl<AGN> Threaded for AGN
where
    AGN: Agent<Reach = Public<AGN>>,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    fn register() {
        let scope = AgentScope::<AGN>::new();
        let responder = WorkerResponder {};
        let link = AgentLink::connect(&scope, responder);
        let upd = AgentLifecycleEvent::Create(link);
        scope.send(upd);
        let handler = move |data: Vec<u8>| {
            let msg = ToWorker::<AGN::Input>::unpack(&data);
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
                    worker_self().close();
                }
            }
        };
        let loaded: FromWorker<AGN::Output> = FromWorker::WorkerLoaded;
        let loaded = loaded.pack();
        let worker = worker_self();
        worker.set_onmessage_closure(handler);
        worker.post_message_vec(loaded);
    }
}

struct RemoteAgent<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    worker: Worker,
    slab: SharedOutputSlab<AGN>,
}

impl<AGN> RemoteAgent<AGN>
where
    AGN: Agent,
    <AGN as Agent>::Input: Serialize + for<'de> Deserialize<'de>,
    <AGN as Agent>::Output: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(worker: Worker, slab: SharedOutputSlab<AGN>) -> Self {
        RemoteAgent { worker, slab }
    }

    fn create_bridge(&mut self, callback: Option<Callback<AGN::Output>>) -> PublicBridge<AGN> {
        let respondable = callback.is_some();
        let mut slab = self.slab.borrow_mut();
        let id: usize = slab.insert(callback);
        let id = HandlerId::new(id, respondable);
        let bridge = PublicBridge {
            worker: self.worker.clone(),
            id,
            _agent: PhantomData,
        };
        bridge.send_message(ToWorker::Connected(bridge.id));

        bridge
    }

    fn remove_bridge(&mut self, bridge: &PublicBridge<AGN>) -> Last {
        let mut slab = self.slab.borrow_mut();
        let _ = slab.remove(bridge.id.raw_id());
        slab.is_empty()
    }
}
