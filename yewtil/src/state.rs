//! Globally shared state between (potentially) isolated components.
//!
//! Shared state is accomplished through a component wrapper [SharedStateComponent](struct.SharedStateComponent.html),
//! which take any component who's properties implements [SharedState](trait.SharedState.html).
//! Shared state can then be handled normally, like any other properties.
use std::collections::HashSet;
use std::rc::Rc;

use yew::{
    html,
    worker::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId},
    Callback, Component, ComponentLink, Html, Properties, ShouldRender,
};

type Reduction<STATE> = Box<dyn FnOnce(&mut STATE)>;

/// Defines the local [Shared](struct.Shared.html) member to be managed by a [SharedStateComponent](struct.SharedStateComponent.html).
///
/// ## Usage
/// ```
/// #[derive(Clone, Default, PartialEq)]
/// struct AppState {
///     // ...
/// }
///
/// #[derive(Clone, Properties)]
/// pub struct Props {
///     #[prop_or_default]
///     pub state: Shared<AppState>,
/// }
///
/// impl SharedState for Props {
///     type State = AppState;
///
///     fn shared_state(&mut self) -> &mut Shared<Self::State> {
///         &mut self.state
///     }
/// }
/// ```
pub trait SharedState {
    type State: Clone + Default;
    /// Return a mutable reference to the `Shared` member.
    fn shared_state(&mut self) -> &mut Shared<Self::State>;
}

/// Handle for accessing shared state.
///
/// `Properties` is implemented for convenience. It may also be used inside other properties that
/// implement [SharedState](trait.SharedState.html).
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
    /// Apply a function that may mutate shared state. Changes are not immediate, and must be handled
    /// in the component's `change` method (like any other properties).
    pub fn reduce(&self, f: impl FnOnce(&mut STATE) + 'static) {
        self.cb_reduce.emit(Box::new(f))
    }

    /// Convenience method for modifying shared state directly from a callback. Similar to `reduce`
    /// but it also accepts the fired event.
    /// ## Example
    /// ```
    /// html! {
    ///   <input
    ///     type="button"
    ///     value="Clear"
    ///     onclick = self.state.reduce_callback(|_, state|  state.user.name.clear())
    ///     />
    /// }
    /// ```
    pub fn reduce_callback<T: 'static>(
        &self,
        f: impl FnOnce(T, &mut STATE) + Copy + 'static,
    ) -> Callback<T> {
        self.cb_reduce
            .reform(move |e| Box::new(move |state| f(e, state)))
    }

    /// Get current state.
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
    /// Apply a state change.
    Apply(Reduction<STATE>),
    /// Subscribe to be notified when state changes.
    Subscribe,
    /// Remove subscription.
    UnSubscribe,
}

enum Response<STATE> {
    /// Update subscribers with current state.
    State(Rc<STATE>),
}

/// Context agent for managing shared state. In charge of applying changes to state then notifying
/// subscribers of new state.
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

/// Wrapper for a component with shared state.
///
/// Manages the [Shared](struct.Shared.html) field of its wrapped component's
/// [SharedState](trait.SharedState.html) properties.
/// ```
/// pub struct Model {
///     state: Shared<AppState>,
/// }
///
/// impl Component for Model {
///     type Properties = Shared<AppState>;
///     ...
/// }
///
/// pub type MyComponent = SharedStateComponent<Model>;
/// ```
pub struct SharedStateComponent<COMP>
where
    COMP: Component,
    COMP::Properties: SharedState,
{
    props: COMP::Properties,
    bridge: Box<dyn Bridge<SharedStateService<<COMP::Properties as SharedState>::State>>>,
    state: Rc<<COMP::Properties as SharedState>::State>,
    cb_reduce: Callback<Reduction<<COMP::Properties as SharedState>::State>>,
}

/// Internal use only.
pub enum SharedStateComponentMsg<STATE> {
    /// Recieve new local state.
    /// IMPORTANT: Changes will **not** be reflected in shared state.
    SetLocal(Rc<STATE>),
    /// Update shared state.
    Apply(Reduction<STATE>),
}

impl<COMP> Component for SharedStateComponent<COMP>
where
    COMP: Component,
    COMP::Properties: SharedState,
{
    type Message = SharedStateComponentMsg<<COMP::Properties as SharedState>::State>;
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
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

impl<COMP> std::ops::Drop for SharedStateComponent<COMP>
where
    COMP: Component,
    COMP::Properties: SharedState,
{
    fn drop(&mut self) {
        self.bridge.send(Request::UnSubscribe);
    }
}
