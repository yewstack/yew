use super::*;
use crate::scheduler::Shared;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{HashSet};

/// A functional state wrapper, enforcing a unidirectional
/// data flow and consistent state to the observers.
///
/// `handle_input` receives incoming messages from components,
/// `reduce` applies changes to the state
pub trait Stateful: Sized + 'static {
    /// TODO
    type Message;
    /// TODO
    type Input;

    /// TODO
    fn new() -> Self;

    /// Receives messages from components and other agents. Use the `link`
    /// to send messages to itself in order to notify `reduce` once your
    /// operation completes. This is the place to do side effects, like
    /// talking to the server, or asking the user for input.
    ///
    /// Note that you can look at the state of your Stateful, but you
    /// cannot modify it here. If you want to modify it, send a Message
    /// to the reducer
    fn handle_input(&self, link: AgentLink<StatefulWrapper<Self>>, msg: Self::Input);

    /// A pure function, with no side effects. Receives a message,
    /// and applies it to the state as it sees fit.
    fn reduce(&mut self, msg: Self::Message);

    /// TODO
    fn destroy(&mut self) {}
}

/// TODO
#[derive(Debug)]
pub struct StatefulWrapper<S: Stateful> {
    /// TODO
    pub handlers: HashSet<HandlerId>,
    /// TODO
    pub link: AgentLink<Self>,

    /// TODO
    pub state: Shared<S>,

    /// TODO
    pub self_dispatcher: Dispatcher<Self>
}

/// TODO
impl<S: Stateful> Agent for StatefulWrapper<S> {
    type Reach = Context<Self>;
    type Message = S::Message;
    type Input = S::Input;
    type Output = Shared<S>;

    fn create(link: AgentLink<Self>) -> Self {
        let state = Rc::new(RefCell::new(S::new()));
        let handlers = HashSet::new();

        // Link to self to never go out of scope
        let self_dispatcher = Self::dispatcher();

        StatefulWrapper {
            handlers,
            state,
            link,
            self_dispatcher
        }
    }

    fn update(&mut self, msg: Self::Message) {
        {
            self.state.borrow_mut().reduce(msg);
        }

        for handler in self.handlers.iter() {
            // Is there a way of sharing a RefCell that cannot have a mutable
            // reference taken? That would be ideal
            self.link.respond(*handler, self.state.clone());
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        self.state.borrow().handle_input(self.link.clone(), msg);
    }

    fn connected(&mut self, id: HandlerId) {
        self.handlers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.handlers.remove(&id);
    }
}

// Those instances are quite unfortunate, as the Rust compiler
// does not support mutually exclusive trait bounds (https://github.com/rust-lang/rust/issues/51774),
// we have to create new traits with the same functions as in the original traits.
// I want to at the very least rename those to some better names,
// preferably get rid of them if you know the way to do it.

/// TODO
pub trait Dispatchable: Sized + 'static {
    /// TODO
    type RefAgent: Agent;

    fn dispatcher() -> Dispatcher<Self::RefAgent>;
}


/// TODO
impl<T> Dispatchable for T
where T: Stateful
{
    /// TODO
    type RefAgent = StatefulWrapper<T>;

    fn dispatcher() -> Dispatcher<Self::RefAgent> {
        Dispatcher(<Self::RefAgent as Agent>::Reach::spawn_or_join(None))
    }
}

/// TODO
pub trait Bridgeable: Sized + 'static {
    /// TODO
    type RefAgent: Agent;

    /// Creates a messaging bridge between a worker and the component.
    fn bridge(callback: Callback<<Self::RefAgent as Agent>::Output>) -> Box<dyn Bridge<Self::RefAgent>>;
}

/// TODO
impl<T> Bridgeable for T
where T: Stateful
{
    /// TODO
    type RefAgent = StatefulWrapper<T>;

    fn bridge(callback: Callback<<Self::RefAgent as Agent>::Output>) -> Box<dyn Bridge<Self::RefAgent>> {
        <Self::RefAgent as Agent>::Reach::spawn_or_join(Some(callback))
    }
}
