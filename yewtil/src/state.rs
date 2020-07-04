use std::collections::HashSet;
use std::rc::Rc;

use yew::{
    html, worker::*, Callback, Children, Component, ComponentLink, Html, Properties, ShouldRender,
};

type Setter<STATE> = Box<dyn FnOnce(&mut STATE)>;

enum Request<STATE> {
    SetState(Setter<STATE>),
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
    link: AgentLink<SharedStateService<STATE>>,
    state: Rc<STATE>,
    subscriptions: HashSet<HandlerId>,
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
            Request::SetState(setter) => {
                self.set_state(setter);
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
    fn set_state(&mut self, setter: impl FnOnce(&mut STATE)) {
        setter(Rc::make_mut(&mut self.state));
        for who in self.subscriptions.iter() {
            self.link
                .respond(who.clone(), Response::State(self.state.clone()));
        }
    }
}

/// Provides shared state to isolated components.
pub struct SharedState<STATE, COMP>
where
    COMP: Component<Properties = Shared<STATE>>,
    STATE: Default + Clone + 'static,
{
    state: Rc<STATE>,
    state_service: Box<dyn Bridge<SharedStateService<STATE>>>,
    callback_setter: Callback<Setter<STATE>>,
    props: SharedStateProps,
    _mark: std::marker::PhantomData<COMP>,
}

pub enum SharedStateMsg<STATE> {
    /// Recieve new local state. IMPORTANT: Changes will **not** be reflected in shared state.
    SetStateLocal(Rc<STATE>),
    /// Update shared state.
    SetState(Setter<STATE>),
}

#[derive(Properties, Clone)]
pub struct SharedStateProps {
    #[prop_or_default]
    children: Children,
}

impl<STATE, COMP> Component for SharedState<STATE, COMP>
where
    COMP: Component<Properties = Shared<STATE>>,
    STATE: Default + Clone,
{
    type Message = SharedStateMsg<STATE>;
    type Properties = SharedStateProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut state_service = SharedStateService::bridge(link.callback(|msg| match msg {
            Response::State(state) => SharedStateMsg::SetStateLocal(state),
        }));
        state_service.send(Request::Subscribe);

        SharedState {
            props,
            state_service,
            state: Default::default(),
            callback_setter: link.callback(|setter| SharedStateMsg::SetState(setter)),
            _mark: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SharedStateMsg::SetState(setter) => {
                self.state_service.send(Request::SetState(setter));
                false
            }
            SharedStateMsg::SetStateLocal(state) => {
                self.state = state;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let props = Shared {
            state: self.state.clone(),
            callback_setter: self.callback_setter.clone(),
            children: self.props.children.clone(),
        };
        html! {
            <COMP with props />
        }
    }
}

impl<STATE, COMP> std::ops::Drop for SharedState<STATE, COMP>
where
    COMP: Component<Properties = Shared<STATE>>,
    STATE: Clone + Default,
{
    fn drop(&mut self) {
        self.state_service.send(Request::UnSubscribe);
    }
}

#[derive(Default, Properties, Clone, PartialEq)]
pub struct Shared<STATE>
where
    STATE: Clone,
{
    state: Rc<STATE>,
    callback_setter: Callback<Setter<STATE>>,
    pub children: Children,
}

impl<STATE> Shared<STATE>
where
    STATE: Clone,
{
    pub fn set_with(&self, setter: impl FnOnce(&mut STATE) + 'static) {
        self.callback_setter.emit(Box::new(setter))
    }

    pub fn get(&self) -> Rc<STATE> {
        self.state.clone()
    }
}
