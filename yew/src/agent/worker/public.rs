use super::*;
use crate::callback::Callback;
use crate::scheduler::Shared;
use anymap::{self, AnyMap};
use cfg_if::cfg_if;
use cfg_match::cfg_match;
use slab::Slab;
use std::any::TypeId;
use std::cell::RefCell;
use std::collections::{hash_map, HashMap, HashSet};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
cfg_if! {
    if #[cfg(feature = "std_web")] {
        use stdweb::Value;
        #[allow(unused_imports)]
        use stdweb::{_js_impl, js};
    } else if #[cfg(feature = "web_sys")] {
        use super::WorkerExt;
        use web_sys::{Worker};
    }
}

thread_local! {
    static REMOTE_AGENTS_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
    static REMOTE_AGENTS_LOADED: RefCell<HashSet<TypeId>> = RefCell::new(HashSet::new());
    static REMOTE_AGENTS_EARLY_MSGS_QUEUE: RefCell<HashMap<TypeId, Vec<Vec<u8>>>> = RefCell::new(HashMap::new());
}

/// Create a single instance in a tab.
#[allow(missing_debug_implementations)]
pub struct Public<AGN> {
    _agent: PhantomData<AGN>
}

impl<AGN: WorkerAgent> Discoverer for Public<AGN>
{
    type Agent = AGN;

    fn spawn_or_join(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>>
    {
        let bridge = REMOTE_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            match pool.entry::<RemoteAgent<AGN>>() {
                anymap::Entry::Occupied(mut entry) => entry.get_mut().create_bridge(callback),
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

impl<AGN: WorkerAgent> Dispatchable for Public<AGN> {}

/// A connection manager for components interaction with workers.
pub struct PublicBridge<AGN: WorkerAgent> {
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    id: HandlerId,
    _agent: PhantomData<AGN>,
}

impl<AGN: WorkerAgent> fmt::Debug for PublicBridge<AGN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PublicBridge<_>")
    }
}

impl<AGN: WorkerAgent> PublicBridge<AGN> {
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

    /// Send a message to the worker, queuing it up if necessary
    fn send_message(&self, msg: ToWorker<AGN::Input>) {
        if self.worker_is_loaded() {
            send_to_remote::<AGN>(&self.worker, msg);
        } else {
            self.msg_to_queue(msg.pack());
        }
    }
}

impl<AGN: WorkerAgent> Bridge<AGN> for PublicBridge<AGN> {
    fn send(&mut self, msg: AGN::Input) {
        let msg = ToWorker::ProcessInput(self.id, msg);
        self.send_message(msg);
    }
}

impl<AGN: WorkerAgent> Drop for PublicBridge<AGN> {
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

            REMOTE_AGENTS_LOADED.with(|loaded| {
                loaded.borrow_mut().remove(&TypeId::of::<AGN>());
            });

            REMOTE_AGENTS_EARLY_MSGS_QUEUE.with(|queue| {
                queue.borrow_mut().remove(&TypeId::of::<AGN>());
            });
        }
    }
}

struct WorkerResponder {}

impl<AGN: WorkerAgent> Responder<AGN> for WorkerResponder {
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

impl<T> Threaded for T
where
    T: WorkerAgent<Reach = Public<T>>,
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

struct RemoteAgent<AGN: WorkerAgent> {
    #[cfg(feature = "std_web")]
    worker: Value,
    #[cfg(feature = "web_sys")]
    worker: Worker,
    slab: SharedOutputSlab<AGN>,
}

impl<AGN: WorkerAgent> RemoteAgent<AGN> {
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