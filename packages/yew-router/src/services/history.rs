use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::base_url;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::EventTarget;
use yew::utils::window;
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

struct HistoryState {
    last_route: Rc<Route>,
    subscribers: Vec<Callback<Rc<Route>>>,
    cb: Closure<dyn Fn()>,
}

thread_local! {
    static HISTORY_STATE: RefCell<Option<HistoryState>> = RefCell::new(None);
}

/// Service to interface with the history API.
#[derive(Debug)]
pub struct HistoryService;

impl HistoryState {
    fn with<R>(f: impl FnOnce(&mut Option<Self>) -> R) -> R {
        HISTORY_STATE.with(|state| f(&mut *state.borrow_mut()))
    }
    fn determine_current_route() -> Route {
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

    fn new() -> Self {
        // Install event listeners
        let cb: Closure<dyn Fn()> = Closure::wrap(Box::new(Self::update));

        let et: EventTarget = window().into();
        et.add_event_listener_with_callback("popstate", cb.as_ref().unchecked_ref())
            .expect("add popstate listener");
        et.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref())
            .expect("add hashchange listener");

        Self {
            last_route: Rc::new(Self::determine_current_route()),
            subscribers: Vec::new(),
            cb,
        }
    }

    // We sometimes return a function to run when the state is not borrowed.
    // This is so that callbacks don't panic if they try to access the state.
    fn update_inner(maybe_state: &mut Option<Self>) -> Option<impl FnOnce() + 'static> {
        let state = maybe_state.as_mut()?;
        if state.subscribers.is_empty() {
            *maybe_state = None;
            None
        } else {
            let route = Rc::new(Self::determine_current_route());
            if state.last_route != route {
                state.last_route = route.clone();
                let subscribers = state.subscribers.clone();
                Some(move || {
                    for subscriber in subscribers {
                        subscriber.emit(route.clone());
                    }
                })
            } else {
                None
            }
        }
    }

    fn update() {
        if let Some(f) = Self::with(Self::update_inner) {
            f();
        }
    }

    fn current_route(maybe_state: &mut Option<Self>) -> Rc<Route> {
        maybe_state.get_or_insert_with(Self::new).last_route.clone()
    }

    fn register(maybe_state: &mut Option<Self>, callback: Callback<Rc<Route>>) -> HistoryListener {
        maybe_state
            .get_or_insert_with(Self::new)
            .subscribers
            .push(callback.clone());
        HistoryListener(callback)
    }

    fn unregister(maybe_state: &mut Option<Self>, callback: &Callback<Rc<Route>>) {
        if let Some(state) = maybe_state {
            if let Some(index) = state
                .subscribers
                .iter()
                .position(|subscriber| subscriber == callback)
            {
                state.subscribers.remove(index);
            }
        }
    }
}

impl Drop for HistoryState {
    fn drop(&mut self) {
        let et: EventTarget = window().into();
        et.remove_event_listener_with_callback("popstate", self.cb.as_ref().unchecked_ref())
            .expect("remove popstate listener");
        et.remove_event_listener_with_callback("hashchange", self.cb.as_ref().unchecked_ref())
            .expect("remove hashchange listener");
    }
}

pub struct HistoryListener(Callback<Rc<Route>>);

impl Drop for HistoryListener {
    fn drop(&mut self) {
        HistoryState::with(|state| HistoryState::unregister(state, &self.0));
    }
}

impl HistoryService {
    pub fn dispatch(action: HistoryAction) {
        let history = window().history().expect("no history");
        match action {
            HistoryAction::Push(mut route) => {
                route.apply_base();
                history
                    .push_state_with_url(&route.state, "", Some(&route.url()))
                    .expect("push history");

                // Not triggered automatically by `pushState`.
                HistoryState::update();
            }
            HistoryAction::Replace(mut route) => {
                route.apply_base();
                history
                    .replace_state_with_url(&route.state, "", Some(&route.url()))
                    .expect("replace history");

                // Not triggered automatically by `replaceState`.
                HistoryState::update();
            }
            HistoryAction::Back => history.back().expect("back history"),
            HistoryAction::Forward => history.forward().expect("forward history"),
            HistoryAction::Go(index) => history.go_with_delta(index).expect("go history"),
        }
    }
    pub fn push(route: Route) {
        Self::dispatch(HistoryAction::Push(route));
    }
    pub fn replace(route: Route) {
        Self::dispatch(HistoryAction::Replace(route));
    }
    pub fn forward() {
        Self::dispatch(HistoryAction::Forward);
    }
    pub fn back() {
        Self::dispatch(HistoryAction::Back);
    }
    pub fn go(index: i32) {
        Self::dispatch(HistoryAction::Go(index));
    }
    pub fn current() -> Rc<Route> {
        HistoryState::with(HistoryState::current_route)
    }
    pub fn register(callback: Callback<Rc<Route>>) -> HistoryListener {
        HistoryState::with(|state| HistoryState::register(state, callback))
    }
}
