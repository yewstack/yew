use super::*;
use crate::callback::Callback;
use std::marker::PhantomData;

const SINGLETON_ID: HandlerId = HandlerId(0, true);

/// Create an instance in the current thread.
#[allow(missing_debug_implementations)]
pub struct Job<AGN> {
    _agent: PhantomData<AGN>,
}

impl<AGN> Discoverer for Job<AGN>
where
    AGN: Agent,
{
    type Agent = AGN;

    fn spawn_or_join(callback: Option<Callback<AGN::Output>>) -> Box<dyn Bridge<AGN>> {
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

struct CallbackResponder<AGN: Agent> {
    callback: Callback<AGN::Output>,
}

impl<AGN: Agent> Responder<AGN> for CallbackResponder<AGN> {
    fn respond(&self, id: HandlerId, output: AGN::Output) {
        assert_eq!(id.raw_id(), SINGLETON_ID.raw_id());
        self.callback.emit(output);
    }
}
