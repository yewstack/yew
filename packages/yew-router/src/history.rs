use std::borrow::Cow;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::{Rc, Weak};

use gloo::events::EventListener;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Event;
use yew::callback::Callback;
use yew::utils::window;

use crate::utils::base_url;
use crate::Routable;

/// A History Listener to manage callbacks registered on [`History`].
///
/// This Listener has the same behaviour as the [`EventListener`] from [`gloo`]
/// that the underlying callback will be unregistered when the listener dropped.
pub struct HistoryListener {
    _listener: Rc<Callback<()>>,
}

#[derive(Debug)]
pub enum SerializeError {
    Bindgen(serde_wasm_bindgen::Error),
    Urlencoded(serde_urlencoded::ser::Error),
}

impl fmt::Display for SerializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bindgen(e) => e.fmt(f),
            Self::Urlencoded(e) => e.fmt(f),
        }
    }
}

impl Error for SerializeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bindgen(ref e) => Some(e),
            Self::Urlencoded(ref e) => Some(e),
        }
    }
}

/// A trait to provide [`History`] access.
pub trait History: Clone + PartialEq {
    type Location: Location<History = Self> + 'static;

    /// Returns the number of elements in [`History`].
    fn len(&self) -> usize;

    /// Returns true if the current [`History`] is empty/
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Moves back 1 page in [`History`].
    fn back(&self);

    /// Moves forward 1 page in [`History`].
    fn forward(&self);

    /// Loads a specific page in [`History`] with a `delta` relative to current page.
    ///
    /// See: https://developer.mozilla.org/en-US/docs/Web/API/History/go
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
    fn push_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static;

    /// Replaces the current history entry with provided [`Routable`] and state.
    ///
    /// The implementation of state serialization differs between [`History`] types.
    ///
    /// For [`BrowserHistory`], it uses [`serde_wasm_bindgen`] where as other types uses
    /// [`Any`](std::any::Any).
    fn replace_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static;

    /// Same as [`.push()`] but affix the queries to the end of the route.
    fn push_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize;

    /// Same as [`.replace()`] but affix the queries to the end of the route.
    fn replace_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize;

    /// Same as [`.push_with_state()`] but affix the queries to the end of the route.
    fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> Result<(), SerializeError>
    where
        Q: Serialize,
        T: Serialize + 'static;

    /// Same as [`.replace_with_state()`] but affix the queries to the end of the route.
    fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> Result<(), SerializeError>
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

    /// Returns the State or [`None`] if it fails to deserialize.
    ///
    /// The implementation differs between [`History`] type.
    ///
    /// For [`BrowserHistory`], it uses [`serde_wasm_bindgen`] where as other types uses
    /// [`downcast_ref()`](std::any::Any::downcast_ref).
    fn state<T>(&self) -> Option<T>
    where
        T: DeserializeOwned + 'static;
}

/// A [`History`] that is implemented with [`web_sys::History`] that provides native browser
/// history and state access.
#[derive(Clone)]
pub struct BrowserHistory {
    _listener: Option<Rc<EventListener>>,
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

    fn back(&self) {
        self.inner.back().expect_throw("failed to go back.")
    }

    fn forward(&self) {
        self.inner.forward().expect_throw("failed to go forward.")
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

        Self::dispatch_event();
    }

