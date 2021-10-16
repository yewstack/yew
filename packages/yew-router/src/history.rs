use std::borrow::Cow;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use gloo::events::EventListener;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::Event;

use crate::utils::base_url;
use crate::Routable;
use yew::callback::Callback;
use yew::utils::window;

pub struct HistoryListener {
    _listener: Rc<Callback<()>>,
}

pub trait History<R>: Clone
where
    R: Routable + 'static,
{
    type Location: Location<R, History = Self> + 'static;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn back(&self);

    fn forward(&self);

    fn go(&self, delta: isize);

    fn push(&self, route: R);
    fn replace(&self, route: R);

    fn push_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize;
    fn replace_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize;

    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static;

    fn location(&self) -> Self::Location;
}

#[derive(Clone)]
pub struct BrowserHistory<R>
where
    R: Routable + 'static,
{
    _listener: Option<Rc<EventListener>>,
    inner: web_sys::History,
    callbacks: Rc<RefCell<Vec<Weak<Callback<()>>>>>,
    _phantom: PhantomData<R>,
}

impl<R> History<R> for BrowserHistory<R>
where
    R: Routable + 'static,
{
    type Location = BrowserLocation<R>;

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

    fn push(&self, route: R) {
        let url = Self::route_to_url(&route);
        self.inner
            .push_state_with_url(&JsValue::NULL, "", Some(&url))
            .expect("failed to push state.");

        Self::dispatch_event();
    }

    fn replace(&self, route: R) {
        let url = Self::route_to_url(&route);
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&url))
            .expect("failed to replace history.");

        Self::dispatch_event();
    }

    fn push_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(&route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .push_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to push history.");

        Ok(())
    }
    fn replace_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let url = Self::route_to_url(&route);
        let query = serde_urlencoded::to_string(query)?;
        self.inner
            .replace_state_with_url(&JsValue::NULL, "", Some(&format!("{}?{}", url, query)))
            .expect("failed to replace history.");

        Ok(())
    }

    fn listen<CB>(&self, callback: CB) -> HistoryListener
    where
        CB: Fn() + 'static,
    {
        let cb = Rc::new(Callback::from(move |_| callback()));

        self.callbacks.borrow_mut().push(Rc::downgrade(&cb));

        HistoryListener { _listener: cb }
    }

    fn location(&self) -> Self::Location {
        BrowserLocation::new()
    }
}

