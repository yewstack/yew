use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use gloo::worker::Spawnable;
use yew::prelude::*;

use super::{Worker, WorkerBridge};
use crate::reach::Reach;
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
    pub children: Children,
}

pub(crate) struct WorkerProviderState<W>
where
    W: Worker,
{
    ctr: usize,
    spawn_bridge_fn: Rc<dyn Fn() -> WorkerBridge<W>>,
    reach: Reach,
    held_bridge: Rc<RefCell<Option<WorkerBridge<W>>>>,
}

impl<W> fmt::Debug for WorkerProviderState<W>
where
    W: Worker,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WorkerProviderState<_>")
    }
}

impl<W> WorkerProviderState<W>
where
    W: Worker,
{
    fn get_held_bridge(&self) -> WorkerBridge<W> {
        let mut held_bridge = self.held_bridge.borrow_mut();

        match held_bridge.as_mut() {
            Some(m) => m.clone(),
            None => {
                let bridge = (self.spawn_bridge_fn)();
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

impl<W> Clone for WorkerProviderState<W>
where
    W: Worker,
{
    fn clone(&self) -> Self {
        Self {
            ctr: self.ctr,
            spawn_bridge_fn: self.spawn_bridge_fn.clone(),
            reach: self.reach,
            held_bridge: self.held_bridge.clone(),
        }
    }
}

impl<W> PartialEq for WorkerProviderState<W>
where
    W: Worker,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.ctr == rhs.ctr
    }
}

static CTR: AtomicUsize = AtomicUsize::new(0);

/// A Worker Agent Provider.
///
/// This component provides its children access to an worker agent.
#[function_component]
pub fn WorkerProvider<W, CODEC = Bincode>(props: &WorkerProviderProps) -> Html
where
    W: Worker + 'static,
    CODEC: Codec + 'static,
{
    // Creates a spawning function so CODEC is can be erased from contexts.
    let spawn_bridge_fn: Rc<dyn Fn() -> WorkerBridge<W>> = {
        let path = props.path.clone();
        Rc::new(move || W::spawner().encoding::<CODEC>().spawn(&path))
    };

    let WorkerProviderProps {
        children,
        path,
        lazy,
        reach,
    } = props.clone();

    let state = {
        use_memo(
            move |(_path, lazy, reach)| {
                let ctr = CTR.fetch_add(1, Ordering::SeqCst);

                let state = WorkerProviderState::<W> {
                    ctr,
                    spawn_bridge_fn,
                    reach: *reach,
                    held_bridge: Rc::default(),
                };

                if *reach == Reach::Public && !*lazy {
                    state.get_held_bridge();
                }
                state
            },
            (path, lazy, reach),
        )
    };

    html! {
        <ContextProvider<WorkerProviderState<W>> context={(*state).clone()}>
            {children}
        </ContextProvider<WorkerProviderState<W>>>
    }
}
