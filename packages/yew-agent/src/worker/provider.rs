use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use yew::prelude::*;

use super::{Bridge, Spawnable, Worker};
use crate::reach::Reach;

/// Properties for [WorkerProvider].
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct WorkerProviderProps {
    /// The path to an agent.
    pub path: AttrValue,

    /// The reachability of an agent.
    reach: Reach,

    /// Lazily spawn the agent.
    ///
    /// The agent will be spawned when the first time a hook requests a bridge.
    ///
    /// Does not effect private agents.
    ///
    /// Default: `true`
    #[prop_or(true)]
    pub lazy: bool,

    /// Children of the provider.
    pub children: Children,
}

pub(super) struct WorkerProviderState<W>
where
    W: Worker,
{
    ctr: usize,
    path: AttrValue,
    reach: Reach,
    lazy: bool,
    held_bridge: Rc<RefCell<Option<Bridge<W>>>>,
}

impl<W> WorkerProviderState<W>
where
    W: Worker,
{
    /// Creates a bridge, uses "fork" for public
    pub fn create_bridge<F>(&self, cb: F) -> Bridge<W>
    where
        F: 'static + Fn(W::Output),
    {
        match self.reach {
            Reach::Public => {
                let mut held_bridge = self.held_bridge.borrow_mut();

                match held_bridge.as_mut() {
                    Some(m) => m.fork(Some(cb)),
                    None => {
                        let new_held_bridge = W::spawner().spawn(&self.path);
                        let bridge = new_held_bridge.fork(Some(cb));

                        *held_bridge = Some(new_held_bridge);
                        bridge
                    }
                }
            }
            Reach::Private => W::spawner().callback(cb).spawn(&self.path),
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
            path: self.path.clone(),
            reach: self.reach,
            lazy: self.lazy,
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

/// A Public Agent Provider.
///
/// This component provides its children access to a public agent.
#[function_component]
pub fn WorkerProvider<W>(props: &WorkerProviderProps) -> Html
where
    W: Worker,
{
    let WorkerProviderProps {
        children,
        path,
        lazy,
        reach,
    } = props.clone();

    let state = use_memo(
        |(path, lazy, reach)| {
            let ctr = CTR.fetch_add(1, Ordering::SeqCst);

            let held_bridge = if props.reach == Reach::Public && !props.lazy {
                Rc::new(RefCell::new(Some(W::spawner().spawn(&props.path))))
            } else {
                Rc::default()
            };

            WorkerProviderState::<W> {
                ctr,
                path: path.clone(),
                lazy: *lazy,
                reach: *reach,
                held_bridge,
            }
        },
        (path, lazy, reach),
    );

    html! {
        <ContextProvider<WorkerProviderState<W>> context={(*state).clone()}>
            {children}
        </ContextProvider<WorkerProviderState<W>>>
    }
}
