//! Globally shared state between (potentially) isolated components.
//!
//! GlobalHandle state is accomplished through a component wrapper [SharedStateComponent](struct.SharedStateComponent.html),
//! which take any component who's properties implements [SharedState](trait.SharedState.html).
//! GlobalHandle state can then be handled normally, like any other properties.
use std::collections::HashSet;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use yew::{
    format::Json,
    html,
    services::{storage::Area, StorageService},
    worker::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId},
    Callback, Children, Component, ComponentLink, Html, Properties, ShouldRender,
};

type Reduction<T> = Box<dyn FnOnce(&mut T)>;

/// Defines how state should be created, modified, and shared.
pub trait StateHandler {
    type Model;

    fn new() -> Self;
    fn apply(&mut self, f: Reduction<Self::Model>);
    fn state(&self) -> Rc<Self::Model>;
}

/// Default handler for shared state.
#[derive(Default)]
pub struct GlobalStateHandler<T> {
    state: Rc<T>,
}

impl<T> StateHandler for GlobalStateHandler<T>
where
    T: Clone + Default,
{
    type Model = T;

    fn new() -> Self {
        Default::default()
    }

    fn apply(&mut self, f: Reduction<Self::Model>) {
        f(Rc::make_mut(&mut self.state));
    }

    fn state(&self) -> Rc<Self::Model> {
        Rc::clone(&self.state)
    }
}

/// The storage key for saving state to the current session. Used by `SessionStateHandler`.
pub trait SessionStorageKey {
    fn key() -> &'static str;
}

/// A state handler that syncs any changes to the current session.
#[derive(Default)]
pub struct SessionStateHandler<T> {
    state: Rc<T>,
    storage: Option<StorageService>,
}

impl<T> SessionStateHandler<T>
where
    T: SessionStorageKey + Serialize + DeserializeOwned,
{
    fn load_state(&mut self) {
        if let Some(Json(Ok(state))) = self.storage.as_mut().map(|s| s.restore(T::key())) {
            self.state = state;
            log::trace!("Loaded state from storage");
        } else {
            log::error!("Error loading shared state from session storage");
        }
    }

    fn save_state(&mut self) {
        if let Some(storage) = &mut self.storage {
            storage.store(T::key(), Json(&self.state));
            log::trace!("Done saving storage");
        } else {
            log::error!("Error saving shared state to session storage");
        }
    }
}

impl<T> StateHandler for SessionStateHandler<T>
where
    T: Default + Clone + SessionStorageKey + Serialize + DeserializeOwned,
{
    type Model = T;

    fn new() -> Self {
        let mut this: Self = Default::default();
        this.storage = StorageService::new(Area::Session)
            .map_err(|err| {
                log::error!("Error accessing session storage: {:?}", err);
            })
            .ok();
        this.load_state();
        this
    }

    fn apply(&mut self, f: Reduction<Self::Model>) {
        f(Rc::make_mut(&mut self.state));
        self.save_state();
    }

    fn state(&self) -> Rc<Self::Model> {
        Rc::clone(&self.state)
    }
}

type HandlerModel<T> = <T as StateHandler>::Model;

pub trait SharedState {
    type Handle: SharedHandle;
    fn handle(&mut self) -> &mut Self::Handle;
}

pub trait SharedHandle {
    type Handler: StateHandler;

    fn state(&self) -> &HandlerModel<Self::Handler>;
    fn callback(&self) -> &Callback<Reduction<HandlerModel<Self::Handler>>>;
    fn __set_local(
        &mut self,
        state: &Rc<HandlerModel<Self::Handler>>,
        callback: &Callback<Reduction<HandlerModel<Self::Handler>>>,
    );

    /// Apply a function that may mutate shared state. Changes are not immediate, and must be handled
    /// in the component's `change` method (like any other properties).
    fn reduce(&self, f: impl FnOnce(&mut HandlerModel<Self::Handler>) + 'static) {
        self.callback().emit(Box::new(f))
    }

    /// Convenience method for modifying shared state directly from a callback.
    /// ## Example
    /// ```
    /// html! {
    ///   <input
    ///     type="button"
    ///     value="Clear"
    ///     onclick = self.state.reduce_callback(|state|  state.user.name.clear())
    ///     />
    /// }
    /// ```
    fn reduce_callback<T: 'static>(
        &self,
        f: impl FnOnce(&mut HandlerModel<Self::Handler>) + Copy + 'static,
    ) -> Callback<T>
    where
        HandlerModel<Self::Handler>: 'static,
    {
        self.callback()
            .reform(move |_| Box::new(move |state| f(state)))
    }

    /// Convenience method for modifying shared state directly from a callback. Similar to `reduce_callback`
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
    fn reduce_callback_with<T: 'static>(
        &self,
        f: impl FnOnce(T, &mut HandlerModel<Self::Handler>) + Copy + 'static,
    ) -> Callback<T>
    where
        HandlerModel<Self::Handler>: 'static,
    {
        self.callback()
            .reform(move |e| Box::new(move |state| f(e, state)))
    }
}

#[derive(Default)]
pub struct StateHandle<T, H>
where
    T: 'static,
    H: StateHandler,
{
    state: Rc<T>,
    callback: Callback<Reduction<T>>,
    _mark: std::marker::PhantomData<H>,
}

impl<T, H> Clone for StateHandle<T, H>
where
    T: Clone + 'static,
    H: StateHandler,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            callback: self.callback.clone(),
            _mark: Default::default(),
        }
    }
}