    fn replace(&self, route: impl Routable) {
        let url = Self::route_to_url(route);
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&url))
            .expect("failed to replace history.");

        Self::dispatch_event();
    }

    fn push_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .push_state_with_url(&state, "", Some(&url))
            .expect("failed to push state.");

        Self::dispatch_event();
        Ok(())
    }

    fn replace_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let state = serde_wasm_bindgen::to_value(&state)?;
        self.inner
            .replace_state_with_url(&state, "", Some(&url))
            .expect("failed to replace state.");

        Self::dispatch_event();
        Ok(())
    }

    fn push_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .push_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to push history.");

        Self::dispatch_event();
        Ok(())
    }
    fn replace_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to replace history.");

        Self::dispatch_event();
        Ok(())
    }

    fn push_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> Result<(), SerializeError>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query).map_err(SerializeError::Urlencoded)?;
        let state = serde_wasm_bindgen::to_value(&state).map_err(SerializeError::Bindgen)?;
        self.inner
            .push_state_with_url(&state, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to push history.");

        Self::dispatch_event();
        Ok(())
    }

    fn replace_with_query_and_state<Q, T>(
        &self,
        route: impl Routable,
        query: Q,
        state: T,
    ) -> Result<(), SerializeError>
    where
        Q: Serialize,
        T: Serialize + 'static,
    {
        let url = Self::route_to_url(route);
        let query = serde_urlencoded::to_string(query).map_err(SerializeError::Urlencoded)?;
        let state = serde_wasm_bindgen::to_value(&state).map_err(SerializeError::Bindgen)?;
        self.inner
            .replace_state_with_url(&state, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to replace history.");

        Self::dispatch_event();
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

    fn state<T>(&self) -> Option<T>
    where
        T: DeserializeOwned + 'static,
    {
        serde_wasm_bindgen::from_value(self.inner.state().expect_throw("failed to read state."))
            .ok()
    }
}

impl Default for BrowserHistory {
    fn default() -> Self {
        // We create browser history only once.
        thread_local! {
            static BROWSER_HISTORY: RefCell<Option<BrowserHistory>> = RefCell::default();
        }

        BROWSER_HISTORY.with(|m| {
            let mut m = m.borrow_mut();

            let history = match *m {
                Some(ref m) => m.clone(),
                None => {
                    let window = window();

                    let inner = window.history().expect_throw("failed to get history.");
                    let callbacks = Rc::default();

                    let mut self_ = Self {
                        _listener: None,
                        inner,
                        callbacks,
                    };

                    let callbacks = Rc::downgrade(&self_.callbacks);

                    self_._listener = Some(Rc::new(EventListener::new(
                        &window,
                        "popstate",
                        move |_| {
                            if let Some(m) = callbacks.upgrade() {
                                Self::notify_callbacks(m)
                            }
                        },
                    )));
                    self_
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

    // Callbacks are dispatched as events so all browser history can acknowledge a state change.
    fn dispatch_event() {
        let event = Event::new("popstate").unwrap();
        yew::utils::window()
            .dispatch_event(&event)
            .expect("dispatch");
    }

    fn route_to_url(route: impl Routable) -> Cow<'static, str> {
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

    fn notify_callbacks(callbacks: Rc<RefCell<Vec<Weak<Callback<()>>>>>) {
        let callables = {
            let mut callbacks_ref = callbacks.borrow_mut();

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
    // /// Sets the `pathname` on the [`Location`] struct.
    // fn set_pathname(&self, pathname: &str);

    /// Returns the queries of current URL in [`String`]
    fn search(&self) -> String;
    // /// Sets the `search` string in [`String`].
    // fn set_search(&self, search: &str);

    /// Returns the queries of current URL parsed as `T`.
    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned;
    // /// Sets the queries of current URL to serialized value of `T`.
    // fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    // where
    //     T: Serialize;

    /// Returns the hash fragment of current URL.
    fn hash(&self) -> String;
    // /// Sets the hash fragment of current URL.
    // fn set_hash(&self, hash: &str);

    /// Returns current route or `None` if none matched.
    fn route<R>(&self) -> Option<R>
    where
        R: Routable;
}

/// The [`Location`] type for [`BrowserHistory`].
///
/// Most functionality of this type is provided by [`web_sys::Location`].
///
/// This type also provides additional methods that are unique in Browsers and are not available in [`Location`].
// ///
// /// Note: most setters(`set_*`) that is unique to [`BrowserLocation`] will cause the page to reload.
#[derive(Clone)]
pub struct BrowserLocation {
    location: web_sys::Location,
    _history: BrowserHistory,
}

impl PartialEq for BrowserLocation {
    fn eq(&self, _rhs: &Self) -> bool {
        // All browser locations are created equal.
        true
    }
}

impl Location for BrowserLocation {
    type History = BrowserHistory;

    fn pathname(&self) -> String {
        self.location
            .pathname()
            .expect_throw("failed to get pathname.")
    }

    // fn set_pathname(&self, pathname: &str) {
    //     self.location
    //         .set_pathname(pathname)
    //         .expect_throw("failed to set pathname.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    fn search(&self) -> String {
        self.location.search().expect_throw("failed to get search.")
    }

    // fn set_search(&self, search: &str) {
    //     self.location
    //         .set_search(search)
    //         .expect_throw("failed to set search.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned,
    {
        let query = self.search();
        serde_urlencoded::from_str(query.strip_prefix('?').unwrap_or(""))
    }

    // fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    // where
    //     T: Serialize,
    // {
    //     let query = serde_urlencoded::to_string(query)?;
    //     self.set_search(&query);
    //     Ok(())
    // }

    fn hash(&self) -> String {
        self.location.hash().expect_throw("failed to get hash.")
    }

    // fn set_hash(&self, hash: &str) {
    //     self.location
    //         .set_hash(hash)
    //         .expect_throw("failed to set hash.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

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
            location: window().location(),
            _history: history,
        }
    }

    /// Returns the [`href`] of current [`Location`].
    pub fn href(&self) -> String {
        self.location.href().expect_throw("failed to get href.")
    }

    // /// Sets the [`href`] property of current [`Location`].
    // pub fn set_href(&self, value: &str) {
    //     self.location
    //         .set_href(value)
    //         .expect_throw("failed to set href.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    /// Returns the [`origin`] of current [`Location`].
    pub fn origin(&self) -> String {
        self.location.origin().expect_throw("failed to get origin.")
    }

    /// Returns the [`protocol`] property of current [`Location`].
    pub fn protocol(&self) -> String {
        self.location
            .protocol()
            .expect_throw("failed to get protocol.")
    }

    // /// Sets the [`protocol`] property of current [`Location`].
    // pub fn set_protocol(&self, value: &str) {
    //     self.location
    //         .set_protocol(value)
    //         .expect_throw("failed to set protocol.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    /// Returns the [`host`] of current [`Location`].
    pub fn host(&self) -> String {
        self.location.host().expect_throw("failed to get host.")
    }

    // /// Sets the [`host`] property of current [`Location`].
    // pub fn set_host(&self, value: &str) {
    //     self.location
    //         .set_host(value)
    //         .expect_throw("failed to set host.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    /// Returns the [`hostname`] of current [`Location`].
    pub fn hostname(&self) -> String {
        self.location
            .hostname()
            .expect_throw("failed to get hostname.")
    }

    // /// Sets the [`hostname`] property of current [`Location`].
    // pub fn set_hostname(&self, value: &str) {
    //     self.location
    //         .set_hostname(value)
    //         .expect_throw("failed to set hostname.");
    //     BrowserHistory::<R>::dispatch_event();
    // }

    // /// Reloads the page.
    // pub fn reload(&self) {
    //     self.location.reload().expect_throw("failed to reload.");
    //     BrowserHistory::<R>::dispatch_event();
    // }
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

    fn back(&self) {
        let Self::Browser(self_) = self;
        self_.back()
    }

    fn forward(&self) {
        let Self::Browser(self_) = self;
        self_.forward()
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

    fn push_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.push_with_state(route, state)
    }

    fn replace_with_state<T>(
        &self,
        route: impl Routable,
        state: T,
    ) -> Result<(), serde_wasm_bindgen::Error>
    where
        T: Serialize + 'static,
    {
        let Self::Browser(self_) = self;
        self_.replace_with_state(route, state)
    }

    fn push_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.push_with_query(route, query)
    }
    fn replace_with_query<Q>(
        &self,
        route: impl Routable,
        query: Q,
    ) -> Result<(), serde_urlencoded::ser::Error>
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
    ) -> Result<(), SerializeError>
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
    ) -> Result<(), SerializeError>
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

    fn state<T>(&self) -> Option<T>
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

    // fn set_pathname(&self, pathname: &str) {
    //     let Self::Browser(self_) = self;
    //     self_.set_pathname(pathname)
    // }

    fn search(&self) -> String {
        let Self::Browser(self_) = self;
        self_.search()
    }

    // fn set_search(&self, search: &str) {
    //     let Self::Browser(self_) = self;
    //     self_.set_search(search)
    // }

    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned,
    {
        let Self::Browser(self_) = self;
        self_.query()
    }

    // fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    // where
    //     T: Serialize,
    // {
    //     let Self::Browser(self_) = self;
    //     self_.set_query(query)
    // }

    fn hash(&self) -> String {
        let Self::Browser(self_) = self;
        self_.hash()
    }

    // fn set_hash(&self, hash: &str) {
    //     let Self::Browser(self_) = self;
    //     self_.set_hash(hash)
    // }

    fn route<R>(&self) -> Option<R>
    where
        R: Routable,
    {
        let Self::Browser(self_) = self;
        self_.route()
    }
}
