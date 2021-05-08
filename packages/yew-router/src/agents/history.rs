use std::collections::HashSet;
use std::rc::Rc;

use crate::utils::base_url;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::EventTarget;
use yew::utils::window;
use yew::worker::{Agent, AgentLink, Bridged, Context, HandlerId};
use yew::Callback;

#[derive(Debug, Clone, PartialEq)]
pub struct Route {
    pub(crate) path: String,
    pub(crate) query: String,
    pub(crate) hash: String,
    pub(crate) state: JsValue,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            path: Default::default(),
            query: Default::default(),
            hash: Default::default(),
            state: JsValue::UNDEFINED,
        }
    }
}

impl Route {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            query: String::new(),
            hash: String::new(),
            state: JsValue::null(),
        }
    }
    pub fn with_state(mut self, state: JsValue) -> Self {
        self.state = state;
        self
    }
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = query.into();
        self
    }
    pub fn with_hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = hash.into();
        self
    }
    pub fn path(&self) -> &str {
        &self.path
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn hash(&self) -> &str {
        &self.hash
    }
    pub fn state(&self) -> &JsValue {
        &self.state
    }
    pub fn url(&self) -> String {
        format!("{}{}{}", self.path, self.query, self.hash)
    }
    fn apply_base(&mut self) {
        if let Some(base_url) = base_url() {
            if self.path.starts_with("/") {
                if self.path == "/" {
                    self.path = base_url;
                } else {
                    self.path = format!("{}{}", base_url, self.path);
                }
            }
        }
    }
    fn unapply_base(&mut self) {
        if let Some(base_url) = base_url() {
            if let Some(path) = self.path.strip_prefix(&base_url) {
                if path.starts_with("/") {
                    self.path = path.into();
                } else if path.is_empty() {
                    self.path = "/".into();
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum HistoryAction {
    Push(Route),
    Replace(Route),
    Forward,
    Back,
    Go(i32),
}

/// Agent to interface with the history API.
#[derive(Debug)]
pub struct HistoryAgent {
    link: AgentLink<Self>,
    last_route: Rc<Route>,
    cb: Closure<dyn Fn()>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for HistoryAgent {
    type Reach = Context<Self>;

    type Message = ();

    type Input = HistoryAction;

    type Output = Rc<Route>;

    fn create(link: AgentLink<Self>) -> Self {
        let last_route = Rc::new(Self::current());

        let link2 = link.clone();
        let cb: Closure<dyn Fn()> = Closure::wrap(Box::new(move || link2.send_message(())));

        let et: EventTarget = window().into();
        et.add_event_listener_with_callback("popstate", cb.as_ref().unchecked_ref())
            .expect("add popstate listener");
        et.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref())
            .expect("add hashchange listener");

        Self {
            link,
            last_route,
            cb,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, _msg: ()) {
        let route = Rc::new(Self::current());
        if route != self.last_route {
            for &id in &self.subscribers {
                self.link.respond(id, route.clone())
            }
        }
    }

    fn handle_input(&mut self, msg: HistoryAction, _id: yew::worker::HandlerId) {
        let history = window().history().expect("no history");
        match msg {
            HistoryAction::Push(mut route) => {
                route.apply_base();
                history
                    .push_state_with_url(&route.state, "", Some(&route.url()))
                    .expect("push history");
            }
            HistoryAction::Replace(mut route) => {
                route.apply_base();
                history
                    .replace_state_with_url(&route.state, "", Some(&route.url()))
                    .expect("replace history");
            }
            HistoryAction::Back => history.back().expect("back history"),
            HistoryAction::Forward => history.forward().expect("forward history"),
            HistoryAction::Go(index) => history.go_with_delta(index).expect("go history"),
        }

        self.update(());
    }

    fn destroy(&mut self) {
        let et: EventTarget = window().into();
        et.remove_event_listener_with_callback("hashchange", self.cb.as_ref().unchecked_ref())
            .expect("remove hashchange listener");
        et.remove_event_listener_with_callback("popstate", self.cb.as_ref().unchecked_ref())
            .expect("remove popstate listener");
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}

impl HistoryAgent {
    pub fn push(route: Route) {
        Self::bridge(Callback::noop()).send(HistoryAction::Push(route));
    }
    pub fn replace(route: Route) {
        Self::bridge(Callback::noop()).send(HistoryAction::Replace(route));
    }
    pub fn forward() {
        Self::bridge(Callback::noop()).send(HistoryAction::Forward);
    }
    pub fn back() {
        Self::bridge(Callback::noop()).send(HistoryAction::Back);
    }
    pub fn go(index: i32) {
        Self::bridge(Callback::noop()).send(HistoryAction::Go(index));
    }
    pub fn current() -> Route {
        let window = window();
        let history = window.history().expect("no history");
        let location = window.location();
        let path = location.pathname().expect("no pathname");
        let query = location.search().expect("no pathname");
        let hash = location.hash().expect("no pathname");
        let state = history.state().expect("no state");
        let mut res = Route {
            path,
            query,
            hash,
            state,
        };
        res.unapply_base();
        res
    }
}
