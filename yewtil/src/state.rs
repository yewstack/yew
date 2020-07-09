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
    STATE: Clone + Default + 'static,
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
    pub fn reduce(&self, f: impl FnOnce(&mut STATE) + 'static) {
        self.cb_reduce.emit(Box::new(f))
    }

    ///
    pub fn reduce_callback<T: 'static>(
        &self,
        f: impl Fn(T, &mut STATE) + Copy + 'static,
    ) -> Callback<T> {
        self.cb_reduce
            .reform(move |e| Box::new(move |state| f(e, state)))
    }

    pub fn get(&self) -> &STATE {
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
                reduce(Rc::make_mut(&mut self.state));
                for who in self.subscriptions.iter().cloned() {
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

/// Provides shared state to isolated components.
pub struct SharedStateComponent<STATE, COMP>
where
    COMP: Component,
    COMP::Properties: SharedState<State = STATE>,
    STATE: Default + Clone + 'static,
{
    _mark: std::marker::PhantomData<COMP>,
    props: COMP::Properties,
    bridge: Box<dyn Bridge<SharedStateService<STATE>>>,
    state: Rc<STATE>,
    cb_reduce: Callback<Reduction<STATE>>,
}

pub enum SharedStateComponentMsg<STATE> {
    /// Recieve new local state.
    /// IMPORTANT: Changes will **not** be reflected in shared state.
    SetLocal(Rc<STATE>),
    /// Update shared state.
    Apply(Reduction<STATE>),
}

impl<STATE, COMP> Component for SharedStateComponent<STATE, COMP>
where
    COMP: Component,
    COMP::Properties: SharedState<State = STATE>,
    STATE: Default + Clone,
{
    type Message = SharedStateComponentMsg<STATE>;
    type Properties = COMP::Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        use SharedStateComponentMsg::*;
        // Bridge to receive new state.
        let mut bridge = SharedStateService::bridge(link.callback(|msg| match msg {
            Response::State(state) => SetLocal(state),
        }));
        // Make sure we receive updates to state.
        bridge.send(Request::Subscribe);

        SharedStateComponent {
            _mark: Default::default(),
            props,
            bridge,
            state: Default::default(),
            cb_reduce: link.callback(|reduce| Apply(reduce)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use SharedStateComponentMsg::*;
        match msg {
            Apply(reduce) => {
                self.bridge.send(Request::Apply(reduce));
                false
            }
            SetLocal(state) => {
                self.state = state;
                true
            }
        }
    }

    // FIXME Render selectively here
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // TODO Is cloning here necessary?
        let mut props = self.props.clone();
        let local = props.shared_state();
        local.state = self.state.clone();
        local.cb_reduce = self.cb_reduce.clone();

        html! {
            <COMP with props />
        }
    }
}

impl<STATE, COMP> std::ops::Drop for SharedStateComponent<STATE, COMP>
where
    COMP: Component,
    COMP::Properties: SharedState<State = STATE>,
    STATE: Clone + Default,
{
    fn drop(&mut self) {
        self.bridge.send(Request::UnSubscribe);
    }
}
