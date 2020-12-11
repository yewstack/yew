use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use std::rc::Rc;
use yew::agent::{Agent, AgentLink, Context, Discoverer, Dispatcher, HandlerId};
use yew::prelude::*;

/// A functional state wrapper, enforcing a unidirectional
/// data flow and consistent state to the observers.
///
/// `handle_input` receives incoming messages from components,
/// `reduce` applies changes to the state
///
/// The state is sent once whenever a bridge is opened and then once
/// for each `Action` sent by the `handle_input` function. This means
/// the initial state of the store must be valid for the consumers.
///
/// Once created with a first bridge, a Store will never be destroyed
/// for the lifetime of the application.
pub trait Store: Sized + 'static {
    /// Messages instructing the store to do somethin
    type Input;
    /// State updates to be consumed by `reduce`
    type Action;

    /// Create a new Store
    fn new() -> Self;

    /// Receives messages from components and other agents. Use the `link`
    /// to send actions to itself in order to notify `reduce` once your
    /// operation completes. This is the place to do side effects, like
    /// talking to the server, or asking the user for input.
    ///
    /// Note that you can look at the state of your Store, but you
    /// cannot modify it here. If you want to modify it, send a Message
    /// to the reducer
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input);

    /// A pure function, with no side effects. Receives a message,
    /// and applies it to the state as it sees fit.
    fn reduce(&mut self, msg: Self::Action);
}

/// Hides the full context Agent from a Store and does
/// the boring data wrangling logic
#[derive(Debug)]
pub struct StoreWrapper<S: Store> {
    /// Currently subscribed components and agents
    pub handlers: HashSet<HandlerId>,
    /// Link to itself so Store::handle_input can send actions to reducer
    pub link: AgentLink<Self>,

    /// The actual Store
    pub state: Shared<S>,

    /// A circular dispatcher to itself so the store is not removed
    pub self_dispatcher: Dispatcher<Self>,
}

type Shared<T> = Rc<RefCell<T>>;

/// A wrapper ensuring state observers can only
/// borrow the state immutably
#[derive(Debug)]
pub struct ReadOnly<S> {
    state: Shared<S>,
}

impl<S> ReadOnly<S> {
    /// Allow only immutable borrows to the underlying data
    pub fn borrow(&self) -> impl Deref<Target = S> + '_ {
        self.state.borrow()
    }
}

/// This is a wrapper, intended to be used as an opaque
/// machinery allowing the Store to do it's things.
impl<S: Store> Agent for StoreWrapper<S> {
    type Reach = Context<Self>;
    type Message = S::Action;
    type Input = S::Input;
    type Output = ReadOnly<S>;

    fn create(link: AgentLink<Self>) -> Self {
        let state = Rc::new(RefCell::new(S::new()));
        let handlers = HashSet::new();

        // Link to self to never go out of scope
        let self_dispatcher = Self::dispatcher();

        StoreWrapper {
            handlers,
            state,
            link,
            self_dispatcher,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        {
            self.state.borrow_mut().reduce(msg);
        }

        for handler in self.handlers.iter() {
            self.link.respond(
                *handler,
                ReadOnly {
                    state: self.state.clone(),
                },
            );
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.handlers.insert(id);
        self.link.respond(
            id,
            ReadOnly {
                state: self.state.clone(),
            },
        );
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        self.state.borrow().handle_input(self.link.clone(), msg);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.handlers.remove(&id);
    }
}

// This instance is quite unfortunate, as the Rust compiler
// does not support mutually exclusive trait bounds (https://github.com/rust-lang/rust/issues/51774),
// we have to create a new trait with the same function as in the original one.

/// Allows us to communicate with a store
pub trait Bridgeable: Sized + 'static {
    /// A wrapper for the store we want to bridge to,
    /// which serves as a communication intermediary
    type Wrapper: Agent;

    /// Creates a messaging bridge between a worker and the component.
    fn bridge(
        callback: Callback<<Self::Wrapper as Agent>::Output>,
    ) -> Box<dyn Bridge<Self::Wrapper>>;
}

/// Implementation of bridge creation
impl<T> Bridgeable for T
where
    T: Store,
{
    /// The hiding wrapper
    type Wrapper = StoreWrapper<T>;

    fn bridge(
        callback: Callback<<Self::Wrapper as Agent>::Output>,
    ) -> Box<dyn Bridge<Self::Wrapper>> {
        <Self::Wrapper as Agent>::Reach::spawn_or_join(Some(callback))
    }
}
