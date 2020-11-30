use super::*;
use crate::callback::Callback;
use crate::scheduler::{scheduler, Runnable, Shared};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Defines communication from Worker to Consumers
pub(crate) trait Responder<AGN: Agent> {
    /// Implementation for communication channel from Worker to Consumers
    fn respond(&self, id: HandlerId, output: AGN::Output);
}

/// Link to agent's scope for creating callbacks.
pub struct AgentLink<AGN: Agent> {
    scope: AgentScope<AGN>,
    responder: Rc<dyn Responder<AGN>>,
}

impl<AGN: Agent> AgentLink<AGN> {
    /// Create link for a scope.
    pub(crate) fn connect<T>(scope: &AgentScope<AGN>, responder: T) -> Self
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

    /// Send a message to the agent
    pub fn send_message<T>(&self, msg: T)
    where
        T: Into<AGN::Message>,
    {
        self.scope.send(AgentLifecycleEvent::Message(msg.into()));
    }

    /// Send an input to self
    pub fn send_input<T>(&self, input: T)
    where
        T: Into<AGN::Input>,
    {
        let handler_id = HandlerId::new(0, false);
        self.scope
            .send(AgentLifecycleEvent::Input(input.into(), handler_id));
    }

    /// Create a callback which will send a message to the agent when invoked.
    pub fn callback<F, IN, M>(&self, function: F) -> Callback<IN>
    where
        M: Into<AGN::Message>,
        F: Fn(IN) -> M + 'static,
    {
        let scope = self.scope.clone();
        let closure = move |input| {
            let output = function(input).into();
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
/// This struct holds a reference to a component and to a global scheduler.
pub(crate) struct AgentScope<AGN: Agent> {
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
pub(crate) enum AgentLifecycleEvent<AGN: Agent> {
    /// Request to create link
    Create(AgentLink<AGN>),
    /// Internal Agent message
    Message(AGN::Message),
    /// Client connected
    Connected(HandlerId),
    /// Received message from Client
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
