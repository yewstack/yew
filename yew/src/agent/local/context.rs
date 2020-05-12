use super::*;
use crate::callback::Callback;
use crate::scheduler::Shared;
use anymap::{self, AnyMap};
use slab::Slab;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

thread_local! {
    static LOCAL_AGENTS_POOL: RefCell<AnyMap> = RefCell::new(AnyMap::new());
}

/// Create a single instance in the current thread.
#[allow(missing_debug_implementations)]
pub struct Context<AGN> {
    _agent: PhantomData<AGN>,
}

impl<AGN> Discoverer for Context<AGN>
where
    AGN: Agent,
{
    type Agent = AGN;

    fn spawn_or_join(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
        let mut scope_to_init = None;
        let bridge = LOCAL_AGENTS_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            match pool.entry::<LocalAgent<AGN>>() {
                anymap::Entry::Occupied(mut entry) => entry.get_mut().create_bridge(callback),
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

struct SlabResponder<AGN: Agent> {
    slab: Shared<Slab<Option<Callback<AGN::Output>>>>,
}

impl<AGN: Agent> Responder<AGN> for SlabResponder<AGN> {
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        locate_callback_and_respond::<AGN>(&self.slab, id, output);
    }
}

impl<AGN: Agent> Dispatchable for Context<AGN> {}

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

struct LocalAgent<AGN: Agent> {
    scope: AgentScope<AGN>,
    slab: SharedOutputSlab<AGN>,
}

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
