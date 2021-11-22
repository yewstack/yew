//! A module that provides universal session history and location information.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use gloo::events::EventListener;
use gloo_utils::window;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use yew::callback::Callback;

use crate::utils::base_url;
use crate::Routable;

/// A History Listener to manage callbacks registered on a [`History`].
///
/// This Listener has the same behaviour as the [`EventListener`] from [`gloo`]
/// that the underlying callback will be unregistered when the listener is dropped.
pub struct HistoryListener {
    _listener: Rc<Callback<()>>,
}

#[derive(Error, Debug)]
pub enum HistoryError {
    #[error("failed to serialize / deserialize state.")]
    State(#[from] serde_wasm_bindgen::Error),
    #[error("failed to serialize query.")]
    QuerySer(#[from] serde_urlencoded::ser::Error),
    #[error("failed to deserialize query.")]
    QueryDe(#[from] serde_urlencoded::de::Error),
}

pub type HistoryResult<T> = std::result::Result<T, HistoryError>;

/// A trait to provide [`History`] access.
pub trait History: Clone + PartialEq {
    type Location: Location<History = Self> + 'static;

    /// Returns the number of elements in [`History`].
    fn len(&self) -> usize;

    /// Returns true if the current [`History`] is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Moves back 1 page in [`History`].
    fn back(&self) {
        self.go(-1);
    }

    /// Moves forward 1 page in [`History`].
    fn forward(&self) {
        self.go(1);
    }

    /// Loads a specific page in [`History`] with a `delta` relative to current page.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/API/History/go>
    fn go(&self, delta: isize);

    /// Pushes a [`Routable`] entry with [`None`] being the state.
    fn push(&self, route: impl Routable);

    /// Replaces the current history entry with provided [`Routable`] and [`None`] state.
    fn replace(&self, route: impl Routable);

    /// Pushes a [`Routable`] entry with state.
    ///
    /// The implementation of state serialization differs between [`History`] types.
    ///
    /// For [`BrowserHistory`], it uses [`serde_wasm_bindgen`] where as other types uses
    /// [`Any`](std::any::Any).
    fn push_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static;

    /// Replaces the current history entry with provided [`Routable`] and state.
    ///
    /// The implementation of state serialization differs between [`History`] types.
    ///
    /// For [`BrowserHistory`], it uses [`serde_wasm_bindgen`] where as other types uses
    /// [`Any`](std::any::Any).
    fn replace_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static;

    /// Same as `.push()` but affix the queries to the end of the route.
    fn push_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize;

    /// Same as `.replace()` but affix the queries to the end of the route.
    fn replace_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize;

    /// Same as `.push_with_state()` but affix the queries to the end of the route.
    fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static;

    /// Same as `.replace_with_state()` but affix the queries to the end of the route.
    fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static;

    /// Creates a Listener that will be notified when current state changes.
    ///
    /// This method returns a [`HistoryListener`] that will automatically unregister the callback
    /// when dropped.
    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static;

    /// Returns the associated [`Location`] of the current history.
    fn location(&self) -> Self::Location;

    fn into_any_history(self) -> AnyHistory;

    /// Returns the State.
    ///
    /// The implementation differs between [`History`] type.
    ///
    /// For [`BrowserHistory`], it uses [`serde_wasm_bindgen`] where as other types uses
    /// `downcast_ref()` on [`Any`](std::any::Any).
    fn state<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned + 'static;
}

/// A [`History`] that is implemented with [`web_sys::History`] that provides native browser
/// history and state access.
#[derive(Clone)]
pub struct BrowserHistory {
    inner: web_sys::History,
    callbacks: Rc<RefCell<Vec<Weak<Callback<()>>>>>,
}

impl PartialEq for BrowserHistory {
    fn eq(&self, _rhs: &Self) -> bool {
        // All browser histories are created equal.
        true
    }
}

impl History for BrowserHistory {
    type Location = BrowserLocation;

    fn len(&self) -> usize {
        self.inner.length().expect_throw("failed to get length.") as usize
    }

    fn go(&self, delta: isize) {
        self.inner
            .go_with_delta(delta as i32)
            .expect_throw("failed to call go.")
    }

    fn push(&self, route: impl Routable) {
        let url = Self::route_to_url(route);
        self.inner
            .push_state_with_url(&JsValue::NULL, "", Some(&url))
            .expect("failed to push state.");

        self.notify_callbacks();
    }

    fn replace(&self, route: impl Routable) {
        let url = Self::route_to_url(route);
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&url))
            .expect("failed to replace history.");

        self.notify_callbacks();
    }

    fn push_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .push_state_with_url(&state, "", Some(&url))
            .expect("failed to push state.");

        self.notify_callbacks();
        Ok(())
    }