impl<R> Default for BrowserHistory<R>
where
    R: Routable + 'static,
{
    fn default() -> Self {
        let window = window();

        let inner = window.history().expect_throw("failed to get history.");
        let callbacks = Rc::default();

        let mut self_ = Self {
            _listener: None,
            inner,
            callbacks,
            _phantom: PhantomData,
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
}

impl<R> BrowserHistory<R>
where
    R: Routable + 'static,
{
    pub fn new() -> Self {
        Self::default()
    }

    fn dispatch_event() {
        let event = Event::new("popstate").unwrap();
        yew::utils::window()
            .dispatch_event(&event)
            .expect("dispatch");
    }

    fn route_to_url(route: &R) -> Cow<'_, str> {
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

pub trait Location<R>: Clone
where
    R: Routable + 'static,
{
    type History: History<R, Location = Self> + 'static;

    fn pathname(&self) -> String;
    fn set_pathname(&self, pathname: &str);
    fn search(&self) -> String;
    fn set_search(&self, search: &str);
    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned;
    fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    where
        T: Serialize;
    fn hash(&self) -> String;

    fn set_hash(&self, hash: &str);

    fn route(&self) -> Option<R>;
}

#[derive(Clone)]
pub struct BrowserLocation<R>
where
    R: Routable + 'static,
{
    location: web_sys::Location,
    _phantom: PhantomData<R>,
}

impl<R> Location<R> for BrowserLocation<R>
where
    R: Routable + 'static,
{
    type History = BrowserHistory<R>;

    fn pathname(&self) -> String {
        self.location
            .pathname()
            .expect_throw("failed to get pathname.")
    }

    fn set_pathname(&self, pathname: &str) {
        self.location
            .set_pathname(pathname)
            .expect_throw("failed to set pathname.");
        BrowserHistory::<R>::dispatch_event();
    }

    fn search(&self) -> String {
        self.location.search().expect_throw("failed to get search.")
    }

    fn set_search(&self, search: &str) {
        self.location
            .set_search(search)
            .expect_throw("failed to set search.");
        BrowserHistory::<R>::dispatch_event();
    }

    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned,
    {
        let query = self.search();
        serde_urlencoded::from_str(query.strip_prefix('?').unwrap_or(""))
    }

    fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    where
        T: Serialize,
    {
        let query = serde_urlencoded::to_string(query)?;
        self.set_search(&query);
        Ok(())
    }

    fn hash(&self) -> String {
        self.location.hash().expect_throw("failed to get hash.")
    }

    fn set_hash(&self, hash: &str) {
        self.location
            .set_hash(hash)
            .expect_throw("failed to set hash.");
        BrowserHistory::<R>::dispatch_event();
    }

    fn route(&self) -> Option<R> {
        R::recognize(&self.pathname())
    }
}

impl<R> BrowserLocation<R>
where
    R: Routable + 'static,
{
    fn new() -> Self {
        Self {
            location: window().location(),
            _phantom: PhantomData,
        }
    }

    pub fn href(&self) -> String {
        self.location.href().expect_throw("failed to get href.")
    }

    pub fn set_href(&self, value: &str) {
        self.location
            .set_href(value)
            .expect_throw("failed to set href.");
        BrowserHistory::<R>::dispatch_event();
    }

    pub fn origin(&self) -> String {
        self.location.origin().expect_throw("failed to get origin.")
    }

    pub fn protocol(&self) -> String {
        self.location
            .protocol()
            .expect_throw("failed to get protocol.")
    }

    pub fn set_protocol(&self, value: &str) {
        self.location
            .set_protocol(value)
            .expect_throw("failed to set protocol.");
        BrowserHistory::<R>::dispatch_event();
    }

    pub fn host(&self) -> String {
        self.location.host().expect_throw("failed to get host.")
    }

    pub fn set_host(&self, value: &str) {
        self.location
            .set_host(value)
            .expect_throw("failed to set host.");
        BrowserHistory::<R>::dispatch_event();
    }

    pub fn hostname(&self) -> String {
        self.location
            .hostname()
            .expect_throw("failed to get hostname.")
    }

    pub fn set_hostname(&self, value: &str) {
        self.location
            .set_hostname(value)
            .expect_throw("failed to set hostname.");
        BrowserHistory::<R>::dispatch_event();
    }

    pub fn reload(&self) {
        self.location.reload().expect_throw("failed to reload.");
        BrowserHistory::<R>::dispatch_event();
    }
}

#[derive(Clone)]
pub enum AnyHistory<R>
where
    R: Routable + 'static,
{
    Browser(BrowserHistory<R>),
}

#[derive(Clone)]
pub enum AnyLocation<R>
where
    R: Routable + 'static,
{
    Browser(BrowserLocation<R>),
}

impl<R> History<R> for AnyHistory<R>
where
    R: Routable + 'static,
{
    type Location = AnyLocation<R>;

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

    fn push(&self, route: R) {
        let Self::Browser(self_) = self;
        self_.push(route)
    }

    fn replace(&self, route: R) {
        let Self::Browser(self_) = self;
        self_.replace(route)
    }

    fn push_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.push_with_query(route, query)
    }
    fn replace_with_query<Q>(&self, route: R, query: Q) -> Result<(), serde_urlencoded::ser::Error>
    where
        Q: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.replace_with_query(route, query)
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
}

impl<R> Location<R> for AnyLocation<R>
where
    R: Routable + 'static,
{
    type History = AnyHistory<R>;

    fn pathname(&self) -> String {
        let Self::Browser(self_) = self;
        self_.pathname()
    }

    fn set_pathname(&self, pathname: &str) {
        let Self::Browser(self_) = self;
        self_.set_pathname(pathname)
    }

    fn search(&self) -> String {
        let Self::Browser(self_) = self;
        self_.search()
    }

    fn set_search(&self, search: &str) {
        let Self::Browser(self_) = self;
        self_.set_search(search)
    }

    fn query<T>(&self) -> Result<T, serde_urlencoded::de::Error>
    where
        T: DeserializeOwned,
    {
        let Self::Browser(self_) = self;
        self_.query()
    }

    fn set_query<T>(&self, query: T) -> Result<(), serde_urlencoded::ser::Error>
    where
        T: Serialize,
    {
        let Self::Browser(self_) = self;
        self_.set_query(query)
    }

    fn hash(&self) -> String {
        let Self::Browser(self_) = self;
        self_.hash()
    }

    fn set_hash(&self, hash: &str) {
        let Self::Browser(self_) = self;
        self_.set_hash(hash)
    }

    fn route(&self) -> Option<R> {
        let Self::Browser(self_) = self;
        self_.route()
    }
}
