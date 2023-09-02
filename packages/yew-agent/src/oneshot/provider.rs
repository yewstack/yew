use core::fmt;
use std::any::type_name;
use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::prelude::*;

use super::{Oneshot, OneshotBridge, OneshotSpawner};
use crate::utils::get_next_id;
use crate::worker::WorkerProviderProps;
use crate::{Bincode, Codec, Reach};

pub(crate) struct OneshotProviderState<T>
where
    T: Oneshot + 'static,
{
    id: usize,
    spawn_bridge_fn: Rc<dyn Fn() -> OneshotBridge<T>>,
    reach: Reach,
    held_bridge: Rc<RefCell<Option<OneshotBridge<T>>>>,
}

impl<T> fmt::Debug for OneshotProviderState<T>
where
    T: Oneshot,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(type_name::<Self>())
    }
}

impl<T> OneshotProviderState<T>
where
    T: Oneshot,
{
    fn get_held_bridge(&self) -> OneshotBridge<T> {
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
    pub fn create_bridge(&self) -> OneshotBridge<T> {
        match self.reach {
            Reach::Public => {
                let held_bridge = self.get_held_bridge();
                held_bridge.fork()
            }
            Reach::Private => (self.spawn_bridge_fn)(),
        }
    }
}

impl<T> Clone for OneshotProviderState<T>
where
    T: Oneshot,
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

impl<T> PartialEq for OneshotProviderState<T>
where
    T: Oneshot,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

/// The Oneshot Agent Provider.
///
/// This component provides its children access to an oneshot agent.
#[function_component]
pub fn OneshotProvider<T, C = Bincode>(props: &WorkerProviderProps) -> Html
where
    T: Oneshot + 'static,
    T::Input: Serialize + for<'de> Deserialize<'de> + 'static,
    T::Output: Serialize + for<'de> Deserialize<'de> + 'static,
    C: Codec + 'static,
{
    let WorkerProviderProps {
        children,
        path,
        lazy,
        reach,
    } = props.clone();

    // Creates a spawning function so Codec is can be erased from contexts.
    let spawn_bridge_fn: Rc<dyn Fn() -> OneshotBridge<T>> = {
        let path = path.clone();
        Rc::new(move || OneshotSpawner::<T>::new().encoding::<C>().spawn(&path))
    };

    let state = {
        use_memo((path, lazy, reach), move |(_path, lazy, reach)| {
            let state = OneshotProviderState::<T> {
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
        <ContextProvider<OneshotProviderState<T>> context={(*state).clone()}>
            {children}
        </ContextProvider<OneshotProviderState<T>>>
    }
}