    fn replace_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .replace_state_with_url(&state, "", Some(&url))
            .expect("failed to replace state.");

        self.notify_callbacks();
        Ok(())
    }

    fn push_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .push_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to push history.");

        self.notify_callbacks();
        Ok(())
    }
    fn replace_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to replace history.");

        self.notify_callbacks();
        Ok(())
    }

    fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .push_state_with_url(&state, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to push history.");

        self.notify_callbacks();
        Ok(())
    }

    fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .replace_state_with_url(&state, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to replace history.");

        self.notify_callbacks();
        Ok(())
    }

    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static,
    {
        // Callbacks do not receive a copy of [`History`] to prevent reference cycle.
        let cb = Rc::new(Callback::from(move |_| callback()));

        self.callbacks.borrow_mut().push(Rc::downgrade(&cb));

        HistoryListener { _listener: cb }
    }

    fn location(&self) -> Self::Location {
        BrowserLocation::new(self.clone())
    }

    fn into_any_history(self) -> AnyHistory {
        AnyHistory::Browser(self)
    }

    fn state<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned + 'static,
    {
        serde_wasm_bindgen::from_value(self.inner.state().expect_throw("failed to read state."))
            .map_err(|e| e.into())
    }
}

impl Default for BrowserHistory {
    fn default() -> Self {
        // We create browser history only once.
        thread_local! {
            static BROWSER_HISTORY: RefCell<Option<BrowserHistory>> = RefCell::default();
            static LISTENER: RefCell<Option<EventListener>> = RefCell::default();
        }

        BROWSER_HISTORY.with(|m| {
            let mut m = m.borrow_mut();

            let history = match *m {
                Some(ref m) => m.clone(),
                None => {
                    let window = window();

                    let inner = window
                        .history()
                        .expect_throw("Failed to create browser history. Are you using a browser?");
                    let callbacks = Rc::default();

                    let history = Self { inner, callbacks };

                    let history_clone = history.clone();

                    // Listens to popstate.
                    LISTENER.with(move |m| {
                        let mut listener = m.borrow_mut();

                        *listener = Some(EventListener::new(&window, "popstate", move |_| {
                            history_clone.notify_callbacks();
                        }));
                    });

                    history
                }
            };

            *m = Some(history.clone());

            history
        })
    }
}

impl BrowserHistory {
    /// Creates a new [`BrowserHistory`]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn route_to_url(route: impl Routable) -> Cow<'static, str> {
        let base = base_url();
        let url = route.to_path();

        let path = match base {
            Some(base) => {
                let path = format!("{}{}", base, url);
                if path.is_empty() {
                    Cow::from("/")
                } else {
                    path.into()
                }
            }
            None => url.into(),
        };

        path
    }

    fn notify_callbacks(&self) {
        let callables = {
            let mut callbacks_ref = self.callbacks.borrow_mut();

            // Any gone weak references are removed when called.
            let (callbacks, callbacks_weak) = callbacks_ref.iter().cloned().fold(
                (Vec::new(), Vec::new()),
                |(mut callbacks, mut callbacks_weak), m| {
                    if let Some(m_strong) = m.clone().upgrade() {
                        callbacks.push(m_strong);
                        callbacks_weak.push(m);
                    }

                    (callbacks, callbacks_weak)
                },
            );

            *callbacks_ref = callbacks_weak;

            callbacks
        };

        for callback in callables {
            callback.emit(())
        }
    }
}

/// A trait to to provide [`Location`] information.
pub trait Location: Clone + PartialEq {
    type History: History<Location = Self> + 'static;

    /// Returns the `pathname` on the [`Location`] struct.
    fn pathname(&self) -> String;

    /// Returns the queries of current URL in [`String`]
    fn search(&self) -> String;

    /// Returns the queries of current URL parsed as `T`.
    fn query<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned;

    /// Returns the hash fragment of current URL.
    fn hash(&self) -> String;

    /// Returns current route or `None` if none matched.
    fn route<R>(&self) -> Option<R>
    where
        R: Routable;
}

