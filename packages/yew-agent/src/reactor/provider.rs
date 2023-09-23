use std::any::type_name;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use gloo_worker::reactor::ReactorScoped;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use super::{Reactor, ReactorBridge, ReactorSpawner};
use crate::utils::get_next_id;
use crate::worker::WorkerProviderProps;
use crate::{Bincode, Codec, Reach};

pub(crate) struct ReactorProviderState<T>
where
    T: Reactor + 'static,
{
    id: usize,
    spawn_bridge_fn: Rc<dyn Fn() -> ReactorBridge<T>>,
    reach: Reach,
    held_bridge: Rc<RefCell<Option<ReactorBridge<T>>>>,
}

impl<T> fmt::Debug for ReactorProviderState<T>
where
    T: Reactor,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(type_name::<Self>())
    }
}

impl<T> ReactorProviderState<T>
where
    T: Reactor,
{
    fn get_held_bridge(&self) -> ReactorBridge<T> {
        let mut held_bridge = self.held_bridge.borrow_mut();

        match held_bridge.as_mut() {
            Some(m) => m.fork(),
            None => {
                let bridge = (self.spawn_bridge_fn)();
                *held_bridge = Some(bridge.fork());
                bridge
            }
        }
    }

    /// Creates a bridge, uses "fork" for public agents.
    pub fn create_bridge(&self) -> ReactorBridge<T> {
        match self.reach {
            Reach::Public => {
                let held_bridge = self.get_held_bridge();
                held_bridge.fork()
            }
            Reach::Private => (self.spawn_bridge_fn)(),
        }
    }
}

impl<T> Clone for ReactorProviderState<T>
where
    T: Reactor,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            spawn_bridge_fn: self.spawn_bridge_fn.clone(),
            reach: self.reach,
            held_bridge: self.held_bridge.clone(),
        }
    }
}

impl<T> PartialEq for ReactorProviderState<T>
where
    T: Reactor,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

/// The Reactor Agent Provider.
///
/// This component provides its children access to a reactor agent.
#[function_component]
pub fn ReactorProvider<R, C = Bincode>(props: &WorkerProviderProps) -> Html
where
    R: 'static + Reactor,
    <<R as Reactor>::Scope as ReactorScoped>::Input:
        Serialize + for<'de> Deserialize<'de> + 'static,
    <<R as Reactor>::Scope as ReactorScoped>::Output:
        Serialize + for<'de> Deserialize<'de> + 'static,
    C: Codec + 'static,
{
    let WorkerProviderProps {
        children,
        path,
        lazy,
        reach,
    } = props.clone();

    // Creates a spawning function so Codec is can be erased from contexts.
    let spawn_bridge_fn: Rc<dyn Fn() -> ReactorBridge<R>> = {
        let path = path.clone();
        Rc::new(move || ReactorSpawner::<R>::new().encoding::<C>().spawn(&path))
    };

    let state = {
        use_memo((path, lazy, reach), move |(_path, lazy, reach)| {
            let state = ReactorProviderState::<R> {
                id: get_next_id(),
                spawn_bridge_fn,
                reach: *reach,
                held_bridge: Rc::default(),
            };

            if *reach == Reach::Public && !*lazy {
                state.get_held_bridge();
            }
            state
        })
    };

    html! {
        <ContextProvider<ReactorProviderState<R>> context={(*state).clone()}>
            {children}
        </ContextProvider<ReactorProviderState<R>>>
    }
}
