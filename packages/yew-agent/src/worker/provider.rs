use std::any::type_name;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use gloo_worker::Spawnable;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use super::{Worker, WorkerBridge};
use crate::reach::Reach;
use crate::utils::get_next_id;
use crate::{Bincode, Codec};

/// Properties for [WorkerProvider].
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct WorkerProviderProps {
    /// The path to an agent.
    pub path: AttrValue,

    /// The reachability of an agent.
    ///
    /// Default: [`Public`](Reach::Public).
    #[prop_or(Reach::Public)]
    pub reach: Reach,

    /// Lazily spawn the agent.
    ///
    /// The agent will be spawned when the first time a hook requests a bridge.
    ///
    /// Does not affect private agents.
    ///
    /// Default: `true`
    #[prop_or(true)]
    pub lazy: bool,

    /// Children of the provider.
    #[prop_or_default]
    pub children: Html,
}

pub(crate) struct WorkerProviderState<W>
where
    W: Worker,
{
    id: usize,
    spawn_bridge_fn: Rc<dyn Fn() -> WorkerBridge<W>>,
    reach: Reach,
    held_bridge: RefCell<Option<Rc<WorkerBridge<W>>>>,
}

impl<W> fmt::Debug for WorkerProviderState<W>
where
    W: Worker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(type_name::<Self>())
    }
}

impl<W> WorkerProviderState<W>
where
    W: Worker,
    W::Output: 'static,
{
    fn get_held_bridge(&self) -> Rc<WorkerBridge<W>> {
        let mut held_bridge = self.held_bridge.borrow_mut();

        match held_bridge.as_mut() {
            Some(m) => m.clone(),
            None => {
                let bridge = Rc::new((self.spawn_bridge_fn)());
                *held_bridge = Some(bridge.clone());
                bridge
            }
        }
    }

    /// Creates a bridge, uses "fork" for public agents.
    pub fn create_bridge(&self, cb: Callback<W::Output>) -> WorkerBridge<W> {
        match self.reach {
            Reach::Public => {
                let held_bridge = self.get_held_bridge();
                held_bridge.fork(Some(move |m| cb.emit(m)))
            }
            Reach::Private => (self.spawn_bridge_fn)(),
        }
    }
}

impl<W> PartialEq for WorkerProviderState<W>
where
    W: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.id == rhs.id
    }
}

/// The Worker Agent Provider.
///
/// This component provides its children access to a worker agent.
#[function_component]
pub fn WorkerProvider<W, C = Bincode>(props: &WorkerProviderProps) -> Html
where
    W: Worker + 'static,
    W::Input: Serialize + for<'de> Deserialize<'de> + 'static,
    W::Output: Serialize + for<'de> Deserialize<'de> + 'static,
    C: Codec + 'static,
{
    let WorkerProviderProps {
        children,
        path,
        lazy,
        reach,
    } = props.clone();

    // Creates a spawning function so Codec is can be erased from contexts.
    let spawn_bridge_fn: Rc<dyn Fn() -> WorkerBridge<W>> = {
        let path = path.clone();
        Rc::new(move || W::spawner().encoding::<C>().spawn(&path))
    };

    let state = {
        use_memo((path, lazy, reach), move |(_path, lazy, reach)| {
            let state = WorkerProviderState::<W> {
                id: get_next_id(),
                spawn_bridge_fn,
                reach: *reach,
                held_bridge: Default::default(),
            };

            if *reach == Reach::Public && !*lazy {
                state.get_held_bridge();
            }
            state
        })
    };

    html! {
        <ContextProvider<Rc<WorkerProviderState<W>>> context={state.clone()}>
            {children}
        </ContextProvider<Rc<WorkerProviderState<W>>>>
    }
}
