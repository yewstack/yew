use std::collections::HashSet;
use std::rc::Rc;

use crate::Routable;
use yew::worker::{Agent, AgentLink, Bridged, Context, HandlerId};
use yew::Bridge;

use super::history::{HistoryAction, HistoryAgent, Route};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RouterAction<T: Routable> {
    Push(T),
    Replace(T),
}

impl<T: Routable> RouterAction<T> {
    pub fn map<U: Routable>(self, f: impl FnOnce(T) -> U) -> RouterAction<U> {
        match self {
            Self::Push(routable) => RouterAction::Push(f(routable)),
            Self::Replace(routable) => RouterAction::Replace(f(routable)),
        }
    }
}

/// Specializes the history API for a particular `Routable` type
pub(crate) struct RouterAgent<T: Routable> {
    link: AgentLink<Self>,
    history: Box<dyn Bridge<HistoryAgent>>,
    subscribers: HashSet<HandlerId>,
}

impl<T: Routable> Agent for RouterAgent<T> {
    type Reach = Context<Self>;
    type Message = Rc<Route>;
    type Input = RouterAction<T>;
    type Output = Rc<T>;

    fn create(link: AgentLink<Self>) -> Self {
        let history = HistoryAgent::bridge(link.callback(|x| x));

        Self {
            link,
            history,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Rc<Route>) {
        match T::from_route(&msg) {
            Ok(routable) => {
                let routable = Rc::new(routable);
                for &id in &self.subscribers {
                    self.link.respond(id, routable.clone())
                }
            }
            Err(routable) => {
                // Redirect to error route
                self.history
                    .send(HistoryAction::Replace(routable.to_route()))
            }
        }
    }

    fn handle_input(&mut self, msg: RouterAction<T>, _id: yew::worker::HandlerId) {
        match msg {
            RouterAction::Push(routable) => {
                self.history.send(HistoryAction::Push(routable.to_route()))
            }
            RouterAction::Replace(routable) => {
                self.history.send(HistoryAction::Push(routable.to_route()))
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl<T: Routable> RouterAgent<T> {
    pub(crate) fn current() -> T {
        match T::from_route(&HistoryAgent::current()) {
            Ok(routable) => routable,
            Err(routable) => routable,
        }
    }
}