/// The [`Location`] type for [`BrowserHistory`].
///
/// Most functionality of this type is provided by [`web_sys::Location`].
///
/// This type also provides additional methods that are unique to Browsers and are not available in [`Location`].
///
/// This types is read-only as most setters on `window.location` would cause a reload.
#[derive(Clone)]
pub struct BrowserLocation {
    inner: web_sys::Location,
    _history: BrowserHistory,
}

impl PartialEq for BrowserLocation {
    fn eq(&self, rhs: &Self) -> bool {
        self._history == rhs._history
    }
}

impl Location for BrowserLocation {
    type History = BrowserHistory;

    fn pathname(&self) -> String {
        self.inner
            .pathname()
            .expect_throw("failed to get pathname.")
    }

    fn search(&self) -> String {
        self.inner.search().expect_throw("failed to get search.")
    }

    fn query<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned,
    {
        let query = self.search();
        serde_urlencoded::from_str(query.strip_prefix('?').unwrap_or("")).map_err(|e| e.into())
    }

    fn hash(&self) -> String {
        self.inner.hash().expect_throw("failed to get hash.")
    }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable,
    {
        R::recognize(&self.pathname())
    }
}

impl BrowserLocation {
    fn new(history: BrowserHistory) -> Self {
        Self {
            inner: window().location(),
            _history: history,
        }
    }

    /// Returns the `href` of current [`Location`].
    pub fn href(&self) -> String {
        self.inner.href().expect_throw("failed to get href.")
    }

    /// Returns the `origin` of current [`Location`].
    pub fn origin(&self) -> String {
        self.inner.origin().expect_throw("failed to get origin.")
    }

    /// Returns the `protocol` property of current [`Location`].
    pub fn protocol(&self) -> String {
        self.inner
            .protocol()
            .expect_throw("failed to get protocol.")
    }

    /// Returns the `host` of current [`Location`].
    pub fn host(&self) -> String {
        self.inner.host().expect_throw("failed to get host.")
    }

    /// Returns the `hostname` of current [`Location`].
    pub fn hostname(&self) -> String {
        self.inner
            .hostname()
            .expect_throw("failed to get hostname.")
    }
}

/// A [`History`] that is always available under a [`Router`](crate::Router).
#[derive(Clone, PartialEq)]
pub enum AnyHistory {
    Browser(BrowserHistory),
}

/// The [`Location`] for [`AnyHistory`]
#[derive(Clone, PartialEq)]
pub enum AnyLocation {
    Browser(BrowserLocation),
}

impl History for AnyHistory {
    type Location = AnyLocation;

    fn len(&self) -> usize {
        let Self::Browser(self_) = self;
        self_.len()
    }

    fn go(&self, delta: isize) {
        let Self::Browser(self_) = self;
        self_.go(delta)
    }

    fn push(&self, route: impl Routable) {
        let Self::Browser(self_) = self;
        self_.push(route)
    }

    fn replace(&self, route: impl Routable) {
        let Self::Browser(self_) = self;
        self_.replace(route)
    }

    fn push_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.push_with_state(route, state)
    }

    fn replace_with_state<T>(&self, route: impl Routable, state: T) -> HistoryResult<()>
    where
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.replace_with_state(route, state)
    }

    fn push_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.push_with_query(route, query)
    }
    fn replace_with_query<Q>(&self, route: impl Routable, query: Q) -> HistoryResult<()>
    where
        Q: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.replace_with_query(route, query)
    }

    fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.push_with_query_and_state(route, query, state)
    }

    fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> HistoryResult<()>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.replace_with_query_and_state(route, query, state)
    }

    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static,
    {
        let Self::Browser(self_) = self;
        self_.listen(callback)
    }

    fn location(&self) -> Self::Location {
        let Self::Browser(self_) = self;
        AnyLocation::Browser(self_.location())
    }

    fn into_any_history(self) -> AnyHistory {
        self
    }

    fn state<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned + 'static,
    {
        let Self::Browser(self_) = self;
        self_.state()
    }
}

impl Location for AnyLocation {
    type History = AnyHistory;

    fn pathname(&self) -> String {
        let Self::Browser(self_) = self;
        self_.pathname()
    }

    fn search(&self) -> String {
        let Self::Browser(self_) = self;
        self_.search()
    }

    fn query<T>(&self) -> HistoryResult<T>
    where
        T: DeserializeOwned,
    {
        let Self::Browser(self_) = self;
        self_.query()
    }

    fn hash(&self) -> String {
        let Self::Browser(self_) = self;
        self_.hash()
    }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable,
    {
        let Self::Browser(self_) = self;
        self_.route()
    }
}
