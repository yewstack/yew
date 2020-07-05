use std::collections::HashSet;
use std::rc::Rc;

use yew::{
    html,
    worker::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId},
    Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};

type Reduction<STATE> = Box<dyn FnOnce(&mut STATE)>;

/// Interface for implementing properties with shared state.
pub trait SharedState {
    type State: Clone + Default;
    fn shared_state(&mut self) -> &mut Shared<Self::State>;
}

/// Wrapper for accessing shared state.
#[derive(Default, Properties, Clone, PartialEq)]
pub struct Shared<STATE>
where
    STATE: Clone + Default,
{
    #[prop_or_default]
    state: Rc<STATE>,
    #[prop_or_default]
    cb_reduce: Callback<Reduction<STATE>>,
}

impl<STATE> Shared<STATE>
where
    STATE: Clone + Default,
{
    /// Apply changes to state.
    pub fn reduce(&self, reduce: impl FnOnce(&mut STATE) + 'static) {
        self.cb_reduce.emit(Box::new(reduce))
    }

    pub fn get(&self) -> &Rc<STATE> {
        &self.state
    }
}

impl<STATE> SharedState for Shared<STATE>
where
    STATE: Clone + Default,
{
    type State = STATE;

    fn shared_state(&mut self) -> &mut Shared<Self::State> {
        self
    }
}

enum Request<STATE> {
    Apply(Reduction<STATE>),
    Subscribe,
    UnSubscribe,
}

enum Response<STATE> {
    State(Rc<STATE>),
}

struct SharedStateService<STATE>
where
    STATE: Clone + Default + 'static,
{
    state: Rc<STATE>,
    subscriptions: HashSet<HandlerId>,
    link: AgentLink<SharedStateService<STATE>>,
}

impl<STATE> Agent for SharedStateService<STATE>
where
    STATE: Clone + Default + 'static,
{
    type Message = ();
    type Reach = Context<Self>;
    type Input = Request<STATE>;
    type Output = Response<STATE>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            state: Default::default(),
            subscriptions: Default::default(),
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Apply(reduce) => {
                self.apply(reduce);
                // Will be notified if subscribed, only send here if it isn't.
                if !self.subscriptions.contains(&who) {
                    self.link.respond(who, Response::State(self.state.clone()));
                }
            }
            Request::Subscribe => {
                self.subscriptions.insert(who);
                self.link.respond(who, Response::State(self.state.clone()));
            }
            Request::UnSubscribe => {
                self.subscriptions.remove(&who);
            }
        }
    }
}

impl<STATE> SharedStateService<STATE>
where
    STATE: Default + Clone,
{
    fn apply(&mut self, reduce: impl FnOnce(&mut STATE)) {
        reduce(Rc::make_mut(&mut self.state));
        for who in self.subscriptions.iter().cloned() {
            self.link.respond(who, Response::State(self.state.clone()));
        }
    }
}

/// Provides shared state to isolated components.
pub struct SharedStateComponent<STATE, COMP, PROPS>
where
    COMP: Component<Properties = PROPS>,
    STATE: Default + Clone + 'static,
    PROPS: Properties + SharedState<State = STATE>,
{
    props: PROPS,
    state: Rc<STATE>,
    cb_reduce: Callback<Reduction<STATE>>,
    bridge: Box<dyn Bridge<SharedStateService<STATE>>>,
    _mark: std::marker::PhantomData<COMP>,
}

pub enum SharedStateComponentMsg<STATE> {
    /// Recieve new local state.
    /// IMPORTANT: Changes will **not** be reflected in shared state.
    SetLocalState(Rc<STATE>),
    /// Update shared state.
    Apply(Reduction<STATE>),
}

impl<STATE, COMP, PROPS> Component for SharedStateComponent<STATE, COMP, PROPS>
where
    COMP: Component<Properties = PROPS>,
    STATE: Default + Clone,
    PROPS: Properties + SharedState<State = STATE> + 'static,
{
    type Message = SharedStateComponentMsg<STATE>;
    type Properties = PROPS;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        use SharedStateComponentMsg::*;

        let mut bridge = SharedStateService::bridge(link.callback(|msg| match msg {
            Response::State(state) => SetLocalState(state),
        }));
        bridge.send(Request::Subscribe);

        SharedStateComponent {
            props,
            bridge,
            state: Default::default(),
            cb_reduce: link.callback(|reduce| Apply(reduce)),
            _mark: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use SharedStateComponentMsg::*;
        match msg {
            Apply(reduce) => {
                self.bridge.send(Request::Apply(reduce));
                false
            }
            SetLocalState(state) => {
                self.state = state;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let mut props = self.props.clone();
        let local = props.shared_state();
        local.state = self.state.clone();
        local.cb_reduce = self.cb_reduce.clone();

        html! {
            <COMP with props />
        }
    }
}

impl<STATE, COMP, PROPS> std::ops::Drop for SharedStateComponent<STATE, COMP, PROPS>
where
    COMP: Component<Properties = PROPS>,
    STATE: Clone + Default,
    PROPS: Properties + SharedState<State = STATE>,
{
    fn drop(&mut self) {
        self.bridge.send(Request::UnSubscribe);
    }
}