impl<T, H> PartialEq for StateHandle<T, H>
where
    T: PartialEq + Clone + 'static,
    H: StateHandler,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.callback == other.callback
    }
}

impl<T, H> SharedHandle for StateHandle<T, H>
where
    T: Default + Clone,
    H: StateHandler<Model = T>,
{
    type Handler = H;

    fn callback(&self) -> &Callback<Reduction<T>> {
        &self.callback
    }

    fn state(&self) -> &T {
        &self.state
    }

    #[doc(hidden)]
    fn __set_local(&mut self, state: &Rc<T>, callback: &Callback<Reduction<T>>) {
        self.state = state.clone();
        self.callback = callback.clone();
    }
}

pub type GlobalHandle<T> = StateHandle<T, GlobalStateHandler<T>>;
pub type SessionHandle<T> = StateHandle<T, SessionStateHandler<T>>;

#[derive(Clone, Properties, PartialEq)]
pub struct GlobalProps<T>
where
    T: Default + Clone + 'static,
{
    #[prop_or_default]
    pub handle: GlobalHandle<T>,
    #[prop_or_default]
    pub children: Children,
}

impl<T> SharedState for GlobalProps<T>
where
    T: Clone + Default,
{
    type Handle = GlobalHandle<T>;

    fn handle(&mut self) -> &mut Self::Handle {
        &mut self.handle
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct SessionProps<T>
where
    T: Default + Clone + 'static,
    T: Default + Clone + Serialize + DeserializeOwned + SessionStorageKey + 'static,
{
    #[prop_or_default]
    pub handle: SessionHandle<T>,
    #[prop_or_default]
    pub children: Children,
}

impl<T> SharedState for SessionProps<T>
where
    T: Clone + Default + Serialize + DeserializeOwned + SessionStorageKey,
{
    type Handle = SessionHandle<T>;

    fn handle(&mut self) -> &mut Self::Handle {
        &mut self.handle
    }
}

enum Request<T> {
    /// Apply a state change.
    Apply(Reduction<T>),
    /// Subscribe to be notified when state changes.
    Subscribe,
    /// Remove subscription.
    UnSubscribe,
}

enum Response<T> {
    /// Update subscribers with current state.
    State(Rc<T>),
}

type SharedStateHandler<T> = <<T as SharedState>::Handle as SharedHandle>::Handler;
type SharedStateModel<T> =
    <<<T as SharedState>::Handle as SharedHandle>::Handler as StateHandler>::Model;

/// Context agent for managing shared state. In charge of applying changes to state then notifying
/// subscribers of new state.
struct SharedStateService<T>
where
    T: SharedState + Clone + 'static,
{
    handler: SharedStateHandler<T>,
    subscriptions: HashSet<HandlerId>,
    link: AgentLink<SharedStateService<T>>,
}

impl<T> Agent for SharedStateService<T>
where
    T: SharedState + Clone + 'static,
{
    type Message = ();
    type Reach = Context<Self>;
    type Input = Request<SharedStateModel<T>>;
    type Output = Response<SharedStateModel<T>>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            handler: SharedStateHandler::<T>::new(),
            subscriptions: Default::default(),
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) {}

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            Request::Apply(reduce) => {
                self.handler.apply(reduce);
                for who in self.subscriptions.iter().cloned() {
                    self.link
                        .respond(who, Response::State(self.handler.state()));
                }
            }
            Request::Subscribe => {
                self.subscriptions.insert(who);
                self.link
                    .respond(who, Response::State(self.handler.state()));
            }
            Request::UnSubscribe => {
                self.subscriptions.remove(&who);
            }
        }
    }
}

/// Wrapper for a component with shared state.
///
/// Manages the [GlobalHandle](struct.GlobalHandle.html) field of its wrapped component's
/// [SharedState](trait.SharedState.html) properties.
/// ```
/// pub struct Model {
///     state: GlobalHandle<AppState>,
/// }
///
/// impl Component for Model {
///     type Properties = GlobalHandle<AppState>;
///     ...
/// }
///
/// pub type MyComponent = SharedStateComponent<Model>;
/// ```
pub struct SharedStateComponent<C>
where
    C: Component,
    C::Properties: SharedState + Clone,
{
    props: C::Properties,
    bridge: Box<dyn Bridge<SharedStateService<C::Properties>>>,
    state: Rc<SharedStateModel<C::Properties>>,
    callback: Callback<Reduction<SharedStateModel<C::Properties>>>,
}

/// Internal use only.
pub enum SharedStateComponentMsg<T> {
    /// Recieve new local state.
    /// IMPORTANT: Changes will **not** be reflected in shared state.
    SetLocal(Rc<T>),
    /// Update shared state.
    Apply(Reduction<T>),
}

impl<C> Component for SharedStateComponent<C>
where
    C: Component,
    C::Properties: SharedState + Clone,
    SharedStateModel<C::Properties>: Default,
{
    type Message = SharedStateComponentMsg<SharedStateModel<C::Properties>>;
    type Properties = C::Properties;

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
            callback: link.callback(Apply),
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
        props.handle().__set_local(&self.state, &self.callback);

        html! {
            <C with props />
        }
    }
}

impl<C> std::ops::Drop for SharedStateComponent<C>
where
    C: Component,
    C::Properties: SharedState + Clone,
{
    fn drop(&mut self) {
        self.bridge.send(Request::UnSubscribe);
    }
}
